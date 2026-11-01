use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackFixedSize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

macro_rules! unmanaged_serialize_many {
    () => {
        #[cfg(target_endian = "little")]
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
        #[cfg(target_endian = "little")]
        #[inline]
        fn deserialize_many(
            reader: &mut MemoryPackReader<'de>,
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
            Self::serialize_nullable_value(value, writer)
        }

        fn nullable_size_hint(_: Option<&Self>) -> usize { Self::NULLABLE_SIZE }
    };
}

macro_rules! nullable_value_deserialize {
    () => {
        fn deserialize_nullable(
            reader: &mut MemoryPackReader<'de>
        ) -> Result<Option<Self>, MemoryPackError> {
            Self::deserialize_nullable_value(reader)
        }
    };
}

impl MemoryPackSerialize for bool {
    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_bool(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 1 }
}

impl<'de> MemoryPackDeserialize<'de> for bool {
    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for i8 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for u8 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for i16 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for u16 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for i32 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for u32 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for i64 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for u64 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for f32 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for f64 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for i128 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

impl<'de> MemoryPackDeserialize<'de> for u128 {
    unmanaged_deserialize_many!();

    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        reader.read_u128()
    }
}

impl MemoryPackSerialize for char {
    nullable_value!();

    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_char(*self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize { 2 }
}

impl<'de> MemoryPackDeserialize<'de> for char {
    nullable_value_deserialize!();

    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        reader.read_char()
    }
}
