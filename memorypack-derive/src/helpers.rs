use syn::Field;

#[inline]
pub fn is_single_field_i32(data_struct: &syn::DataStruct) -> bool {
    use syn::Fields;
    matches!(&data_struct.fields,
        Fields::Unnamed(fields) if fields.unnamed.len() == 1
            && matches!(&fields.unnamed[0].ty,
                syn::Type::Path(type_path) if type_path.path.is_ident("i32")
            )
    )
}

#[inline]
pub fn has_explicit_discriminants(data_enum: &syn::DataEnum) -> bool {
    data_enum.variants.iter().all(|v| v.discriminant.is_some())
}

#[inline]
pub fn should_skip_field(field: &Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("memorypack")
            && attr
                .meta
                .require_list()
                .map(|m| {
                    let tokens = m.tokens.to_string();
                    tokens.contains("skip") || tokens.contains("ignore")
                })
                .unwrap_or(false)
    }) || field
        .ident
        .as_ref()
        .map(|ident| ident.to_string().starts_with('_'))
        .unwrap_or(false)
}

pub fn get_field_order(field: &Field) -> Option<usize> {
    field.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("memorypack") {
            return None;
        }

        let list = attr.meta.require_list().ok()?;
        let tokens = list.tokens.to_string();
        let order_pos = tokens.find("order")?;
        let after_order = &tokens[order_pos..];
        let eq_pos = after_order.find('=')?;
        let after_eq = after_order[eq_pos + 1..].trim();

        let num_str = after_eq
            .find(|c: char| !c.is_ascii_digit())
            .map(|end| &after_eq[..end])
            .unwrap_or(after_eq);

        num_str.parse::<usize>().ok()
    })
}

#[inline]
pub fn is_zero_copy_field(field: &Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("memorypack")
            && attr
                .meta
                .require_list()
                .map(|m| m.tokens.to_string().contains("zero_copy"))
                .unwrap_or(false)
    })
}

#[inline]
pub fn is_borrowed_str(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(type_ref) = ty {
        if let syn::Type::Path(inner_path) = &*type_ref.elem {
            return inner_path.path.is_ident("str");
        }
    }
    false
}

#[inline]
pub fn is_borrowed_slice(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(type_ref) = ty {
        return matches!(&*type_ref.elem, syn::Type::Slice(_));
    }
    false
}

pub fn is_option_box(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    if segment.ident != "Option" {
        return false;
    }

    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return false;
    };
    let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() else {
        return false;
    };
    let syn::Type::Path(inner_path) = inner_ty else {
        return false;
    };
    let Some(inner_segment) = inner_path.path.segments.last() else {
        return false;
    };

    inner_segment.ident == "Box"
}

pub struct OrderedField<'a> {
    pub order: usize,
    pub field: &'a Field,
    pub ident: &'a Option<syn::Ident>,
}

pub fn prepare_ordered_fields<'a>(fields: &'a [&'a Field]) -> Vec<OrderedField<'a>> {
    let mut ordered: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(idx, f)| OrderedField {
            order: get_field_order(f).unwrap_or(idx),
            field: f,
            ident: &f.ident,
        })
        .collect();
    ordered.sort_by_key(|f| f.order);
    ordered
}

pub fn generate_field_deserialize(
    field: &Field,
    is_zero_copy_struct: bool,
) -> proc_macro2::TokenStream {
    use quote::quote;

    let name = &field.ident;
    let ty = &field.ty;

    if should_skip_field(field) {
        return quote! {
            let mut temp_reader = memorypack::MemoryPackReader::new(&[]);
            let _: #ty = memorypack::MemoryPackDeserialize::deserialize(&mut temp_reader)?;
            let #name = Default::default();
        };
    }

    let is_field_zero_copy = is_zero_copy_field(field);

    if is_zero_copy_struct || is_field_zero_copy {
        if is_borrowed_str(ty) {
            return quote! { let #name = reader.read_str()?; };
        }
        if is_borrowed_slice(ty) {
            return quote! { let #name = reader.read_bytes()?; };
        }
        if is_field_zero_copy {
            return quote! { let #name = memorypack::MemoryPackDeserializeZeroCopy::deserialize(reader)?; };
        }
    }

    quote! { let #name = memorypack::MemoryPackDeserialize::deserialize(reader)?; }
}
