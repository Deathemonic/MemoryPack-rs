use std::io::Read;
use crate::error::Result;
use crate::reader::MemoryPackReader;
use crate::writer::MemoryPackWriter;

pub trait MemoryPackSerialize {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()>;
}

pub trait MemoryPackDeserialize: Sized {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self>;
}

impl MemoryPackSerialize for bool {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_bool(*self)
    }
}

impl MemoryPackDeserialize for bool {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_bool()
    }
}

impl MemoryPackSerialize for i8 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_i8(*self)
    }
}

impl MemoryPackDeserialize for i8 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_i8()
    }
}

impl MemoryPackSerialize for u8 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_u8(*self)
    }
}

impl MemoryPackDeserialize for u8 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_u8()
    }
}

impl MemoryPackSerialize for i16 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_i16(*self)
    }
}

impl MemoryPackDeserialize for i16 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_i16()
    }
}

impl MemoryPackSerialize for u16 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_u16(*self)
    }
}

impl MemoryPackDeserialize for u16 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_u16()
    }
}

impl MemoryPackSerialize for i32 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_i32(*self)
    }
}

impl MemoryPackDeserialize for i32 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_i32()
    }
}

impl MemoryPackSerialize for u32 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_u32(*self)
    }
}

impl MemoryPackDeserialize for u32 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_u32()
    }
}

impl MemoryPackSerialize for i64 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_i64(*self)
    }
}

impl MemoryPackDeserialize for i64 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_i64()
    }
}

impl MemoryPackSerialize for u64 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_u64(*self)
    }
}

impl MemoryPackDeserialize for u64 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_u64()
    }
}

impl MemoryPackSerialize for f32 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_f32(*self)
    }
}

impl MemoryPackDeserialize for f32 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_f32()
    }
}

impl MemoryPackSerialize for f64 {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_f64(*self)
    }
}

impl MemoryPackDeserialize for f64 {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_f64()
    }
}

impl MemoryPackSerialize for String {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_string(self)
    }
}

impl MemoryPackDeserialize for String {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        reader.read_string()
    }
}

impl MemoryPackSerialize for &str {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_string(self)
    }
}

impl<T: MemoryPackSerialize> MemoryPackSerialize for Vec<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_i32(self.len() as i32)?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Vec<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        let size = reader.read_i32()?;
        if size == -1 {
            return Ok(Vec::new());
        }
        if size < 0 {
            return Err(crate::error::MemoryPackError::InvalidLength(size));
        }

        let mut result = Vec::with_capacity(size as usize);
        for _ in 0..size {
            result.push(T::deserialize(reader)?);
        }
        Ok(result)
    }
}

impl MemoryPackSerialize for Option<String> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
        writer.write_string_option(self.as_deref())
    }
}

impl MemoryPackDeserialize for Option<String> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
        let marker = reader.read_i32()?;
        match marker {
            -1 => Ok(None),
            0 => Ok(Some(String::new())),
            n if n < 0 => {
                let byte_count = !n as usize;
                let _char_length = reader.read_i32()?;
                let mut buffer = vec![0u8; byte_count];
                reader.cursor.read_exact(&mut buffer)?;
                Ok(Some(String::from_utf8(buffer)?))
            }
            _ => {
                let char_count = marker as usize;
                let byte_count = char_count * 2;
                let mut buffer = vec![0u8; byte_count];
                reader.cursor.read_exact(&mut buffer)?;
                let utf16_chars: Vec<u16> = buffer
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();
                Ok(Some(String::from_utf16(&utf16_chars)
                    .map_err(|_| crate::error::MemoryPackError::InvalidUtf8)?))
            }
        }
    }
}


macro_rules! impl_tuple {
    ($($T:ident),+) => {
        impl<$($T),+> MemoryPackSerialize for ($($T,)+)
        where
            $($T: MemoryPackSerialize,)+
        {
            #[allow(non_snake_case)]
            fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<()> {
                let ($($T,)+) = self;
                $($T.serialize(writer)?;)+
                Ok(())
            }
        }

        impl<$($T),+> MemoryPackDeserialize for ($($T,)+)
        where
            $($T: MemoryPackDeserialize,)+
        {
            fn deserialize(reader: &mut MemoryPackReader) -> Result<Self> {
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
