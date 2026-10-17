use std::{mem, slice};

use crate::error::MemoryPackError;
use crate::serialization::format::{NULL_COLLECTION_LENGTH, OBJECT_REFERENCE_HEADER};
use crate::serializer_options::{MemoryPackSerializerOptions, StringEncoding};
use crate::state::MemoryPackWriterOptionalState;
use crate::traits::MemoryPackUnmanaged;
use crate::varint;

#[derive(Default)]
pub struct MemoryPackWriter {
    pub buffer: Vec<u8>,
    pub optional_state: Option<MemoryPackWriterOptionalState>,
    options: MemoryPackSerializerOptions
}

impl MemoryPackWriter {
    pub const fn new() -> Self {
        Self {
            buffer: Vec::new(),
            optional_state: None,
            options: MemoryPackSerializerOptions::UTF8
        }
    }

    pub fn new_with_state() -> Self {
        Self {
            buffer: Vec::new(),
            optional_state: Some(MemoryPackWriterOptionalState::new()),
            options: MemoryPackSerializerOptions::UTF8
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            optional_state: None,
            options: MemoryPackSerializerOptions::UTF8
        }
    }

    pub fn with_capacity_and_options(
        capacity: usize,
        options: MemoryPackSerializerOptions
    ) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            optional_state: None,
            options
        }
    }

    pub fn new_with_options(options: MemoryPackSerializerOptions) -> Self {
        Self::with_capacity_and_options(0, options)
    }

    #[inline]
    pub(crate) fn write_unmanaged_slice<T: MemoryPackUnmanaged>(&mut self, values: &[T]) {
        let byte_length = mem::size_of_val(values);
        // SAFETY: the unmanaged types contain only initialized bytes, without
        // padding. Viewing their storage as bytes preserves alignment
        // and the slice lifetime.
        let bytes = unsafe { slice::from_raw_parts(values.as_ptr().cast::<u8>(), byte_length) };

        self.buffer.extend_from_slice(bytes);
    }

    pub const fn options(&self) -> MemoryPackSerializerOptions { self.options }

    #[inline]
    pub const fn len(&self) -> usize { self.buffer.len() }

    #[inline]
    pub const fn is_empty(&self) -> bool { self.buffer.is_empty() }

    #[inline]
    pub fn write_string(&mut self, value: &str) -> Result<(), MemoryPackError> {
        if value.is_empty() {
            return self.write_i32(0);
        }

        if self.options.string_encoding == StringEncoding::Utf16 {
            let utf16_units: Vec<_> = value.encode_utf16().collect();

            self.write_i32(utf16_units.len() as i32)?;

            for code_unit in utf16_units {
                self.write_u16(code_unit)?;
            }

            return Ok(());
        }

        let bytes = value.as_bytes();
        let utf16_length: usize = if value.is_ascii() {
            bytes.len()
        } else {
            value.chars().map(char::len_utf16).sum()
        };

        self.write_i32(!(bytes.len() as i32))?;
        self.write_i32(utf16_length as i32)?;
        self.buffer.extend_from_slice(bytes);

        Ok(())
    }

    #[inline]
    pub fn write_string_option(&mut self, value: Option<&str>) -> Result<(), MemoryPackError> {
        match value {
            Some(s) => self.write_string(s),
            None => self.write_i32(NULL_COLLECTION_LENGTH)
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
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u16(&mut self, value: u16) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_i32(&mut self, value: i32) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u32(&mut self, value: u32) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_i64(&mut self, value: i64) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u64(&mut self, value: u64) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_f32(&mut self, value: f32) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_bits().to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_f64(&mut self, value: f64) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_bits().to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_i128(&mut self, value: i128) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_u128(&mut self, value: u128) -> Result<(), MemoryPackError> {
        self.buffer
            .extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    #[inline(always)]
    pub fn write_char(&mut self, value: char) -> Result<(), MemoryPackError> {
        let code_unit =
            u16::try_from(u32::from(value)).map_err(|_| MemoryPackError::InvalidCodePoint)?;

        self.write_u16(code_unit)
    }

    #[inline]
    pub fn into_bytes(self) -> Vec<u8> { self.buffer }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] { &self.buffer }

    #[inline]
    pub fn reset(&mut self) {
        self.buffer.clear();

        if let Some(state) = &mut self.optional_state {
            state.reset();
        }
    }

    pub fn write_object_reference_id(&mut self, reference_id: u32) -> Result<(), MemoryPackError> {
        self.write_u8(OBJECT_REFERENCE_HEADER)?;
        varint::write_varint(self, reference_id as i64)?;
        Ok(())
    }
}
