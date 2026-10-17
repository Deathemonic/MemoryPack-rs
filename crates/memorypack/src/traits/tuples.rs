use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

macro_rules! impl_tuple {
    ($($T:ident : $index:tt),+) => {
        impl<$($T),+> MemoryPackSerialize for ($($T,)+)
        where
            $($T: MemoryPackSerialize,)+
        {
            #[inline]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                $(self.$index.serialize(writer)?;)+

                Ok(())
            }
        }

        impl<'de, $($T),+> MemoryPackDeserialize<'de> for ($($T,)+)
        where
            $($T: MemoryPackDeserialize<'de>,)+
        {
            #[inline]
            fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
                Ok(($($T::deserialize(reader)?,)+))
            }
        }
    };
}

impl_tuple!(T1: 0);

impl_tuple!(T1: 0, T2: 1);

impl_tuple!(T1: 0, T2: 1, T3: 2);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10);

impl_tuple!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10, T12: 11);
