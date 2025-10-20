use crate::error::Result;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;
use std::collections::HashMap;

/// MemoryPack dictionary format: [header][count][entries...]
/// Each entry: [key][object_header][value]
impl<T> MemoryPackDeserialize for HashMap<String, T>
where
    T: MemoryPackDeserialize + Default,
{
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        let header = reader.read_i8()?;
        if header == -1 || header == 0 {
            return Ok(HashMap::new());
        }

        let count = reader.read_i32()?;
        if count <= 0 {
            return if count == 0 {
                Ok(HashMap::new())
            } else {
                Err(crate::error::MemoryPackError::InvalidLength(count))
            };
        }

        let capacity = ((count as usize * 4) / 3).max(count as usize);
        let mut map = HashMap::with_capacity(capacity);
        
        for _ in 0..count {
            let key = String::deserialize(reader)?;
            let _object_header = reader.read_i8()?;
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
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_i32(self.len() as i32)?;
        for (key, value) in self.iter() {
            key.serialize(writer)?;
            value.serialize(writer)?;
        }
        Ok(())
    }
}
