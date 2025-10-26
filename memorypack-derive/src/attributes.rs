pub struct AttributeFlags {
    pub is_transparent: bool,
    pub is_flags: bool,
    pub is_union: bool,
    pub is_version_tolerant: bool,
    pub is_circular: bool,
    pub is_zero_copy: bool,
    pub has_repr_i32: bool,
}

impl AttributeFlags {
    pub fn parse(attrs: &[syn::Attribute]) -> Self {
        let mut result = Self {
            is_transparent: false,
            is_flags: false,
            is_union: false,
            is_version_tolerant: false,
            is_circular: false,
            is_zero_copy: false,
            has_repr_i32: false,
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
                        result.is_zero_copy = tokens.contains("zero_copy");
                    }
                }
                _ => {}
            }
        }

        result
    }
}
