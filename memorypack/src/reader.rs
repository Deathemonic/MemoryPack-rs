use crate::error::MemoryPackError;
use crate::state::MemoryPackReaderOptionalState;

use simdutf8::basic;
use zerocopy::{FromBytes, LittleEndian, U16, U32, U64, I16, I32, I64, U128, I128, F32, F64};

pub struct MemoryPackReader<'a> {
    data: &'a [u8],
    pos: usize,
    pub optional_state: Option<MemoryPackReaderOptionalState>,
}

impl<'a> MemoryPackReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            optional_state: None,
        }
    }

    pub fn new_with_state(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            optional_state: Some(MemoryPackReaderOptionalState::new()),
        }
    }

    #[inline(always)]
    fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    #[inline(always)]
    fn read_slice(&mut self, n: usize) -> Result<&'a [u8], MemoryPackError> {
        if self.remaining() < n {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    #[inline(always)]
    fn read_type<T: FromBytes>(&mut self) -> Result<T, MemoryPackError> {
        let size = size_of::<T>();
        if self.remaining() < size {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }
        let (val, _) = T::read_from_prefix(&self.data[self.pos..])
            .map_err(|_| MemoryPackError::UnexpectedEndOfBuffer)?;
        self.pos += size;
        Ok(val)
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

    #[inline]
    fn read_utf8_string(&mut self, byte_count: usize) -> Result<String, MemoryPackError> {
        let _char_length = self.read_i32()?;
        let slice = self.read_slice(byte_count)?;

        basic::from_utf8(slice).map_err(|_| MemoryPackError::InvalidUtf8)?;

        Ok(unsafe { String::from_utf8_unchecked(slice.to_vec()) })
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
        let slice = self.read_slice(byte_count)?;
        basic::from_utf8(slice).map_err(|_| MemoryPackError::InvalidUtf8)
    }

    #[inline]
    pub fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], MemoryPackError> {
        self.read_slice(length)
    }

    #[inline]
    pub fn read_bytes_vec(&mut self, length: usize) -> Result<Vec<u8>, MemoryPackError> {
        Ok(self.read_slice(length)?.to_vec())
    }

    #[inline]
    pub fn read_fixed_bytes<const N: usize>(&mut self) -> Result<[u8; N], MemoryPackError> {
        if self.remaining() < N {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }
        let arr = std::array::from_fn(|i| self.data[self.pos + i]);
        self.pos += N;
        Ok(arr)
    }

    #[inline]
    fn read_utf16_string(&mut self, char_count: usize) -> Result<String, MemoryPackError> {
        let byte_count = char_count * 2;
        let slice = self.read_slice(byte_count)?;

        let mut result = String::with_capacity(char_count * 3);
        let mut i = 0;
        while i < byte_count {
            let code_unit = u16::from_le_bytes([slice[i], slice[i + 1]]);
            i += 2;

            if code_unit < 0xD800 || code_unit > 0xDFFF {
                if let Some(c) = char::from_u32(code_unit as u32) {
                    result.push(c);
                } else {
                    return Err(MemoryPackError::InvalidUtf8);
                }
            } else if code_unit >= 0xD800 && code_unit <= 0xDBFF {
                if i + 2 > byte_count {
                    return Err(MemoryPackError::InvalidUtf8);
                }

                let low = u16::from_le_bytes([slice[i], slice[i + 1]]);

                i += 2;
                if low < 0xDC00 || low > 0xDFFF {
                    return Err(MemoryPackError::InvalidUtf8);
                }

                let code_point = 0x10000 + ((code_unit as u32 - 0xD800) << 10) + (low as u32 - 0xDC00);
                
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
    pub fn read_bool(&mut self) -> Result<bool, MemoryPackError> {
        Ok(self.read_u8()? == 1)
    }

    #[inline(always)]
    pub fn read_i8(&mut self) -> Result<i8, MemoryPackError> {
        Ok(self.read_u8()? as i8)
    }

    #[inline(always)]
    pub fn read_u8(&mut self) -> Result<u8, MemoryPackError> {
        if self.remaining() < 1 {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }
        let val = self.data[self.pos];
        self.pos += 1;
        Ok(val)
    }

    #[inline(always)]
    pub fn read_i16(&mut self) -> Result<i16, MemoryPackError> {
        Ok(self.read_type::<I16<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_u16(&mut self) -> Result<u16, MemoryPackError> {
        Ok(self.read_type::<U16<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_i32(&mut self) -> Result<i32, MemoryPackError> {
        Ok(self.read_type::<I32<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_u32(&mut self) -> Result<u32, MemoryPackError> {
        Ok(self.read_type::<U32<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_i64(&mut self) -> Result<i64, MemoryPackError> {
        Ok(self.read_type::<I64<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_u64(&mut self) -> Result<u64, MemoryPackError> {
        Ok(self.read_type::<U64<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_f32(&mut self) -> Result<f32, MemoryPackError> {
        Ok(self.read_type::<F32<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_f64(&mut self) -> Result<f64, MemoryPackError> {
        Ok(self.read_type::<F64<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_i128(&mut self) -> Result<i128, MemoryPackError> {
        Ok(self.read_type::<I128<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_u128(&mut self) -> Result<u128, MemoryPackError> {
        Ok(self.read_type::<U128<LittleEndian>>()?.get())
    }

    #[inline(always)]
    pub fn read_char(&mut self) -> Result<char, MemoryPackError> {
        let code_unit = self.read_u16()?;
        
        if code_unit < 0xD800 || code_unit > 0xDFFF {
            return char::from_u32(code_unit as u32).ok_or_else(|| {
                MemoryPackError::DeserializationError("Invalid Unicode code point".into())
            });
        }
        
        Err(MemoryPackError::DeserializationError(
            "Surrogate code unit cannot be converted to Rust char".into(),
        ))
    }

    #[inline]
    pub fn skip(&mut self, n: usize) -> Result<(), MemoryPackError> {
        if self.remaining() < n {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }
        self.pos += n;
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
    pub fn position(&self) -> u64 {
        self.pos as u64
    }
}
