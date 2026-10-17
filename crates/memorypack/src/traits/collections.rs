use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

#[cfg(feature = "ahash")]
use ahash::{AHashMap, AHashSet};
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap as HashbrownHashMap;
#[cfg(feature = "hashbrown")]
use hashbrown::HashSet as HashbrownHashSet;
#[cfg(feature = "fxhash")]
use rustc_hash::{FxHashMap, FxHashSet};

use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::serialization::format::{COLLECTION_HEADER_SIZE, NULL_COLLECTION_LENGTH};
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

macro_rules! impl_std_hashmap {
    ($key_type:ty) => {
        impl<'de, V: MemoryPackDeserialize<'de> + Default> MemoryPackDeserialize<'de>
            for HashMap<$key_type, V>
        {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

            #[inline]
            fn serialized_size_hint(&self) -> usize {
                4 + self
                    .iter()
                    .map(|(key, value)| key.serialized_size_hint() + value.serialized_size_hint())
                    .sum::<usize>()
            }
        }
    };
}

macro_rules! impl_btreemap {
    ($key_type:ty) => {
        impl<'de, V: MemoryPackDeserialize<'de> + Default> MemoryPackDeserialize<'de>
            for BTreeMap<$key_type, V>
        {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

#[cfg(feature = "hashbrown")]
macro_rules! impl_hashbrown_hashmap {
    ($key_type:ty) => {
        impl<'de, V: MemoryPackDeserialize<'de> + Default> MemoryPackDeserialize<'de>
            for HashbrownHashMap<$key_type, V>
        {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
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

#[cfg(feature = "ahash")]
macro_rules! impl_ahash_hashmap {
    ($key_type:ty) => {
        impl<'de, V: MemoryPackDeserialize<'de> + Default> MemoryPackDeserialize<'de>
            for AHashMap<$key_type, V>
        {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
                let count = reader.read_i32()?;
                match validate_size(count)? {
                    None => Ok(AHashMap::new()),
                    Some(capacity) => {
                        let mut map = AHashMap::with_capacity(capacity);
                        for _ in 0..capacity {
                            map.insert(<$key_type>::deserialize(reader)?, V::deserialize(reader)?);
                        }
                        Ok(map)
                    }
                }
            }
        }

        impl<V: MemoryPackSerialize> MemoryPackSerialize for AHashMap<$key_type, V> {
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

#[cfg(feature = "fxhash")]
macro_rules! impl_fxhash_hashmap {
    ($key_type:ty) => {
        impl<'de, V: MemoryPackDeserialize<'de> + Default> MemoryPackDeserialize<'de>
            for FxHashMap<$key_type, V>
        {
            #[inline(always)]
            fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
                let count = reader.read_i32()?;
                match validate_size(count)? {
                    None => Ok(FxHashMap::default()),
                    Some(capacity) => {
                        let mut map = FxHashMap::with_capacity_and_hasher(
                            capacity,
                            rustc_hash::FxBuildHasher::default()
                        );
                        for _ in 0..capacity {
                            map.insert(<$key_type>::deserialize(reader)?, V::deserialize(reader)?);
                        }
                        Ok(map)
                    }
                }
            }
        }

        impl<V: MemoryPackSerialize> MemoryPackSerialize for FxHashMap<$key_type, V> {
            #[inline(always)]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
                write_collection_header(writer, self.len())?;
                for (key, value) in self.iter() {
                    key.serialize(writer)?;
                    value.serialize(writer)?;
                }
                Ok(())
            }

            #[inline]
            fn serialized_size_hint(&self) -> usize {
                4 + self
                    .iter()
                    .map(|(key, value)| key.serialized_size_hint() + value.serialized_size_hint())
                    .sum::<usize>()
            }
        }
    };
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Vec<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;
        T::serialize_many(self, writer)
    }

    #[inline]
    fn serialized_size_hint(&self) -> usize {
        4 + self
            .iter()
            .map(MemoryPackSerialize::serialized_size_hint)
            .sum::<usize>()
    }

    fn serialize_nullable(
        value: Option<&Self>,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError> {
        match value {
            Some(value) => value.serialize(writer),
            None => writer.write_i32(NULL_COLLECTION_LENGTH)
        }
    }

    fn nullable_size_hint(value: Option<&Self>) -> usize {
        value.map_or(4, MemoryPackSerialize::serialized_size_hint)
    }
}

impl<'de, T: MemoryPackDeserialize<'de>> MemoryPackDeserialize<'de> for Vec<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        validate_size(size)?
            .map_or_else(|| Ok(Self::new()), |capacity| T::deserialize_many(reader, capacity))
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader<'de>
    ) -> Result<Option<Self>, MemoryPackError> {
        let size = reader.read_i32()?;

        if size == NULL_COLLECTION_LENGTH {
            return Ok(None);
        }

        if size < 0 {
            return Err(MemoryPackError::InvalidLength(size));
        }

        Ok(Some(T::deserialize_many(reader, size as usize)?))
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for VecDeque<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;

        for item in self {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

impl<'de, T: MemoryPackDeserialize<'de>> MemoryPackDeserialize<'de> for VecDeque<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        match validate_size(size)? {
            None => Ok(Self::new()),
            Some(capacity) => {
                let mut result = Self::with_capacity(capacity);

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

        for item in self {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

impl<'de, T: MemoryPackDeserialize<'de>> MemoryPackDeserialize<'de> for LinkedList<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        match validate_size(size)? {
            None => Ok(Self::new()),
            Some(capacity) => {
                let mut result = Self::new();

                for _ in 0..capacity {
                    result.push_back(T::deserialize(reader)?);
                }

                Ok(result)
            }
        }
    }
}

impl<T: MemoryPackSerialize + Eq + Hash> MemoryPackSerialize for HashSet<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;

        for item in self {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

impl<'de, T: MemoryPackDeserialize<'de> + Eq + Hash> MemoryPackDeserialize<'de> for HashSet<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        match validate_size(size)? {
            None => Ok(Self::new()),
            Some(capacity) => {
                let mut result = Self::with_capacity(capacity);

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

        for item in self {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

impl<'de, T: MemoryPackDeserialize<'de> + Ord> MemoryPackDeserialize<'de> for BTreeSet<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        match validate_size(size)? {
            None => Ok(Self::new()),
            Some(capacity) => {
                let mut result = Self::new();

                for _ in 0..capacity {
                    result.insert(T::deserialize(reader)?);
                }

                Ok(result)
            }
        }
    }
}

#[cfg(feature = "hashbrown")]
impl<T: MemoryPackSerialize + Eq + Hash> MemoryPackSerialize for HashbrownHashSet<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;

        for item in self {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "hashbrown")]
impl<'de, T: MemoryPackDeserialize<'de> + Eq + Hash> MemoryPackDeserialize<'de>
    for HashbrownHashSet<T>
{
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        match validate_size(size)? {
            None => Ok(Self::new()),
            Some(capacity) => {
                let mut result = Self::with_capacity(capacity);

                for _ in 0..capacity {
                    result.insert(T::deserialize(reader)?);
                }

                Ok(result)
            }
        }
    }
}

#[cfg(feature = "ahash")]
impl<T: MemoryPackSerialize + Eq + Hash> MemoryPackSerialize for AHashSet<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;

        for item in self {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "ahash")]
impl<'de, T: MemoryPackDeserialize<'de> + Eq + Hash> MemoryPackDeserialize<'de> for AHashSet<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        match validate_size(size)? {
            None => Ok(Self::new()),
            Some(capacity) => {
                let mut result = Self::with_capacity(capacity);

                for _ in 0..capacity {
                    result.insert(T::deserialize(reader)?);
                }

                Ok(result)
            }
        }
    }
}

