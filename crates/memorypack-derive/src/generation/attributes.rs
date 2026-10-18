use proc_macro2::TokenStream;
use syn::token::Paren;
use syn::{Attribute, Expr, LitInt, Token, Type, parenthesized};

pub struct AttributeFlags {
    pub is_transparent: bool,
    pub is_flags: bool,
    pub is_union: bool,
    pub is_version_tolerant: bool,
    pub is_circular: bool,
    pub is_arena: bool,
    pub has_repr_i32: bool
}

#[derive(Default)]
pub struct FieldAttributes {
    pub formatter: Option<Type>,
    pub order: Option<usize>,
    pub skipped: bool
}

impl AttributeFlags {
    pub fn parse(attrs: &[syn::Attribute]) -> Self {
        let mut result = Self {
            is_transparent: false,
            is_flags: false,
            is_union: false,
            is_version_tolerant: false,
            is_circular: false,
            is_arena: false,
            has_repr_i32: false
        };

        for attr in attrs {
            match attr.path() {
                path if path.is_ident("repr") => {
                    if let Ok(list) = attr.meta.require_list() {
                        let tokens = list.tokens.to_string();

                        result.is_transparent = tokens.contains("transparent");
                        result.has_repr_i32 = tokens.contains("i32");
                    }
                }
                path if path.is_ident("memorypack") => {
                    if let Ok(list) = attr.meta.require_list() {
                        let tokens = list.tokens.to_string();

                        result.is_flags = tokens.contains("flags");
                        result.is_union = tokens.contains("union");
                        result.is_version_tolerant = tokens.contains("version_tolerant");
                        result.is_circular = tokens.contains("circular");
                        result.is_arena = tokens.contains("arena");
                    }
                }
                _ => {}
            }
        }

        result
    }
}

impl FieldAttributes {
    pub fn parse(attributes: &[Attribute]) -> syn::Result<Self> {
        let mut result = Self::default();

        for attribute in attributes
            .iter()
            .filter(|attribute| attribute.path().is_ident("memorypack"))
        {
            attribute.parse_nested_meta(|meta| {
                if meta.path.is_ident("formatter") {
                    if result.formatter.is_some() {
                        return Err(meta.error("duplicate MemoryPack formatter"));
                    }

                    result.formatter = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("order") {
                    result.order = Some(meta.value()?.parse::<LitInt>()?.base10_parse()?);
                } else if meta.path.is_ident("skip") || meta.path.is_ident("ignore") {
                    result.skipped = true;
                } else if meta.input.peek(Token![=]) {
                    let _: Expr = meta.value()?.parse()?;
                } else if meta.input.peek(Paren) {
                    let content;
                    parenthesized!(content in meta.input);
                    let _: TokenStream = content.parse()?;
                }

                Ok(())
            })?;
        }

        Ok(result)
    }

    pub fn for_field(field: &syn::Field) -> Self {
        Self::parse(&field.attrs).expect("MemoryPack field attributes have been validated")
    }
}
