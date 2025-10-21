use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields};

#[proc_macro_derive(MemoryPackable, attributes(memorypack, tag))]
pub fn derive_memorypack(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = AttributeFlags::parse(&input.attrs);

    let (serialize_impl, deserialize_impl) = match &input.data {
        Data::Struct(data_struct) if attrs.is_transparent && is_single_field_i32(data_struct) => (
            generate_transparent_serialize(),
            generate_transparent_deserialize(),
        ),
        Data::Struct(_) if attrs.is_version_tolerant => (
            generate_version_tolerant_serialize(&input.data),
            generate_version_tolerant_deserialize(&input.data),
        ),
        Data::Struct(_) => (
            generate_serialize(&input.data),
            generate_deserialize(&input.data),
        ),
        Data::Enum(data_enum) if attrs.is_union => (
            generate_union_serialize(data_enum),
            generate_union_deserialize(name, data_enum),
        ),
        Data::Enum(data_enum) => {
            let has_explicit = has_explicit_discriminants(data_enum);
            
            if !attrs.has_repr_i32 && !has_explicit {
                return syn::Error::new_spanned(
                    &input,
                    "C-like enums for MemoryPack must have either #[repr(i32)] or explicit discriminants"
                ).to_compile_error().into();
            }
            
            let deserialize = if has_explicit {
                generate_enum_deserialize_safe(data_enum)
            } else {
                generate_enum_deserialize_unsafe()
            };
            
            (generate_enum_serialize(), deserialize)
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(
                &input,
                "MemoryPackable cannot be derived for Rust unions"
            ).to_compile_error().into();
        }
    };

    let flags_impl = if attrs.is_flags && attrs.is_transparent {
        generate_flags_impls(name)
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl #impl_generics memorypack::MemoryPackSerialize for #name #ty_generics #where_clause {
            fn serialize(&self, writer: &mut memorypack::MemoryPackWriter) -> Result<(), memorypack::MemoryPackError> {
                #serialize_impl
                Ok(())
            }
        }

        impl #impl_generics memorypack::MemoryPackDeserialize for #name #ty_generics #where_clause {
            fn deserialize(reader: &mut memorypack::MemoryPackReader) -> Result<Self, memorypack::MemoryPackError> {
                #deserialize_impl
            }
        }

        #flags_impl
    };

    expanded.into()
}

struct AttributeFlags {
    is_transparent: bool,
    is_flags: bool,
    is_union: bool,
    is_version_tolerant: bool,
    has_repr_i32: bool,
}

impl AttributeFlags {
    fn parse(attrs: &[syn::Attribute]) -> Self {
        let mut result = Self {
            is_transparent: false,
            is_flags: false,
            is_union: false,
            is_version_tolerant: false,
            has_repr_i32: false,
        };

        for attr in attrs {
            if attr.path().is_ident("repr") {
                if let Ok(list) = attr.meta.require_list() {
                    let tokens = list.tokens.to_string();
                    if tokens.contains("transparent") {
                        result.is_transparent = true;
                    }
                    if tokens.contains("i32") {
                        result.has_repr_i32 = true;
                    }
                }
            } else if attr.path().is_ident("memorypack") {
                if let Ok(list) = attr.meta.require_list() {
                    let tokens = list.tokens.to_string();
                    if tokens.contains("flags") {
                        result.is_flags = true;
                    }
                    if tokens.contains("union") {
                        result.is_union = true;
                    }
                    if tokens.contains("version_tolerant") {
                        result.is_version_tolerant = true;
                    }
                }
            }
        }

        result
    }
}

#[inline]
fn is_single_field_i32(data_struct: &syn::DataStruct) -> bool {
    matches!(&data_struct.fields,
        Fields::Unnamed(fields) if fields.unnamed.len() == 1
            && matches!(&fields.unnamed[0].ty,
                syn::Type::Path(type_path) if type_path.path.is_ident("i32")
            )
    )
}

#[inline]
fn has_explicit_discriminants(data_enum: &syn::DataEnum) -> bool {
    data_enum.variants.iter().all(|v| v.discriminant.is_some())
}

#[inline]
fn should_skip_field(field: &Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("memorypack")
            && attr.meta.require_list()
                .map(|m| m.tokens.to_string().contains("skip"))
                .unwrap_or(false)
    }) || field.ident.as_ref()
        .map(|ident| ident.to_string().starts_with('_'))
        .unwrap_or(false)
}

fn get_field_order(field: &Field) -> Option<usize> {
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

struct OrderedField<'a> {
    order: usize,
    field: &'a Field,
    ident: &'a Option<syn::Ident>,
}

