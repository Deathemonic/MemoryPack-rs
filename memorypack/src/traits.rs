use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::writer::MemoryPackWriter;

use std::sync::Arc;
use std::rc::Rc;

pub trait MemoryPackSerialize {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError>;
}

pub trait MemoryPackDeserialize: Sized {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError>;
}

pub trait MemoryPackDeserializeZeroCopy<'a>: Sized {
    fn deserialize(reader: &mut MemoryPackReader<'a>) -> Result<Self, MemoryPackError>;
}

impl MemoryPackSerialize for bool {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_bool(*self)
    }
}

impl MemoryPackDeserialize for bool {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_bool()
    }
}

impl MemoryPackSerialize for i8 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i8(*self)
    }
}

impl MemoryPackDeserialize for i8 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i8()
    }
}

impl MemoryPackSerialize for u8 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8(*self)
    }
}

impl MemoryPackDeserialize for u8 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u8()
    }
}

impl MemoryPackSerialize for i16 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i16(*self)
    }
}

impl MemoryPackDeserialize for i16 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i16()
    }
}

impl MemoryPackSerialize for u16 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u16(*self)
    }
}

impl MemoryPackDeserialize for u16 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u16()
    }
}

impl MemoryPackSerialize for i32 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(*self)
    }
}

impl MemoryPackDeserialize for i32 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i32()
    }
}

impl MemoryPackSerialize for u32 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u32(*self)
    }
}

impl MemoryPackDeserialize for u32 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u32()
    }
}

impl MemoryPackSerialize for i64 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i64(*self)
    }
}

impl MemoryPackDeserialize for i64 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_i64()
    }
}

impl MemoryPackSerialize for u64 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u64(*self)
    }
}

impl MemoryPackDeserialize for u64 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_u64()
    }
}

impl MemoryPackSerialize for f32 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f32(*self)
    }
}

impl MemoryPackDeserialize for f32 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f32()
    }
}

impl MemoryPackSerialize for f64 {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f64(*self)
    }
}

impl MemoryPackDeserialize for f64 {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_f64()
    }
}

impl MemoryPackSerialize for String {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_string(self)
    }
}

impl MemoryPackDeserialize for String {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_string()
    }
}

impl MemoryPackSerialize for &str {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_string(self)
    }
}

impl<'a> MemoryPackDeserializeZeroCopy<'a> for &'a str {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader<'a>) -> Result<Self, MemoryPackError> {
        reader.read_str()
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Vec<T> {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_i32(self.len() as i32)?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Vec<T> {
    #[inline]
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


#[inline]
fn serialize_option_generic<T: MemoryPackSerialize + Default>(opt: &Option<T>, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
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
fn deserialize_option_generic<T: MemoryPackDeserialize>(reader: &mut MemoryPackReader) -> Result<Option<T>, MemoryPackError> {
    let has_value = reader.read_i32()?;
    let value = T::deserialize(reader)?;
    if has_value == 0 {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

#[inline]
fn serialize_nullable_string(opt: &Option<String>, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
    match opt {
        Some(value) => writer.write_string(value),
        None => writer.write_i32(-1),
    }
}

#[inline]
fn deserialize_nullable_string(reader: &mut MemoryPackReader) -> Result<Option<String>, MemoryPackError> {
    let len = reader.read_i32()?;
    if len == -1 {
        Ok(None)
    } else {
        reader.rewind(4)?;
        Ok(Some(String::deserialize(reader)?))
    }
}

#[inline]
fn serialize_nullable_vec<T: MemoryPackSerialize>(opt: &Option<Vec<T>>, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
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
fn deserialize_nullable_vec<T: MemoryPackDeserialize>(reader: &mut MemoryPackReader) -> Result<Option<Vec<T>>, MemoryPackError> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiDimArray<T> {
    pub dimensions: Vec<usize>,
    pub data: Vec<T>,
}

impl<T> MultiDimArray<T> {
    pub fn new(dimensions: Vec<usize>, data: Vec<T>) -> Self {
        let total: usize = dimensions.iter().product();
        assert_eq!(total, data.len(), "Data length must match product of dimensions");
        Self { dimensions, data }
    }
    
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for MultiDimArray<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8((self.rank() + 1) as u8)?;
        
        for &dim in &self.dimensions {
            writer.write_i32(dim as i32)?;
        }
        
        let total: usize = self.dimensions.iter().product();
        writer.write_i32(total as i32)?;
        
        for item in &self.data {
            item.serialize(writer)?;
        }
        
        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for MultiDimArray<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let rank_plus_1 = reader.read_u8()?;
        let rank = (rank_plus_1 as usize).saturating_sub(1);
        
        if rank == 0 {
            return Err(MemoryPackError::DeserializationError("Invalid array rank".into()));
        }
        
        let mut dimensions = Vec::with_capacity(rank);
        for _ in 0..rank {
            let dim = reader.read_i32()?;
            if dim < 0 {
                return Err(MemoryPackError::InvalidLength(dim));
            }
            dimensions.push(dim as usize);
        }
        
        let total = reader.read_i32()?;
        if total < 0 {
            return Err(MemoryPackError::InvalidLength(total));
        }
        
        let mut data = Vec::with_capacity(total as usize);
        for _ in 0..total {
            data.push(T::deserialize(reader)?);
        }
        
        Ok(MultiDimArray { dimensions, data })
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for MultiDimArray<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MultiDimArray", 2)?;
        state.serialize_field("dimensions", &self.dimensions)?;
        state.serialize_field("data", &self.data)?;
        state.end()
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Box<T> {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        (**self).serialize(writer)
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Box<T> {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(Box::new(T::deserialize(reader)?))
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Rc<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        (**self).serialize(writer)
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Rc<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(Rc::new(T::deserialize(reader)?))
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Arc<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        (**self).serialize(writer)
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Arc<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(Arc::new(T::deserialize(reader)?))
    }
}


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
