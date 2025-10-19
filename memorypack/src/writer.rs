use crate::error::Result;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct MemoryPackWriter {
    pub(crate) buffer: Vec<u8>,
}

impl MemoryPackWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn write_string(&mut self, value: &str) -> Result<()> {
        let bytes = value.as_bytes();
        self.write_i32(bytes.len() as i32)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    pub fn write_bool(&mut self, value: bool) -> Result<()> {
        self.buffer.write_u8(if value { 1 } else { 0 })?;
        Ok(())
    }

    pub fn write_i8(&mut self, value: i8) -> Result<()> {
        self.buffer.write_i8(value)?;
        Ok(())
    }

    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        self.buffer.write_u8(value)?;
        Ok(())
    }

    pub fn write_i16(&mut self, value: i16) -> Result<()> {
        self.buffer.write_i16::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        self.buffer.write_u16::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn write_i32(&mut self, value: i32) -> Result<()> {
        self.buffer.write_i32::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        self.buffer.write_u32::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn write_i64(&mut self, value: i64) -> Result<()> {
        self.buffer.write_i64::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        self.buffer.write_u64::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn write_f32(&mut self, value: f32) -> Result<()> {
        self.buffer.write_f32::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn write_f64(&mut self, value: f64) -> Result<()> {
        self.buffer.write_f64::<LittleEndian>(value)?;
        Ok(())
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Default for MemoryPackWriter {
    fn default() -> Self {
        Self::new()
    }
}
