#[inline]
pub(super) fn utf16_length(value: &str) -> usize {
    const HIGH_BITS: u64 = 0x8080_8080_8080_8080;
    const BYTE_ONES: u64 = 0x0101_0101_0101_0101;
    let (chunks, remainder) = value.as_bytes().as_chunks::<8>();
    let mut length = 0;

    for bytes in chunks {
        let word = u64::from_ne_bytes(*bytes);

        if word & HIGH_BITS == 0 {
            length += bytes.len();
            continue;
        }

        let continuation = word & !(word << 1) & HIGH_BITS;
        let four_byte_leader = word & (word << 1) & (word << 2) & (word << 3) & HIGH_BITS;
        let units = BYTE_ONES - (continuation >> 7) + (four_byte_leader >> 7);

        length += (units.wrapping_mul(BYTE_ONES) >> 56) as usize;
    }

    for &byte in remainder {
        length += usize::from(byte & 0xC0 != 0x80) + usize::from(byte >= 0xF0);
    }

    length
}
