use simdutf8::basic;

use crate::error::MemoryPackError;
use crate::state::MemoryPackReaderOptionalState;

pub struct MemoryPackReader<'a> {
    data: &'a [u8],
    pos: usize,
    pub optional_state: Option<MemoryPackReaderOptionalState>
}

impl<'a> MemoryPackReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            optional_state: None
        }
    }

    pub fn new_with_state(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            optional_state: Some(MemoryPackReaderOptionalState::new())
        }
    }

    pub fn read_string(&mut self) -> Result<String, MemoryPackError> {
        let length_or_marker = self.read_i32()?;

        if length_or_marker == -1 {
            return Ok(String::new());
        }

        if length_or_marker < 0 {
            return self.read_utf8_string(!length_or_marker as usize);
        }

        let char_count = length_or_marker as usize;
        if char_count == 0 {
            return Ok(String::new());
        }

        self.read_utf16_string(char_count)
    }

    fn read_utf8_string(&mut self, byte_count: usize) -> Result<String, MemoryPackError> {
        let _char_length = self.read_i32()?;
        let slice = self.read_bytes(byte_count)?;

        Ok(basic::from_utf8(slice).map_err(|_| MemoryPackError::InvalidUtf8)?.to_string())
    }

    #[inline]
    pub fn read_str(&mut self) -> Result<&'a str, MemoryPackError> {
        let length_or_marker = self.read_i32()?;

        if length_or_marker == -1 || length_or_marker == 0 {
            return Ok("");
        }

        if length_or_marker < 0 {
            return self.read_utf8_str(!length_or_marker as usize);
        }

        Err(MemoryPackError::Utf16NotSupportedForZeroCopy)
    }

    #[inline]
    fn read_utf8_str(&mut self, byte_count: usize) -> Result<&'a str, MemoryPackError> {
        let _char_length = self.read_i32()?;
        let slice = self.read_bytes(byte_count)?;

        let str_slice = basic::from_utf8(slice).map_err(|_| MemoryPackError::InvalidUtf8)?;

        Ok(str_slice)
    }

    #[inline]
    pub fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], MemoryPackError> {
        let pos = self.pos;

        let end = pos.checked_add(length).ok_or(MemoryPackError::UnexpectedEndOfBuffer)?;
        if end > self.data.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        let slice = &self.data[pos..end];
        self.pos = end;
        Ok(slice)
    }

    #[inline]
    pub fn read_bytes_vec(&mut self, length: usize) -> Result<Vec<u8>, MemoryPackError> {
        Ok(self.read_bytes(length)?.to_vec())
    }

    #[inline]
    pub fn read_fixed_bytes<const N: usize>(&mut self) -> Result<[u8; N], MemoryPackError> {
        let mut buffer = [0u8; N];
        buffer.copy_from_slice(self.read_bytes(N)?);
        Ok(buffer)
    }

    #[inline(always)]
    fn read_unaligned<T: Copy>(&mut self) -> Result<T, MemoryPackError> {
        let size = std::mem::size_of::<T>();
        let end = self.pos.checked_add(size).ok_or(MemoryPackError::UnexpectedEndOfBuffer)?;

        if end > self.data.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        let value = unsafe { std::ptr::read_unaligned(self.data.as_ptr().add(self.pos) as *const T) };
        self.pos = end;
        Ok(value)
    }

    #[inline]
    fn read_utf16_string(&mut self, char_count: usize) -> Result<String, MemoryPackError> {
        let byte_count = char_count * 2;
        let slice = self.read_bytes(byte_count)?;

        let mut result = String::with_capacity(char_count * 3);
        let mut i = 0;
        while i < byte_count {
            let code_unit = u16::from_le_bytes([slice[i], slice[i + 1]]);
            i += 2;

            if !(0xD800..=0xDFFF).contains(&code_unit) {
                if let Some(c) = char::from_u32(code_unit as u32) {
                    result.push(c);
                } else {
                    return Err(MemoryPackError::InvalidUtf8);
                }
            } else if (0xD800..=0xDBFF).contains(&code_unit) {
                if i + 2 > byte_count {
                    return Err(MemoryPackError::InvalidUtf8);
                }

                let low = u16::from_le_bytes([slice[i], slice[i + 1]]);

                i += 2;
                if !(0xDC00..=0xDFFF).contains(&low) {
                    return Err(MemoryPackError::InvalidUtf8);
                }

                let code_point =
                    0x10000 + ((code_unit as u32 - 0xD800) << 10) + (low as u32 - 0xDC00);

                if let Some(c) = char::from_u32(code_point) {
                    result.push(c);
                } else {
                    return Err(MemoryPackError::InvalidUtf8);
                }
            } else {
                return Err(MemoryPackError::InvalidUtf8);
            }
        }
        Ok(result)
    }

    #[inline(always)]
    pub fn read_bool(&mut self) -> Result<bool, MemoryPackError> { Ok(self.read_u8()? == 1) }

    #[inline(always)]
    pub fn read_i8(&mut self) -> Result<i8, MemoryPackError> { Ok(self.read_u8()? as i8) }

    #[inline(always)]
    pub fn read_u8(&mut self) -> Result<u8, MemoryPackError> {
        if self.pos >= self.data.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        let value = self.data[self.pos];
        self.pos += 1;
        Ok(value)
    }

    #[inline(always)]
    pub fn read_i16(&mut self) -> Result<i16, MemoryPackError> {
        Ok(i16::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_u16(&mut self) -> Result<u16, MemoryPackError> {
        Ok(u16::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_i32(&mut self) -> Result<i32, MemoryPackError> {
        Ok(i32::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_u32(&mut self) -> Result<u32, MemoryPackError> {
        Ok(u32::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_i64(&mut self) -> Result<i64, MemoryPackError> {
        Ok(i64::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_u64(&mut self) -> Result<u64, MemoryPackError> {
        Ok(u64::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_f32(&mut self) -> Result<f32, MemoryPackError> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    #[inline(always)]
    pub fn read_f64(&mut self) -> Result<f64, MemoryPackError> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    #[inline(always)]
    pub fn read_i128(&mut self) -> Result<i128, MemoryPackError> {
        Ok(i128::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_u128(&mut self) -> Result<u128, MemoryPackError> {
        Ok(u128::from_le(self.read_unaligned()?))
    }

    #[inline(always)]
    pub fn read_char(&mut self) -> Result<char, MemoryPackError> {
        let code_unit = self.read_u16()?;

        if !(0xD800..=0xDFFF).contains(&code_unit) {
            return char::from_u32(code_unit as u32).ok_or(MemoryPackError::InvalidCodePoint);
        }

        Err(MemoryPackError::DeserializationError(
            "Surrogate code unit cannot be converted to Rust char".into()
        ))
    }

    #[inline]
    pub fn skip(&mut self, n: usize) -> Result<(), MemoryPackError> {
        let new_pos = self.pos.checked_add(n).ok_or(MemoryPackError::UnexpectedEndOfBuffer)?;
        if new_pos > self.data.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        self.pos = new_pos;
        Ok(())
    }

    #[inline]
    pub fn rewind(&mut self, n: usize) -> Result<(), MemoryPackError> {
        if n > self.pos {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        self.pos -= n;
        Ok(())
    }

    #[inline]
    pub fn position(&self) -> u64 { self.pos as u64 }
}
