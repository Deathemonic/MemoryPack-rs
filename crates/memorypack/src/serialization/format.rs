pub const NULL_OBJECT_HEADER: u8 = u8::MAX;
pub const OBJECT_REFERENCE_HEADER: u8 = 250;
pub const NULL_COLLECTION_LENGTH: i32 = -1;
pub const COLLECTION_HEADER_SIZE: usize = size_of::<i32>();
pub const UTF8_STRING_HEADER_SIZE: usize = COLLECTION_HEADER_SIZE * 2;