fn prepare_ordered_fields<'a>(fields: &'a [&'a Field]) -> Vec<OrderedField<'a>> {
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

fn generate_serialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable serialize can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            let ordered = prepare_ordered_fields(&non_skip);
            let field_count = ordered.len() as u8;

            let serialize_fields = ordered.iter().map(|of| {
                let name = of.ident;
                quote! { memorypack::MemoryPackSerialize::serialize(&self.#name, writer)?; }
            });

            quote! {
                writer.write_u8(#field_count)?;
                #(#serialize_fields)*
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len() as u8;
            let serialize_fields = (0..fields.unnamed.len()).map(|i| {
                let index = syn::Index::from(i);
                quote! { memorypack::MemoryPackSerialize::serialize(&self.#index, writer)?; }
            });

            quote! {
                writer.write_u8(#field_count)?;
                #(#serialize_fields)*
            }
        }
        Fields::Unit => quote! {},
    }
}

fn generate_deserialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable deserialize can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            let ordered = prepare_ordered_fields(&non_skip);
            
            let all_field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            
            let deserialize_stmts: Vec<_> = fields.named.iter().map(|f| {
                let name = &f.ident;
                if should_skip_field(f) {
                    let ty = &f.ty;
                    quote! {
                        let _: #ty = memorypack::MemoryPackDeserialize::deserialize(reader)?;
                        let #name = Default::default();
                    }
                } else {
                    quote! { let #name = memorypack::MemoryPackDeserialize::deserialize(reader)?; }
                }
            }).collect();

            let mut ordered_deserialize = Vec::new();
            let mut skip_field_idx = 0;
            let mut ordered_idx = 0;
            
            for f in &fields.named {
                if should_skip_field(f) {
                    ordered_deserialize.push(deserialize_stmts[skip_field_idx + ordered_idx].clone());
                    skip_field_idx += 1;
                } else if ordered_idx < ordered.len() {
                    let field_idx = fields.named.iter()
                        .position(|field| std::ptr::eq(field, ordered[ordered_idx].field))
                        .unwrap();
                    ordered_deserialize.push(deserialize_stmts[field_idx].clone());
                    ordered_idx += 1;
                }
            }

            quote! {
                let _header = reader.read_u8()?;
                #(#ordered_deserialize)*
                Ok(Self { #(#all_field_names),* })
            }
        }
        Fields::Unnamed(fields) => {
            let len = fields.unnamed.len();
            let field_vars: Vec<_> = (0..len)
                .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                .collect();

            let deserialize_stmts = field_vars.iter().map(|var| {
                quote! { let #var = memorypack::MemoryPackDeserialize::deserialize(reader)?; }
            });

            quote! {
                let _header = reader.read_u8()?;
                #(#deserialize_stmts)*
                Ok(Self(#(#field_vars),*))
            }
        }
        Fields::Unit => quote! { Ok(Self) },
    }
}

#[inline]
fn generate_enum_serialize() -> proc_macro2::TokenStream {
    quote! {
        writer.write_i32(*self as i32)?;
    }
}

fn generate_enum_deserialize_unsafe() -> proc_macro2::TokenStream {
    quote! {
        let value = reader.read_i32()?;
        Ok(unsafe { std::mem::transmute(value) })
    }
}

fn generate_enum_deserialize_safe(data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminant = &variant.discriminant.as_ref().unwrap().1;
        
        quote! {
            #discriminant => Ok(Self::#variant_name),
        }
    });

    quote! {
        let value = reader.read_i32()?;
        match value {
            #(#variants)*
            _ => Err(memorypack::MemoryPackError::DeserializationError(
                format!("Invalid discriminant {} for enum {}", value, stringify!(Self))
            ))
        }
    }
}

#[inline]
fn generate_transparent_serialize() -> proc_macro2::TokenStream {
    quote! {
        writer.write_i32(self.0)?;
    }
}

#[inline]
fn generate_transparent_deserialize() -> proc_macro2::TokenStream {
    quote! {
        Ok(Self(reader.read_i32()?))
    }
}

fn generate_flags_impls(name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl #name {
            #[inline]
            pub const fn contains(self, other: #name) -> bool {
                (self.0 & other.0) == other.0
            }

            #[inline]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
        }

        impl std::ops::BitOr for #name {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitAnd for #name {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitXor for #name {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::Not for #name {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }
    }
}

fn generate_union_serialize(data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().enumerate().map(|(tag, variant)| {
        let variant_name = &variant.ident;
        let tag_value = tag as u8;

        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                quote! {
                    Self::#variant_name(inner) => {
                        writer.write_u8(#tag_value)?;
                        memorypack::MemoryPackSerialize::serialize(inner, writer)?;
                    }
                }
            }
            _ => {
                return quote! {
                    compile_error!("Union variants must have exactly one unnamed field");
                };
            }
        }
    });

    quote! {
        match self {
            #(#variants)*
        }
    }
}

fn generate_union_deserialize(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().enumerate().map(|(tag, variant)| {
        let variant_name = &variant.ident;
        let tag_value = tag as u8;

        quote! {
            #tag_value => {
                let inner = memorypack::MemoryPackDeserialize::deserialize(reader)?;
                Ok(Self::#variant_name(inner))
            }
        }
    });

    quote! {
        let tag = reader.read_u8()?;
        match tag {
            #(#variants)*
            _ => Err(memorypack::MemoryPackError::DeserializationError(
                format!("Unknown union tag {} for {}", tag, stringify!(#name))
            ))
        }
    }
}

fn generate_version_tolerant_serialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable version_tolerant can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();

            if non_skip.is_empty() {
                return quote! { writer.write_u8(0)?; };
            }

            let ordered = prepare_ordered_fields(&non_skip);
            let max_order = ordered.last().map(|f| f.order).unwrap_or(0);
            let member_count = max_order + 1;

            let serialize_logic = (0..member_count).map(|order| {
                if let Some(of) = ordered.iter().find(|f| f.order == order) {
                    let name = of.ident;
                    quote! {
                        let mut temp_writer = memorypack::MemoryPackWriter::new();
                        memorypack::MemoryPackSerialize::serialize(&self.#name, &mut temp_writer)?;
                        let field_bytes = temp_writer.into_bytes();
                        field_buffers.push((field_bytes.len() as i64, field_bytes));
                    }
                } else {
                    quote! { field_buffers.push((0i64, Vec::new())); }
                }
            });

            quote! {
                writer.write_u8(#member_count as u8)?;
                let mut field_buffers = Vec::with_capacity(#member_count);
                #(#serialize_logic)*
                for (length, _) in &field_buffers {
                    memorypack::varint::write_varint(writer, *length)?;
                }
                for (_, buf) in field_buffers {
                    writer.buffer.extend_from_slice(&buf);
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();
            let field_indices: Vec<_> = (0..field_count).map(syn::Index::from).collect();

            let serialize_to_buffers = field_indices.iter().map(|idx| {
                quote! {
                    let mut temp_writer = memorypack::MemoryPackWriter::new();
                    memorypack::MemoryPackSerialize::serialize(&self.#idx, &mut temp_writer)?;
                    let field_bytes = temp_writer.into_bytes();
                    field_buffers.push((field_bytes.len() as i64, field_bytes));
                }
            });

            quote! {
                writer.write_u8(#field_count as u8)?;
                let mut field_buffers = Vec::with_capacity(#field_count);
                #(#serialize_to_buffers)*
                for (length, _) in &field_buffers {
                    memorypack::varint::write_varint(writer, *length)?;
                }
                for (_, buf) in field_buffers {
                    writer.buffer.extend_from_slice(&buf);
                }
            }
        }
        Fields::Unit => quote! { writer.write_u8(0)?; },
    }
}

fn generate_version_tolerant_deserialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable version_tolerant can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            
            if non_skip.is_empty() {
                return quote! {
                    let _member_count = reader.read_u8()?;
                    Ok(Self {})
                };
            }

            let ordered = prepare_ordered_fields(&non_skip);
            let all_field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            let deserialize_logic: Vec<_> = ordered.iter().map(|of| {
                let name = of.ident;
                let order = of.order;
                quote! {
                    let #name = if #order < member_count && lengths[#order] > 0 {
                        memorypack::MemoryPackDeserialize::deserialize(reader)?
                    } else {
                        if #order < member_count {
                            reader.skip(lengths[#order])?;
                        }
                        Default::default()
                    };
                }
            }).collect();

            let skip_extra_fields = if let Some(max_order) = ordered.last().map(|f| f.order) {
                let next_order = max_order + 1;
                quote! {
                    for i in #next_order..member_count {
                        reader.skip(lengths[i])?;
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                let member_count = reader.read_u8()? as usize;
                let mut lengths = Vec::with_capacity(member_count);
                for _ in 0..member_count {
                    lengths.push(memorypack::varint::read_varint(reader)? as usize);
                }
                #(#deserialize_logic)*
                #skip_extra_fields
                Ok(Self { #(#all_field_names),* })
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();
            let field_vars: Vec<_> = (0..field_count)
                .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                .collect();

            let deserialize_fields = field_vars.iter().enumerate().map(|(i, var)| {
                quote! {
                    let #var = if #i < member_count {
                        memorypack::MemoryPackDeserialize::deserialize(reader)?
                    } else {
                        Default::default()
                    };
                }
            });

            quote! {
                let member_count = reader.read_u8()? as usize;
                let mut lengths = Vec::with_capacity(member_count);
                for _ in 0..member_count {
                    lengths.push(memorypack::varint::read_varint(reader)? as usize);
                }
                #(#deserialize_fields)*
                for i in #field_count..member_count {
                    reader.skip(lengths[i])?;
                }
                Ok(Self(#(#field_vars),*))
            }
        }
        Fields::Unit => quote! {
            let _member_count = reader.read_u8()?;
            Ok(Self)
        },
    }
}
