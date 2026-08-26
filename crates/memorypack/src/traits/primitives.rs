use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

macro_rules! unmanaged_serialize_many {
    () => {
        #[inline]
        fn serialize_many(
            values: &[Self],
            writer: &mut MemoryPackWriter
        ) -> Result<(), MemoryPackError> {
            writer.write_unmanaged_slice(values);
            Ok(())
        }
    };
}

macro_rules! unmanaged_deserialize_many {
    () => {
        #[inline]
        fn deserialize_many(
            reader: &mut MemoryPackReader,
            count: usize
        ) -> Result<Vec<Self>, MemoryPackError> {
            reader.read_unmanaged_vec(count)
        }
    };
}

macro_rules! nullable_value {
    () => {
        fn serialize_nullable(
            value: Option<&Self>,
            writer: &mut MemoryPackWriter
        ) -> Result<(), MemoryPackError> {
            let alignment = std::mem::align_of::<Self>();
            let value_offset = (1 + alignment - 1) & !(alignment - 1);
            let size =
                (value_offset + std::mem::size_of::<Self>() + alignment - 1) & !(alignment - 1);
            writer.write_u8(u8::from(value.is_some()))?;
            for _ in 1..value_offset {
                writer.write_u8(0)?;
            }
            value.copied().unwrap_or_default().serialize(writer)?;
            for _ in value_offset + std::mem::size_of::<Self>()..size {
                writer.write_u8(0)?;
            }
            Ok(())
        }

        fn nullable_size_hint(_: Option<&Self>) -> usize {
            let alignment = std::mem::align_of::<Self>();
            let value_offset = (1 + alignment - 1) & !(alignment - 1);
            (value_offset + std::mem::size_of::<Self>() + alignment - 1) & !(alignment - 1)
        }
    };
}

macro_rules! nullable_value_deserialize {
    () => {
        fn deserialize_nullable(
            reader: &mut MemoryPackReader
        ) -> Result<Option<Self>, MemoryPackError> {
            let alignment = std::mem::align_of::<Self>();
            let value_offset = (1 + alignment - 1) & !(alignment - 1);
            let size =
                (value_offset + std::mem::size_of::<Self>() + alignment - 1) & !(alignment - 1);
            let has_value = reader.read_u8()? != 0;
            reader.skip(value_offset - 1)?;
            let value = Self::deserialize(reader)?;
            reader.skip(size - value_offset - std::mem::size_of::<Self>())?;
            Ok(has_value.then_some(value))
        }
    };
}

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
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i8(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 1 }
}

impl MemoryPackDeserialize for i8 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i8()
    }
}

impl MemoryPackSerialize for u8 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 1 }
}

impl MemoryPackDeserialize for u8 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u8()
    }
}

impl MemoryPackSerialize for i16 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i16(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 2 }
}

impl MemoryPackDeserialize for i16 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i16()
    }
}

impl MemoryPackSerialize for u16 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u16(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 2 }
}

impl MemoryPackDeserialize for u16 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u16()
    }
}

impl MemoryPackSerialize for i32 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 4 }
}

impl MemoryPackDeserialize for i32 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i32()
    }
}

impl MemoryPackSerialize for u32 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u32(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 4 }
}

impl MemoryPackDeserialize for u32 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u32()
    }
}

impl MemoryPackSerialize for i64 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i64(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 8 }
}

impl MemoryPackDeserialize for i64 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i64()
    }
}

impl MemoryPackSerialize for u64 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u64(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 8 }
}

impl MemoryPackDeserialize for u64 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u64()
    }
}

impl MemoryPackSerialize for f32 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f32(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 4 }
}

impl MemoryPackDeserialize for f32 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f32()
    }
}

impl MemoryPackSerialize for f64 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f64(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 8 }
}

impl MemoryPackDeserialize for f64 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f64()
    }
}

impl MemoryPackSerialize for i128 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i128(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 16 }
}

impl MemoryPackDeserialize for i128 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i128()
    }
}

impl MemoryPackSerialize for u128 {
    unmanaged_serialize_many!();

    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u128(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 16 }
}

impl MemoryPackDeserialize for u128 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

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
