pub struct AttributeFlags {
    pub is_transparent: bool,
    pub is_flags: bool,
    pub is_union: bool,
    pub is_version_tolerant: bool,
    pub is_circular: bool,
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
                    if tokens.contains("circular") {
                        result.is_circular = true;
                    }
                }
            }
        }

        result
    }
}

