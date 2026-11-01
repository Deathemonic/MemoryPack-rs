use crate::{
    MemoryPackDeserialize,
    MemoryPackError,
    MemoryPackReader,
    MemoryPackSerialize,
    MemoryPackWriter
};

pub trait MemoryPackFixedSize: Sized {
    const SIZE: usize;
    const ALIGNMENT: usize;
    const NULLABLE_SIZE: usize = (Self::ALIGNMENT + Self::SIZE).next_multiple_of(Self::ALIGNMENT);

    fn serialize_nullable_value(
        value: Option<&Self>,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError>
    where
        Self: MemoryPackSerialize
    {
        writer.write_u8(u8::from(value.is_some()))?;

        for _ in 1..Self::ALIGNMENT {
            writer.write_u8(0)?;
        }

        if let Some(value) = value {
            value.serialize(writer)?;
        } else {
            writer.buffer.resize(writer.len() + Self::SIZE, 0);
        }

        for _ in Self::ALIGNMENT + Self::SIZE..Self::NULLABLE_SIZE {
            writer.write_u8(0)?;
        }

        Ok(())
    }

    fn deserialize_nullable_value<'de>(
        reader: &mut MemoryPackReader<'de>
    ) -> Result<Option<Self>, MemoryPackError>
    where
        Self: MemoryPackDeserialize<'de>
    {
        let has_value = reader.read_u8()? != 0;
        reader.skip(Self::ALIGNMENT - 1)?;

        let value = if has_value {
            Some(Self::deserialize(reader)?)
        } else {
            reader.skip(Self::SIZE)?;

            None
        };

        reader.skip(Self::NULLABLE_SIZE - Self::ALIGNMENT - Self::SIZE)?;

        Ok(value)
    }
}

pub trait MemoryPackNonNullableValue: MemoryPackFixedSize {}

macro_rules! impl_fixed_size {
    ($($value_type:ty),+ $(,)?) => {
        $(
            impl MemoryPackFixedSize for $value_type {
                const SIZE: usize = size_of::<Self>();
                const ALIGNMENT: usize = align_of::<Self>();
            }

            impl MemoryPackNonNullableValue for $value_type {}
        )+
    };
}

impl_fixed_size!(bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64);

impl MemoryPackFixedSize for char {
    const ALIGNMENT: usize = align_of::<u16>();
    const SIZE: usize = size_of::<u16>();
}

impl MemoryPackNonNullableValue for char {}

impl<T: MemoryPackNonNullableValue> MemoryPackFixedSize for Option<T> {
    const ALIGNMENT: usize = T::ALIGNMENT;
    const SIZE: usize = T::NULLABLE_SIZE;
}
