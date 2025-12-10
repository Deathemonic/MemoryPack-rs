use crate::error::MemoryPackError;
use crate::state::MemoryPackWriterOptionalState;
use crate::varint;

#[inline]
fn count_utf16_code_units(s: &str) -> usize {
    s.chars().map(|c| c.len_utf16()).sum()
}

pub struct MemoryPackWriter {
    pub buffer: Vec<u8>,
    pub optional_state: Option<MemoryPackWriterOptionalState>,
}

impl MemoryPackWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            optional_state: None,
        }
    }

    pub fn new_with_state() -> Self {
        Self {
            buffer: Vec::new(),
            optional_state: Some(MemoryPackWriterOptionalState::new()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            optional_state: None,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[inline]
    pub fn write_string(&mut self, value: &str) -> Result<(), MemoryPackError> {
        if value.is_empty() {
            return self.write_i32(0);
        }
        let bytes = value.as_bytes();

        let utf16_length = count_utf16_code_units(value) as i32;
        self.write_i32(!(bytes.len() as i32))?;
        self.write_i32(utf16_length)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    #[inline]
    pub fn write_string_option(&mut self, value: Option<&str>) -> Result<(), MemoryPackError> {
        match value {
            Some(s) => self.write_string(s),
            None => self.write_i32(-1),
        }
    }

    #[inline(always)]
    pub fn write_bool(&mut self, value: bool) -> Result<(), MemoryPackError> {
        self.buffer.push(value as u8);
        Ok(())
    }

    #[inline(always)]
    pub fn write_i8(&mut self, value: i8) -> Result<(), MemoryPackError> {
        self.buffer.push(value as u8);
        Ok(())
    }

    #[inline(always)]
    pub fn write_u8(&mut self, value: u8) -> Result<(), MemoryPackError> {
        self.buffer.push(value);
        Ok(())
    }

    #[inline(always)]
    pub fn write_i16(&mut self, value: i16) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u16(&mut self, value: u16) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_i32(&mut self, value: i32) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u32(&mut self, value: u32) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_i64(&mut self, value: i64) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u64(&mut self, value: u64) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_f32(&mut self, value: f32) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_f64(&mut self, value: f64) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_i128(&mut self, value: i128) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u128(&mut self, value: u128) -> Result<(), MemoryPackError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_char(&mut self, value: char) -> Result<(), MemoryPackError> {
        let code = value as u32;
        if code <= 0xFFFF {
            self.buffer.extend_from_slice(&(code as u16).to_le_bytes());
        } else {
            let adjusted = code - 0x10000;
            let high_surrogate = ((adjusted >> 10) as u16) + 0xD800;
            self.buffer.extend_from_slice(&high_surrogate.to_le_bytes());
        }
        Ok(())
    }

    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Default for MemoryPackWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPackWriter {
    pub fn write_object_reference_id(&mut self, reference_id: u32) -> Result<(), MemoryPackError> {
        self.write_u8(250)?;
        varint::write_varint(self, reference_id as i64)?;
        Ok(())
    }
}
