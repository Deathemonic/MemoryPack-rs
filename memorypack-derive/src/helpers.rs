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
            && attr.meta.require_list()
                .map(|m| m.tokens.to_string().contains("skip"))
                .unwrap_or(false)
    }) || field.ident.as_ref()
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

pub fn is_option_box(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else { return false; };
    let Some(segment) = type_path.path.segments.last() else { return false; };
    
    if segment.ident != "Option" {
        return false;
    }
    
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return false; };
    let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() else { return false; };
    let syn::Type::Path(inner_path) = inner_ty else { return false; };
    let Some(inner_segment) = inner_path.path.segments.last() else { return false; };
    
    inner_segment.ident == "Box"
}

pub struct OrderedField<'a> {
    pub order: usize,
    pub field: &'a Field,
    pub ident: &'a Option<syn::Ident>,
}

pub fn prepare_ordered_fields<'a>(fields: &'a [&'a Field]) -> Vec<OrderedField<'a>> {
    let mut ordered: Vec<_> = fields.iter()
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

