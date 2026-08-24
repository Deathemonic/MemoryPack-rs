use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

impl MemoryPackSerialize for bool {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_bool(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 1 }
}

impl MemoryPackDeserialize for bool {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_bool()
    }
}

impl MemoryPackSerialize for i8 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i8(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 1 }
}

impl MemoryPackDeserialize for i8 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i8()
    }
}

impl MemoryPackSerialize for u8 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 1 }
}

impl MemoryPackDeserialize for u8 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u8()
    }
}

impl MemoryPackSerialize for i16 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i16(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 2 }
}

impl MemoryPackDeserialize for i16 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i16()
    }
}

impl MemoryPackSerialize for u16 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u16(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 2 }
}

impl MemoryPackDeserialize for u16 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u16()
    }
}

impl MemoryPackSerialize for i32 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 4 }
}

impl MemoryPackDeserialize for i32 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i32()
    }
}

impl MemoryPackSerialize for u32 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u32(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 4 }
}

impl MemoryPackDeserialize for u32 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u32()
    }
}

impl MemoryPackSerialize for i64 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i64(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 8 }
}

impl MemoryPackDeserialize for i64 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i64()
    }
}

impl MemoryPackSerialize for u64 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u64(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 8 }
}

impl MemoryPackDeserialize for u64 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u64()
    }
}

impl MemoryPackSerialize for f32 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f32(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 4 }
}

impl MemoryPackDeserialize for f32 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f32()
    }
}

impl MemoryPackSerialize for f64 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f64(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 8 }
}

impl MemoryPackDeserialize for f64 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f64()
    }
}

impl MemoryPackSerialize for i128 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i128(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 16 }
}

impl MemoryPackDeserialize for i128 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i128()
    }
}

impl MemoryPackSerialize for u128 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u128(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 16 }
}

impl MemoryPackDeserialize for u128 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u128()
    }
}

impl MemoryPackSerialize for char {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_char(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 2 }
}

impl MemoryPackDeserialize for char {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_char()
    }
}
