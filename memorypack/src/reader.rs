use crate::error::MemoryPackError;
use crate::state::MemoryPackReaderOptionalState;

use byteorder::{LittleEndian, ReadBytesExt};
use simdutf8::basic;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub struct MemoryPackReader<'a> {
    pub(crate) cursor: Cursor<&'a [u8]>,
    pub optional_state: Option<MemoryPackReaderOptionalState>,
}

impl<'a> MemoryPackReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
            optional_state: None,
        }
    }

    pub fn new_with_state(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
            optional_state: Some(MemoryPackReaderOptionalState::new()),
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

    #[inline]
    fn read_utf8_string(&mut self, byte_count: usize) -> Result<String, MemoryPackError> {
        let _char_length = self.read_i32()?;
        let mut buffer = vec![0u8; byte_count];
        self.cursor.read_exact(&mut buffer)?;

        basic::from_utf8(&buffer).map_err(|_| MemoryPackError::InvalidUtf8)?;

        Ok(unsafe { String::from_utf8_unchecked(buffer) })
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
        let pos = self.cursor.position() as usize;
        let buffer = self.cursor.get_ref();

        if pos + byte_count > buffer.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        let str_slice = basic::from_utf8(&buffer[pos..pos + byte_count])
            .map_err(|_| MemoryPackError::InvalidUtf8)?;

        self.cursor.set_position((pos + byte_count) as u64);
        Ok(str_slice)
    }

    #[inline]
    pub fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], MemoryPackError> {
        let pos = self.cursor.position() as usize;
        let buffer = self.cursor.get_ref();

        if pos + length > buffer.len() {
            return Err(MemoryPackError::UnexpectedEndOfBuffer);
        }

        let slice = &buffer[pos..pos + length];
        self.cursor.set_position((pos + length) as u64);
        Ok(slice)
    }

    #[inline]
    pub fn read_bytes_vec(&mut self, length: usize) -> Result<Vec<u8>, MemoryPackError> {
        let mut buffer = vec![0u8; length];
        self.cursor.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    #[inline]
    pub fn read_fixed_bytes<const N: usize>(&mut self) -> Result<[u8; N], MemoryPackError> {
        let mut buffer = [0u8; N];
        self.cursor.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    #[inline]
    fn read_utf16_string(&mut self, char_count: usize) -> Result<String, MemoryPackError> {
        let byte_count = char_count * 2;
        let mut buffer = vec![0u8; byte_count];
        self.cursor.read_exact(&mut buffer)?;

        let utf16_chars: Vec<u16> = buffer
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        String::from_utf16(&utf16_chars).map_err(|_| MemoryPackError::InvalidUtf8)
    }

    #[inline(always)]
    pub fn read_bool(&mut self) -> Result<bool, MemoryPackError> {
        Ok(self.cursor.read_u8()? == 1)
    }

    #[inline(always)]
    pub fn read_i8(&mut self) -> Result<i8, MemoryPackError> {
        Ok(self.cursor.read_i8()?)
    }

    #[inline(always)]
    pub fn read_u8(&mut self) -> Result<u8, MemoryPackError> {
        Ok(self.cursor.read_u8()?)
    }

    #[inline(always)]
    pub fn read_i16(&mut self) -> Result<i16, MemoryPackError> {
        Ok(self.cursor.read_i16::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_u16(&mut self) -> Result<u16, MemoryPackError> {
        Ok(self.cursor.read_u16::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_i32(&mut self) -> Result<i32, MemoryPackError> {
        Ok(self.cursor.read_i32::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_u32(&mut self) -> Result<u32, MemoryPackError> {
        Ok(self.cursor.read_u32::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_i64(&mut self) -> Result<i64, MemoryPackError> {
        Ok(self.cursor.read_i64::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_u64(&mut self) -> Result<u64, MemoryPackError> {
        Ok(self.cursor.read_u64::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_f32(&mut self) -> Result<f32, MemoryPackError> {
        Ok(self.cursor.read_f32::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_f64(&mut self) -> Result<f64, MemoryPackError> {
        Ok(self.cursor.read_f64::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_i128(&mut self) -> Result<i128, MemoryPackError> {
        Ok(self.cursor.read_i128::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_u128(&mut self) -> Result<u128, MemoryPackError> {
        Ok(self.cursor.read_u128::<LittleEndian>()?)
    }

    #[inline(always)]
    pub fn read_char(&mut self) -> Result<char, MemoryPackError> {
        let code_point = self.cursor.read_u32::<LittleEndian>()?;
        char::from_u32(code_point).ok_or_else(|| {
            MemoryPackError::DeserializationError("Invalid Unicode code point".into())
        })
    }

    #[inline]
    pub fn skip(&mut self, n: usize) -> Result<(), MemoryPackError> {
        self.cursor.seek(SeekFrom::Current(n as i64))?;
        Ok(())
    }

    #[inline]
    pub fn rewind(&mut self, n: usize) -> Result<(), MemoryPackError> {
        self.cursor.seek(SeekFrom::Current(-(n as i64)))?;
        Ok(())
    }

    #[inline]
    pub fn position(&self) -> u64 {
        self.cursor.position()
    }
}
