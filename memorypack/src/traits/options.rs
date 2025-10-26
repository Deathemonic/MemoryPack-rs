use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

#[inline]
pub(super) fn serialize_option_generic<T: MemoryPackSerialize + Default>(opt: &Option<T>, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
    match opt {
        Some(value) => {
            writer.write_i32(1)?;
            value.serialize(writer)?;
        }
        None => {
            writer.write_i32(0)?;
            T::default().serialize(writer)?;
        }
    }
    Ok(())
}

#[inline]
pub(super) fn deserialize_option_generic<T: MemoryPackDeserialize>(reader: &mut MemoryPackReader) -> Result<Option<T>, MemoryPackError> {
    let has_value = reader.read_i32()?;
    let value = T::deserialize(reader)?;
    if has_value == 0 {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

#[inline]
pub(super) fn serialize_nullable_string(opt: &Option<String>, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
    match opt {
        Some(value) => writer.write_string(value),
        None => writer.write_i32(-1),
    }
}

#[inline]
pub(super) fn deserialize_nullable_string(reader: &mut MemoryPackReader) -> Result<Option<String>, MemoryPackError> {
    let len = reader.read_i32()?;
    if len == -1 {
        Ok(None)
    } else {
        reader.rewind(4)?;
        Ok(Some(String::deserialize(reader)?))
    }
}

#[inline]
pub(super) fn serialize_nullable_vec<T: MemoryPackSerialize>(opt: &Option<Vec<T>>, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
    match opt {
        Some(vec) => {
            writer.write_i32(vec.len() as i32)?;
            for item in vec.iter() {
                item.serialize(writer)?;
            }
            Ok(())
        }
        None => writer.write_i32(-1),
    }
}

#[inline]
pub(super) fn deserialize_nullable_vec<T: MemoryPackDeserialize>(reader: &mut MemoryPackReader) -> Result<Option<Vec<T>>, MemoryPackError> {
    let size = reader.read_i32()?;
    if size == -1 {
        return Ok(None);
    }
    if size < 0 {
        return Err(MemoryPackError::InvalidLength(size));
    }

    let mut result = Vec::with_capacity(size as usize);
    for _ in 0..size {
        result.push(T::deserialize(reader)?);
    }
    Ok(Some(result))
}

#[cfg(feature = "nightly")]
mod option_impls {
    use super::*;

    impl<T: MemoryPackSerialize + Default> MemoryPackSerialize for Option<T> {
        #[inline]
        default fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
            serialize_option_generic(self, writer)
        }
    }

    impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Option<T> {
        #[inline]
        default fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
            deserialize_option_generic(reader)
        }
    }

    impl MemoryPackSerialize for Option<String> {
        #[inline]
        fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
            serialize_nullable_string(self, writer)
        }
    }

    impl MemoryPackDeserialize for Option<String> {
        #[inline]
        fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
            deserialize_nullable_string(reader)
        }
    }

    impl<T: MemoryPackSerialize> MemoryPackSerialize for Option<Vec<T>> {
        #[inline]
        fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
            serialize_nullable_vec(self, writer)
        }
    }

    impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Option<Vec<T>> {
        #[inline]
        fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
            deserialize_nullable_vec(reader)
        }
    }
}

#[cfg(not(feature = "nightly"))]
mod option_impls {
    use super::*;

    impl<T: MemoryPackSerialize + Default> MemoryPackSerialize for Option<T> {
        #[inline]
        fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
            serialize_option_generic(self, writer)
        }
    }

    impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Option<T> {
        #[inline]
        fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
            deserialize_option_generic(reader)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NullableString(pub Option<String>);

    impl MemoryPackSerialize for NullableString {
        #[inline]
        fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
            serialize_nullable_string(&self.0, writer)
        }
    }

    impl MemoryPackDeserialize for NullableString {
        #[inline]
        fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
            Ok(NullableString(deserialize_nullable_string(reader)?))
        }
    }

    #[cfg(feature = "serde")]
    impl serde::Serialize for NullableString {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serde::Serialize::serialize(&self.0, serializer)
        }
    }
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NullableVec<T>(pub Option<Vec<T>>);

    impl<T: MemoryPackSerialize> MemoryPackSerialize for NullableVec<T> {
        #[inline]
        fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
            serialize_nullable_vec(&self.0, writer)
        }
    }

    impl<T: MemoryPackDeserialize> MemoryPackDeserialize for NullableVec<T> {
        #[inline]
        fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
            Ok(NullableVec(deserialize_nullable_vec(reader)?))
        }
    }

    #[cfg(feature = "serde")]
    impl<T: serde::Serialize> serde::Serialize for NullableVec<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serde::Serialize::serialize(&self.0, serializer)
        }
    }
}

#[cfg(not(feature = "nightly"))]
pub use option_impls::{NullableString, NullableVec};
