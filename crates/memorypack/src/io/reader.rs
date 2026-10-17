use std::{mem, ptr};

use simdutf8::basic;

use crate::error::MemoryPackError;
use crate::serialization::format::NULL_COLLECTION_LENGTH;
use crate::state::MemoryPackReaderOptionalState;
use crate::traits::MemoryPackUnmanaged;

pub struct MemoryPackReader<'a> {
    pub optional_state: Option<MemoryPackReaderOptionalState>,
    data: &'a [u8],
    position: usize
}

impl<'a> MemoryPackReader<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            optional_state: None
        }
    }

    pub fn new_with_state(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            optional_state: Some(MemoryPackReaderOptionalState::new())
        }
    }

    #[inline]
    pub(crate) fn read_unmanaged_vec<T: MemoryPackUnmanaged>(
        &mut self,
        count: usize
    ) -> Result<Vec<T>, MemoryPackError> {
        let byte_count = count
            .checked_mul(mem::size_of::<T>())
            .ok_or(MemoryPackError::UnexpectedEndOfBuffer)?;
        let bytes = self.read_bytes(byte_count)?;
        let mut values = Vec::<T>::with_capacity(count);
        // SAFETY: the unmanaged types have no padding and accept every bit
        // pattern. The checked byte count fits both the input slice and
        // the newly allocated vector.

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), values.as_mut_ptr().cast::<u8>(), byte_count);
            values.set_len(count);
        }

        Ok(values)
    }

    #[inline]
    pub fn read_string(&mut self) -> Result<String, MemoryPackError> {
        let length_or_marker = self.read_i32()?;

        if length_or_marker == NULL_COLLECTION_LENGTH {
            return Ok(String::new());
        }

        if length_or_marker < 0 {
            return self.read_utf8_string(!length_or_marker as usize);
        }

        let character_count = length_or_marker as usize;

        if character_count == 0 {
            return Ok(String::new());
        }

        self.read_utf16_string(character_count)
    }

    #[inline]
    pub fn read_str(&mut self) -> Result<&'a str, MemoryPackError> {
        let length_or_marker = self.read_i32()?;

        if length_or_marker == NULL_COLLECTION_LENGTH || length_or_marker == 0 {
            return Ok("");
        }

        if length_or_marker < 0 {
            return self.read_utf8_str(!length_or_marker as usize);
        }

        Err(MemoryPackError::Utf16NotSupportedForZeroCopy)
    }

    #[inline]
    pub fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], MemoryPackError> {
        let position = self.position;

        let end = position
            .checked_add(length)
            .ok_or(MemoryPackError::UnexpectedEndOfBuffer)?;

        if end > self.data.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        let slice = &self.data[position..end];

        self.position = end;

        Ok(slice)
    }

    #[inline]
    pub fn read_bytes_vec(&mut self, length: usize) -> Result<Vec<u8>, MemoryPackError> {
        Ok(self.read_bytes(length)?.to_vec())
    }

    #[inline]
    pub fn read_fixed_bytes<const N: usize>(&mut self) -> Result<[u8; N], MemoryPackError> {
        let mut buffer = [0_u8; N];

        buffer.copy_from_slice(self.read_bytes(N)?);
        Ok(buffer)
    }

    #[inline(always)]
    pub fn read_bool(&mut self) -> Result<bool, MemoryPackError> { Ok(self.read_u8()? == 1) }

    #[inline(always)]
    pub fn read_i8(&mut self) -> Result<i8, MemoryPackError> { Ok(self.read_u8()? as i8) }

    #[inline(always)]
    pub const fn read_u8(&mut self) -> Result<u8, MemoryPackError> {
        if self.position >= self.data.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        let value = self.data[self.position];

        self.position += 1;

        Ok(value)
    }

    #[inline(always)]
    pub fn read_i16(&mut self) -> Result<i16, MemoryPackError> {
        Ok(i16::from_le_bytes(self.read_fixed_bytes()?))
    }

    #[inline(always)]
    pub fn read_u16(&mut self) -> Result<u16, MemoryPackError> {
        Ok(u16::from_le_bytes(self.read_fixed_bytes()?))
    }

    #[inline(always)]
    pub fn read_i32(&mut self) -> Result<i32, MemoryPackError> {
        Ok(i32::from_le_bytes(self.read_fixed_bytes()?))
    }

    #[inline(always)]
    pub fn read_u32(&mut self) -> Result<u32, MemoryPackError> {
        Ok(u32::from_le_bytes(self.read_fixed_bytes()?))
    }

    #[inline(always)]
    pub fn read_i64(&mut self) -> Result<i64, MemoryPackError> {
        Ok(i64::from_le_bytes(self.read_fixed_bytes()?))
    }

    #[inline(always)]
    pub fn read_u64(&mut self) -> Result<u64, MemoryPackError> {
        Ok(u64::from_le_bytes(self.read_fixed_bytes()?))
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
        Ok(i128::from_le_bytes(self.read_fixed_bytes()?))
    }

    #[inline(always)]
    pub fn read_u128(&mut self) -> Result<u128, MemoryPackError> {
        Ok(u128::from_le_bytes(self.read_fixed_bytes()?))
    }

    #[inline(always)]
    pub fn read_char(&mut self) -> Result<char, MemoryPackError> {
        let code_unit = self.read_u16()?;

        if !(0xD800..=0xDFFF).contains(&code_unit) {
            return char::from_u32(code_unit as u32).ok_or(MemoryPackError::InvalidCodePoint);
        }

        Err(MemoryPackError::SurrogateCodeUnit)
    }

    #[inline]
    pub fn skip(&mut self, n: usize) -> Result<(), MemoryPackError> {
        let new_pos = self
            .position
            .checked_add(n)
            .ok_or(MemoryPackError::UnexpectedEndOfBuffer)?;

        if new_pos > self.data.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        self.position = new_pos;

        Ok(())
    }

    #[inline]
    pub const fn rewind(&mut self, n: usize) -> Result<(), MemoryPackError> {
        if n > self.position {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        self.position -= n;
        Ok(())
    }

    #[inline]
    pub const fn position(&self) -> u64 { self.position as u64 }

    #[inline]
    fn read_utf8_string(&mut self, byte_count: usize) -> Result<String, MemoryPackError> {
        let _char_length = self.read_i32()?;
        let slice = self.read_bytes(byte_count)?;

        basic::from_utf8(slice)
            .map(str::to_owned)
            .map_err(|_| MemoryPackError::InvalidUtf8)
    }

    #[inline]
    fn read_utf8_str(&mut self, byte_count: usize) -> Result<&'a str, MemoryPackError> {
        let _char_length = self.read_i32()?;
        let slice = self.read_bytes(byte_count)?;

        basic::from_utf8(slice).map_err(|_| MemoryPackError::InvalidUtf8)
    }

    #[cold]
    #[inline(never)]
    fn read_utf16_string(&mut self, character_count: usize) -> Result<String, MemoryPackError> {
        let byte_count = character_count
            .checked_mul(size_of::<u16>())
            .ok_or(MemoryPackError::UnexpectedEndOfBuffer)?;
        let bytes = self.read_bytes(byte_count)?;
        let units = bytes
            .as_chunks::<{ size_of::<u16>() }>()
            .0
            .iter()
            .copied()
            .map(u16::from_le_bytes);

        char::decode_utf16(units).try_fold(
            String::with_capacity(character_count),
            |mut value, character| {
                value.push(character.map_err(|_| MemoryPackError::InvalidUtf8)?);

                Ok(value)
            }
        )
    }
}