#[cfg(feature = "fxhash")]
impl<T: MemoryPackSerialize + Eq + Hash> MemoryPackSerialize for FxHashSet<T> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;

        for item in self {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "fxhash")]
impl<'de, T: MemoryPackDeserialize<'de> + Eq + Hash> MemoryPackDeserialize<'de> for FxHashSet<T> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let size = reader.read_i32()?;

        match validate_size(size)? {
            None => Ok(Self::default()),
            Some(capacity) => {
                let mut result =
                    Self::with_capacity_and_hasher(capacity, rustc_hash::FxBuildHasher);

                for _ in 0..capacity {
                    result.insert(T::deserialize(reader)?);
                }

                Ok(result)
            }
        }
    }
}

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

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(String);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(i8);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(u8);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(i16);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(u16);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(i32);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(u32);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(i64);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(u64);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(i128);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(u128);

#[cfg(feature = "hashbrown")]
impl_hashbrown_hashmap!(char);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(String);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(i8);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(u8);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(i16);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(u16);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(i32);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(u32);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(i64);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(u64);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(i128);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(u128);

#[cfg(feature = "ahash")]
impl_ahash_hashmap!(char);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(String);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(i8);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(u8);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(i16);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(u16);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(i32);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(u32);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(i64);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(u64);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(i128);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(u128);

#[cfg(feature = "fxhash")]
impl_fxhash_hashmap!(char);

impl<T: MemoryPackSerialize> MemoryPackSerialize for &[T] {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        write_collection_header(writer, self.len())?;

        T::serialize_many(self, writer)
    }

    fn serialize_nullable(
        value: Option<&Self>,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError> {
        match value {
            Some(value) => value.serialize(writer),
            None => writer.write_i32(NULL_COLLECTION_LENGTH)
        }
    }
}

impl<'de: 'value, 'value> MemoryPackDeserialize<'de> for &'value [u8] {
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let length = reader.read_i32()?;
        let count = validate_size(length)?.unwrap_or_default();

        reader.read_bytes(count)
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader<'de>
    ) -> Result<Option<Self>, MemoryPackError> {
        let length = reader.read_i32()?;

        if length == NULL_COLLECTION_LENGTH {
            return Ok(None);
        }

        reader.rewind(COLLECTION_HEADER_SIZE)?;

        Self::deserialize(reader).map(Some)
    }
}

#[inline(always)]
const fn validate_size(size: i32) -> Result<Option<usize>, MemoryPackError> {
    match size {
        NULL_COLLECTION_LENGTH | 0 => Ok(None),
        s if s < 0 => Err(MemoryPackError::InvalidLength(s)),
        s => Ok(Some(s as usize))
    }
}

#[inline(always)]
fn write_collection_header(
    writer: &mut MemoryPackWriter,
    len: usize
) -> Result<(), MemoryPackError> {
    writer.write_i32(len as i32)
}
