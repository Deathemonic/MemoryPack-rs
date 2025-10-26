use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

macro_rules! impl_tuple {
    ($($T:ident),+) => {
        impl<$($T),+> MemoryPackSerialize for ($($T,)+)
        where
            $($T: MemoryPackSerialize,)+
        {
            #[allow(non_snake_case)]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                let ($($T,)+) = self;
                $($T.serialize(writer)?;)+
                Ok(())
            }
        }

        impl<$($T),+> MemoryPackDeserialize for ($($T,)+)
        where
            $($T: MemoryPackDeserialize,)+
        {
            fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
                Ok(($($T::deserialize(reader)?,)+))
            }
        }
    };
}

impl_tuple!(T1);
impl_tuple!(T1, T2);
impl_tuple!(T1, T2, T3);
impl_tuple!(T1, T2, T3, T4);
impl_tuple!(T1, T2, T3, T4, T5);
impl_tuple!(T1, T2, T3, T4, T5, T6);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
