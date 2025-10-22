use crate::error::MemoryPackError;
use crate::state::MemoryPackReaderOptionalState;
use byteorder::{LittleEndian, ReadBytesExt};
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
        String::from_utf8(buffer).map_err(|e| e.into())
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

    #[inline]
    pub fn read_bool(&mut self) -> Result<bool, MemoryPackError> {
        Ok(self.cursor.read_u8()? == 1)
    }

    #[inline]
    pub fn read_i8(&mut self) -> Result<i8, MemoryPackError> {
        Ok(self.cursor.read_i8()?)
    }

    #[inline]
    pub fn read_u8(&mut self) -> Result<u8, MemoryPackError> {
        Ok(self.cursor.read_u8()?)
    }

    #[inline]
    pub fn read_i16(&mut self) -> Result<i16, MemoryPackError> {
        Ok(self.cursor.read_i16::<LittleEndian>()?)
    }

    #[inline]
    pub fn read_u16(&mut self) -> Result<u16, MemoryPackError> {
        Ok(self.cursor.read_u16::<LittleEndian>()?)
    }

    #[inline]
    pub fn read_i32(&mut self) -> Result<i32, MemoryPackError> {
        Ok(self.cursor.read_i32::<LittleEndian>()?)
    }

    #[inline]
    pub fn read_u32(&mut self) -> Result<u32, MemoryPackError> {
        Ok(self.cursor.read_u32::<LittleEndian>()?)
    }

    #[inline]
    pub fn read_i64(&mut self) -> Result<i64, MemoryPackError> {
        Ok(self.cursor.read_i64::<LittleEndian>()?)
    }

    #[inline]
    pub fn read_u64(&mut self) -> Result<u64, MemoryPackError> {
        Ok(self.cursor.read_u64::<LittleEndian>()?)
    }

    #[inline]
    pub fn read_f32(&mut self) -> Result<f32, MemoryPackError> {
        Ok(self.cursor.read_f32::<LittleEndian>()?)
    }

    #[inline]
    pub fn read_f64(&mut self) -> Result<f64, MemoryPackError> {
        Ok(self.cursor.read_f64::<LittleEndian>()?)
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
