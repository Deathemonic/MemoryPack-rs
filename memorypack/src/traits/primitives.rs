use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

impl MemoryPackSerialize for bool {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_bool(*self)
    }
}

impl MemoryPackDeserialize for bool {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_bool()
    }
}

impl MemoryPackSerialize for i8 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i8(*self)
    }
}

impl MemoryPackDeserialize for i8 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i8()
    }
}

impl MemoryPackSerialize for u8 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8(*self)
    }
}

impl MemoryPackDeserialize for u8 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u8()
    }
}

impl MemoryPackSerialize for i16 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i16(*self)
    }
}

impl MemoryPackDeserialize for i16 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i16()
    }
}

impl MemoryPackSerialize for u16 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u16(*self)
    }
}

impl MemoryPackDeserialize for u16 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u16()
    }
}

impl MemoryPackSerialize for i32 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(*self)
    }
}

impl MemoryPackDeserialize for i32 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i32()
    }
}

impl MemoryPackSerialize for u32 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u32(*self)
    }
}

impl MemoryPackDeserialize for u32 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u32()
    }
}

impl MemoryPackSerialize for i64 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i64(*self)
    }
}

impl MemoryPackDeserialize for i64 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i64()
    }
}

impl MemoryPackSerialize for u64 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u64(*self)
    }
}

impl MemoryPackDeserialize for u64 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u64()
    }
}

impl MemoryPackSerialize for f32 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f32(*self)
    }
}

impl MemoryPackDeserialize for f32 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f32()
    }
}

impl MemoryPackSerialize for f64 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f64(*self)
    }
}

impl MemoryPackDeserialize for f64 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f64()
    }
}
