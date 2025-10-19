use crate::error::{MemoryPackError, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub struct MemoryPackReader<'a> {
    pub(crate) cursor: Cursor<&'a [u8]>,
}

impl<'a> MemoryPackReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn read_string(&mut self) -> Result<String> {
        let length_or_marker = self.read_i32()?;

        if length_or_marker == -1 {
            return Ok(String::new());
        }

        if length_or_marker < 0 {
            let byte_count = !length_or_marker as usize;
            let _char_length = self.read_i32()?;

            let mut buffer = vec![0u8; byte_count];
            self.cursor.read_exact(&mut buffer)?;
            return String::from_utf8(buffer).map_err(|e| e.into());
        }

        let char_count = length_or_marker as usize;
        if char_count == 0 {
            return Ok(String::new());
        }

        let byte_count = char_count * 2;
        let mut buffer = vec![0u8; byte_count];
        self.cursor.read_exact(&mut buffer)?;

        let utf16_chars: Vec<u16> = buffer
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        String::from_utf16(&utf16_chars).map_err(|_| MemoryPackError::InvalidUtf8)
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.cursor.read_u8()? == 1)
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.cursor.read_i8()?)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.cursor.read_u8()?)
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.cursor.read_i16::<LittleEndian>()?)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(self.cursor.read_u16::<LittleEndian>()?)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.cursor.read_i32::<LittleEndian>()?)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(self.cursor.read_u32::<LittleEndian>()?)
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(self.cursor.read_i64::<LittleEndian>()?)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(self.cursor.read_u64::<LittleEndian>()?)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(self.cursor.read_f32::<LittleEndian>()?)
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(self.cursor.read_f64::<LittleEndian>()?)
    }

    pub fn position(&self) -> u64 {
        self.cursor.position()
    }
}
