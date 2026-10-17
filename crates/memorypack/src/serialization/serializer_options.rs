#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StringEncoding {
    #[default]
    Utf8,
    Utf16
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MemoryPackSerializerOptions {
    pub string_encoding: StringEncoding
}

impl MemoryPackSerializerOptions {
    pub const UTF16: Self = Self {
        string_encoding: StringEncoding::Utf16
    };
    pub const UTF8: Self = Self {
        string_encoding: StringEncoding::Utf8
    };

    pub const fn utf8() -> Self { Self::UTF8 }

    pub const fn utf16() -> Self { Self::UTF16 }
}
