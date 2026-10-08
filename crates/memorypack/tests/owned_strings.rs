use memorypack::{
    MemoryPackError,
    MemoryPackReader,
    MemoryPackSerializer,
    MemoryPackSerializerOptions
};

fn utf8_wire(bytes: &[u8]) -> Vec<u8> {
    let mut wire = (!(bytes.len() as i32)).to_le_bytes().to_vec();
    wire.extend_from_slice(&1_i32.to_le_bytes());
    wire.extend_from_slice(bytes);
    wire
}
fn utf16_wire(units: &[u16]) -> Vec<u8> {
    let mut wire = (units.len() as i32).to_le_bytes().to_vec();
    for unit in units {
        wire.extend_from_slice(&unit.to_le_bytes());
    }
    wire
}
#[test]
fn owned_strings_preserve_all_characters_and_encoding_boundaries() {
    let values = [
        "".to_owned(),
        "a".to_owned(),
        "Test Data".to_owned(),
        "\0embedded\0".to_owned(),
        "éλ你好👋".to_owned(),
        "a".repeat(63),
        "a".repeat(64),
        "a".repeat(65),
        "a".repeat(4096),
        "你好👋λ".repeat(256)
    ];
    for value in values {
        for options in [MemoryPackSerializerOptions::UTF8, MemoryPackSerializerOptions::UTF16] {
            let bytes = MemoryPackSerializer::serialize_with_options(&value, &options).unwrap();
            let mut reader = MemoryPackReader::new(&bytes);
            assert_eq!(reader.read_string().unwrap(), value);
            assert_eq!(reader.position() as usize, bytes.len());
            for end in 0..bytes.len() {
                assert!(
                    matches!(
                        MemoryPackReader::new(&bytes[..end]).read_string(),
                        Err(MemoryPackError::UnexpectedEndOfBuffer)
                    ),
                    "truncated input end={end}, len={}",
                    bytes.len()
                );
            }
        }
    }
}
#[test]
fn rejects_invalid_utf8_and_surrogates() {
    for invalid in [
        &[0x80][..],
        &[0xC0, 0xAF],
        &[0xED, 0xA0, 0x80],
        &[0xF4, 0x90, 0x80, 0x80],
        &[b'a', 0xFF],
        &[0xE2, 0x82]
    ] {
        let mut bytes = utf8_wire(invalid);
        assert!(matches!(
            MemoryPackReader::new(&bytes).read_string(),
            Err(MemoryPackError::InvalidUtf8)
        ));
        bytes[4..8].copy_from_slice(&(invalid.len() as i32).to_le_bytes());
        assert!(matches!(
            MemoryPackReader::new(&bytes).read_string(),
            Err(MemoryPackError::InvalidUtf8)
        ));
    }
    for length in [63, 64, 65, 4096] {
        let mut invalid = vec![b'a'; length];
        invalid[length - 1] = 0xFF;
        let bytes = utf8_wire(&invalid);
        assert!(matches!(
            MemoryPackReader::new(&bytes).read_string(),
            Err(MemoryPackError::InvalidUtf8)
        ));
    }
    for units in [&[0xD800][..], &[0xDC00], &[0xD800, 0x0061], &[0x0061, 0xDC00]] {
        let bytes = utf16_wire(units);
        assert!(matches!(
            MemoryPackReader::new(&bytes).read_string(),
            Err(MemoryPackError::InvalidUtf8)
        ));
    }
}
#[test]
fn null_empty_and_valid_surrogate_pairs() {
    for marker in [-1_i32, 0] {
        let bytes = marker.to_le_bytes();
        let mut reader = MemoryPackReader::new(&bytes);
        assert_eq!(reader.read_string().unwrap(), "");
        assert_eq!(reader.position(), 4);
    }
    let bytes = utf16_wire(&[0x0061, 0xD83D, 0xDC4B, 0x0062]);
    assert_eq!(MemoryPackReader::new(&bytes).read_string().unwrap(), "a👋b");
}
#[test]
fn owned_result_survives_input_mutation_and_drop() {
    let original = "你好👋owned".to_owned();
    let mut bytes = MemoryPackSerializer::serialize(&original).unwrap();
    let value = MemoryPackSerializer::deserialize::<String>(&bytes).unwrap();
    bytes.fill(0);
    drop(bytes);
    assert_eq!(value, original);
}
