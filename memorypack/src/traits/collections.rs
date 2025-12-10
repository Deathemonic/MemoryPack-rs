use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

use hashbrown::HashMap as HashbrownHashMap;
use hashbrown::HashSet as HashbrownHashSet;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

#[inline(always)]
fn validate_size(size: i32) -> Result<Option<usize>, MemoryPackError> {
    match size {
        -1 | 0 => Ok(None),
        s if s < 0 => Err(MemoryPackError::InvalidLength(s)),
        s => Ok(Some(s as usize)),
    }
}

#[inline(always)]
fn write_collection_header(
    writer: &mut MemoryPackWriter,
    len: usize,
) -> Result<(), MemoryPackError> {
    writer.write_i32(len as i32)
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Vec<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;
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
        match validate_size(size)? {
            None => Ok(Vec::new()),
            Some(capacity) => {
                let mut result = Vec::with_capacity(capacity);
                for _ in 0..capacity {
                    result.push(T::deserialize(reader)?);
                }
                Ok(result)
            }
        }
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for VecDeque<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for VecDeque<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;
        match validate_size(size)? {
            None => Ok(VecDeque::new()),
            Some(capacity) => {
                let mut result = VecDeque::with_capacity(capacity);
                for _ in 0..capacity {
                    result.push_back(T::deserialize(reader)?);
                }
                Ok(result)
            }
        }
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for LinkedList<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for LinkedList<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;
        match validate_size(size)? {
            None => Ok(LinkedList::new()),
            Some(capacity) => {
                let mut result = LinkedList::new();
                for _ in 0..capacity {
                    result.push_back(T::deserialize(reader)?);
                }
                Ok(result)
            }
        }
    }
}

impl<T: MemoryPackSerialize + Eq + std::hash::Hash> MemoryPackSerialize for HashbrownHashSet<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize + Eq + std::hash::Hash> MemoryPackDeserialize for HashbrownHashSet<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;
        match validate_size(size)? {
            None => Ok(HashbrownHashSet::new()),
            Some(capacity) => {
                let mut result = HashbrownHashSet::with_capacity(capacity);
                for _ in 0..capacity {
                    result.insert(T::deserialize(reader)?);
                }
                Ok(result)
            }
        }
    }
}

impl<T: MemoryPackSerialize + Eq + std::hash::Hash> MemoryPackSerialize for HashSet<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize + Eq + std::hash::Hash> MemoryPackDeserialize for HashSet<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;
        match validate_size(size)? {
            None => Ok(HashSet::new()),
            Some(capacity) => {
                let mut result = HashSet::with_capacity(capacity);
                for _ in 0..capacity {
                    result.insert(T::deserialize(reader)?);
                }
                Ok(result)
            }
        }
    }
}

impl<T: MemoryPackSerialize + Ord> MemoryPackSerialize for BTreeSet<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize + Ord> MemoryPackDeserialize for BTreeSet<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;
        match validate_size(size)? {
            None => Ok(BTreeSet::new()),
            Some(capacity) => {
                let mut result = BTreeSet::new();
                for _ in 0..capacity {
                    result.insert(T::deserialize(reader)?);
                }
                Ok(result)
            }
        }
    }
}

macro_rules! impl_hashmap {
    ($key_type:ty) => {
        impl<V: MemoryPackDeserialize + Default> MemoryPackDeserialize for HashbrownHashMap<$key_type, V> {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
                let count = reader.read_i32()?;
                match validate_size(count)? {
                    None => Ok(HashbrownHashMap::new()),
                    Some(capacity) => {
                        let mut map = HashbrownHashMap::with_capacity(capacity);
                        for _ in 0..capacity {
                            map.insert(<$key_type>::deserialize(reader)?, V::deserialize(reader)?);
                        }
                        Ok(map)
                    }
                }
            }
        }

        impl<V: MemoryPackSerialize> MemoryPackSerialize for HashbrownHashMap<$key_type, V> {
            #[inline(always)]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                write_collection_header(writer, self.len())?;
                for (key, value) in self.iter() {
                    key.serialize(writer)?;
                    value.serialize(writer)?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_std_hashmap {
    ($key_type:ty) => {
        impl<V: MemoryPackDeserialize + Default> MemoryPackDeserialize for HashMap<$key_type, V> {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
                let count = reader.read_i32()?;
                match validate_size(count)? {
                    None => Ok(HashMap::new()),
                    Some(capacity) => {
                        let mut map = HashMap::with_capacity(capacity);
                        for _ in 0..capacity {
                            map.insert(<$key_type>::deserialize(reader)?, V::deserialize(reader)?);
                        }
                        Ok(map)
                    }
                }
            }
        }

        impl<V: MemoryPackSerialize> MemoryPackSerialize for HashMap<$key_type, V> {
            #[inline(always)]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                write_collection_header(writer, self.len())?;
                for (key, value) in self.iter() {
                    key.serialize(writer)?;
                    value.serialize(writer)?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_btreemap {
    ($key_type:ty) => {
        impl<V: MemoryPackDeserialize + Default> MemoryPackDeserialize for BTreeMap<$key_type, V> {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
                let count = reader.read_i32()?;
                match validate_size(count)? {
                    None => Ok(BTreeMap::new()),
                    Some(capacity) => {
                        let mut map = BTreeMap::new();
                        for _ in 0..capacity {
                            map.insert(<$key_type>::deserialize(reader)?, V::deserialize(reader)?);
                        }
                        Ok(map)
                    }
                }
            }
        }

        impl<V: MemoryPackSerialize> MemoryPackSerialize for BTreeMap<$key_type, V> {
            #[inline(always)]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                write_collection_header(writer, self.len())?;
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

impl_std_hashmap!(String);
impl_std_hashmap!(i8);
impl_std_hashmap!(u8);
impl_std_hashmap!(i16);
impl_std_hashmap!(u16);
impl_std_hashmap!(i32);
impl_std_hashmap!(u32);
impl_std_hashmap!(i64);
impl_std_hashmap!(u64);
impl_std_hashmap!(i128);
impl_std_hashmap!(u128);
impl_std_hashmap!(char);

impl_btreemap!(String);
impl_btreemap!(i8);
impl_btreemap!(u8);
impl_btreemap!(i16);
impl_btreemap!(u16);
impl_btreemap!(i32);
impl_btreemap!(u32);
impl_btreemap!(i64);
impl_btreemap!(u64);
impl_btreemap!(i128);
impl_btreemap!(u128);
impl_btreemap!(char);
