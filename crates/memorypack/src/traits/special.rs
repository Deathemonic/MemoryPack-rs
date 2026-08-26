use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lazy<T>(pub T);

impl<T: MemoryPackSerialize> MemoryPackSerialize for Lazy<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8(1)?;
        self.0.serialize(writer)
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Lazy<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let count = reader.read_u8()?;
        if count != 1 {
            return Err(MemoryPackError::DeserializationError(format!(
                "Invalid Lazy field count: expected 1, got {count}"
            )));
        }
        Ok(Self(T::deserialize(reader)?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stack<T>(pub Vec<T>);

impl<T: MemoryPackSerialize> MemoryPackSerialize for Stack<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(self.0.len() as i32)?;
        for value in self.0.iter().rev() {
            value.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Stack<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let count = reader.read_i32()?;
        if count < 0 {
            return Err(MemoryPackError::InvalidLength(count));
        }
        let mut values = T::deserialize_many(reader, count as usize)?;
        values.reverse();
        Ok(Self(values))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple<T>(pub T);

#[doc(hidden)]
pub trait TupleElements: Sized {
    const COUNT: u8;
    fn serialize_elements(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError>;
    fn deserialize_elements(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError>;
}

impl<T: TupleElements> MemoryPackSerialize for Tuple<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8(T::COUNT)?;
        self.0.serialize_elements(writer)
    }
}

impl<T: TupleElements> MemoryPackDeserialize for Tuple<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let count = reader.read_u8()?;
        if count != T::COUNT {
            return Err(MemoryPackError::DeserializationError(format!(
                "Invalid Tuple field count: expected {}, got {count}",
                T::COUNT
            )));
        }
        Ok(Self(T::deserialize_elements(reader)?))
    }
}

macro_rules! impl_tuple_elements {
    ($count:literal; $($T:ident),+) => {
        impl<$($T),+> TupleElements for ($($T,)+)
        where
            $($T: MemoryPackSerialize + MemoryPackDeserialize,)+
        {
            const COUNT: u8 = $count;

            #[allow(non_snake_case)]
            fn serialize_elements(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                let ($($T,)+) = self;
                $($T.serialize(writer)?;)+
                Ok(())
            }

            fn deserialize_elements(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
                Ok(($($T::deserialize(reader)?,)+))
            }
        }
    };
}

impl_tuple_elements!(1; T1);
impl_tuple_elements!(2; T1, T2);
impl_tuple_elements!(3; T1, T2, T3);
impl_tuple_elements!(4; T1, T2, T3, T4);
impl_tuple_elements!(5; T1, T2, T3, T4, T5);
impl_tuple_elements!(6; T1, T2, T3, T4, T5, T6);
impl_tuple_elements!(7; T1, T2, T3, T4, T5, T6, T7);
impl_tuple_elements!(8; T1, T2, T3, T4, T5, T6, T7, T8);
