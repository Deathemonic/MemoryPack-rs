use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields};

/// Derive macro for MemoryPackable trait.
///
/// # Structs
/// Fields starting with `_` or marked with `#[memorypack(skip)]` are skipped.
///
/// # Transparent Wrappers (Flags)
/// Use `#[memorypack(flags)]` to generate bitwise operators:
///
/// ```ignore
/// #[derive(MemoryPackable)]
/// #[memorypack(flags)]
/// #[repr(transparent)]
/// struct Permissions(i32);
/// ```
///
/// # Enums (Simple)
/// C-like enums are serialized as i32:
///
/// ```ignore
/// #[derive(MemoryPackable)]
/// #[repr(i32)]
/// enum Color { Red, Green, Blue }
/// ```
///
/// # Unions (Polymorphism)
/// Rust enums with data variants become C# Union types:
///
/// ```ignore
/// #[derive(MemoryPackable)]
/// #[memorypack(union)]
/// enum Shape {
///     Circle(CircleData),      // tag = 0
///     Rectangle(RectangleData), // tag = 1
/// }
/// ```
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
                    "C-like enums for MemoryPack must have either #[repr(i32)] or explicit discriminants (e.g., Red = 0, Green = 1)"
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
                "MemoryPackable cannot be derived for Rust unions (use enums for MemoryPack unions)"
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
            fn serialize(&self, writer: &mut memorypack::MemoryPackWriter) -> memorypack::Result<()> {
                #serialize_impl
                Ok(())
            }
        }

        impl #impl_generics memorypack::MemoryPackDeserialize for #name #ty_generics #where_clause {
            fn deserialize(reader: &mut memorypack::MemoryPackReader) -> memorypack::Result<Self> {
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
    has_repr_i32: bool,
}

impl AttributeFlags {
    fn parse(attrs: &[syn::Attribute]) -> Self {
        let mut result = Self {
            is_transparent: false,
            is_flags: false,
            is_union: false,
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
    let has_skip_attr = field.attrs.iter().any(|attr| {
        attr.path().is_ident("memorypack")
            && attr.meta.require_list()
                .map(|m| m.tokens.to_string().contains("skip"))
                .unwrap_or(false)
    });

    // Check if field name starts with underscore
    let starts_with_underscore = field.ident.as_ref()
        .map(|ident| ident.to_string().starts_with('_'))
        .unwrap_or(false);

    has_skip_attr || starts_with_underscore
}

fn generate_serialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable serialize can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip_fields: Vec<_> = fields.named.iter()
                .filter(|f| !should_skip_field(f))
                .collect();
            let field_count = non_skip_fields.len() as u8;

            let serialize_fields = non_skip_fields.iter().map(|f| {
                let name = &f.ident;
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
            let deserialize_fields = fields.named.iter().map(|f| {
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
            });

            let field_names = fields.named.iter().map(|f| &f.ident);

            quote! {
                let _header = reader.read_u8()?;
                #(#deserialize_fields)*
                Ok(Self { #(#field_names),* })
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
        // SAFETY: Transmuting i32 to repr(i32) enum.
        // This requires the enum to have #[repr(i32)] attribute.
        // For invalid discriminant values, this may produce undefined behavior,
        // but matches C# MemoryPack's behavior for enum serialization.
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
