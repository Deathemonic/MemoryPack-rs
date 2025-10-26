use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;
use std::rc::Rc;
use std::sync::Arc;

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
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        (**self).serialize(writer)
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Rc<T> {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(Rc::new(T::deserialize(reader)?))
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Arc<T> {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        (**self).serialize(writer)
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Arc<T> {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(Arc::new(T::deserialize(reader)?))
    }
}
