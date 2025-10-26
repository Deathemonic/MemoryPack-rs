use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiDimArray<T> {
    pub dimensions: Vec<usize>,
    pub data: Vec<T>,
}

impl<T> MultiDimArray<T> {
    #[inline]
    pub fn new(dimensions: Vec<usize>, data: Vec<T>) -> Self {
        let total: usize = dimensions.iter().product();
        assert_eq!(
            total,
            data.len(),
            "Data length must match product of dimensions"
        );
        Self { dimensions, data }
    }

    #[inline]
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    #[inline]
    fn total_elements(&self) -> usize {
        self.data.len()
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for MultiDimArray<T> {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u8((self.rank() + 1) as u8)?;

        for &dim in &self.dimensions {
            writer.write_i32(dim as i32)?;
        }

        writer.write_i32(self.total_elements() as i32)?;

        for item in &self.data {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for MultiDimArray<T> {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let rank_plus_1 = reader.read_u8()?;
        let rank = (rank_plus_1 as usize).saturating_sub(1);

        if rank == 0 {
            return Err(MemoryPackError::DeserializationError(
                "Invalid array rank".into(),
            ));
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

        let total_usize = total as usize;
        let mut data = Vec::with_capacity(total_usize);

        for _ in 0..total_usize {
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
