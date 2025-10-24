use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;
use hashbrown::HashMap;


impl<T> MemoryPackDeserialize for HashMap<String, T>
where
    T: MemoryPackDeserialize + Default,
{
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let count = reader.read_i32()?;
        
        if count == -1 {
            return Ok(HashMap::new());
        }
        
        if count < 0 {
            return Err(MemoryPackError::InvalidLength(count));
        }
        
        if count == 0 {
            return Ok(HashMap::new());
        }

        let mut map = HashMap::with_capacity(count as usize);
        
        for _ in 0..count {
            let key = String::deserialize(reader)?;
            let value = T::deserialize(reader)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<T> MemoryPackSerialize for HashMap<String, T>
where
    T: MemoryPackSerialize,
{
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(self.len() as i32)?;
        for (key, value) in self.iter() {
            key.serialize(writer)?;
            value.serialize(writer)?;
        }
        Ok(())
    }
}
