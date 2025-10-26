use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;
use hashbrown::HashMap;

impl<T: MemoryPackSerialize> MemoryPackSerialize for Vec<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(self.len() as i32)?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Vec<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;
        if size == -1 {
            return Ok(Vec::new());
        }
        if size < 0 {
            return Err(MemoryPackError::InvalidLength(size));
        }

        let mut result = Vec::with_capacity(size as usize);
        for _ in 0..size {
            result.push(T::deserialize(reader)?);
        }
        Ok(result)
    }
}

macro_rules! impl_hashmap {
    ($key_type:ty) => {
        impl<T> MemoryPackDeserialize for HashMap<$key_type, T>
        where
            T: MemoryPackDeserialize + Default,
        {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
                let count = reader.read_i32()?;
                
                if count == -1 || count == 0 {
                    return Ok(HashMap::new());
                }
                
                if count < 0 {
                    return Err(MemoryPackError::InvalidLength(count));
                }

                let mut map = HashMap::with_capacity(count as usize);
                
                for _ in 0..count {
                    let key = <$key_type>::deserialize(reader)?;
                    let value = T::deserialize(reader)?;
                    map.insert(key, value);
                }

                Ok(map)
            }
        }

        impl<T> MemoryPackSerialize for HashMap<$key_type, T>
        where
            T: MemoryPackSerialize,
        {
            #[inline(always)]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                writer.write_i32(self.len() as i32)?;
                for (key, value) in self.iter() {
                    key.serialize(writer)?;
                    value.serialize(writer)?;
                }
                Ok(())
            }
        }
    };
}

impl_hashmap!(String);
impl_hashmap!(i8);
impl_hashmap!(u8);
impl_hashmap!(i16);
impl_hashmap!(u16);
impl_hashmap!(i32);
impl_hashmap!(u32);
impl_hashmap!(i64);
impl_hashmap!(u64);
impl_hashmap!(i128);
impl_hashmap!(u128);
impl_hashmap!(char);
