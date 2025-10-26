use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::writer::MemoryPackWriter;

mod codes {
    pub const MAX_SINGLE_VALUE: i8 = 127;
    pub const MIN_SINGLE_VALUE: i8 = -120;

    pub const BYTE: i8 = -121;
    pub const SBYTE: i8 = -122;
    pub const UINT16: i8 = -123;
    pub const INT16: i8 = -124;
    pub const UINT32: i8 = -125;
    pub const INT32: i8 = -126;
    pub const UINT64: i8 = -127;
    pub const INT64: i8 = -128;
}

pub fn write_varint(writer: &mut MemoryPackWriter, value: i64) -> Result<(), MemoryPackError> {
    if value >= 0 {
        if value <= codes::MAX_SINGLE_VALUE as i64 {
            writer.write_i8(value as i8)?;
        } else if value <= i16::MAX as i64 {
            writer.write_i8(codes::INT16)?;
            writer.write_i16(value as i16)?;
        } else if value <= i32::MAX as i64 {
            writer.write_i8(codes::INT32)?;
            writer.write_i32(value as i32)?;
        } else {
            writer.write_i8(codes::INT64)?;
            writer.write_i64(value)?;
        }
    } else {
        if value >= codes::MIN_SINGLE_VALUE as i64 {
            writer.write_i8(value as i8)?;
        } else if value >= i8::MIN as i64 {
            writer.write_i8(codes::SBYTE)?;
            writer.write_i8(value as i8)?;
        } else if value >= i16::MIN as i64 {
            writer.write_i8(codes::INT16)?;
            writer.write_i16(value as i16)?;
        } else if value >= i32::MIN as i64 {
            writer.write_i8(codes::INT32)?;
            writer.write_i32(value as i32)?;
        } else {
            writer.write_i8(codes::INT64)?;
            writer.write_i64(value)?;
        }
    }
    Ok(())
}

pub fn read_varint(reader: &mut MemoryPackReader) -> Result<i64, MemoryPackError> {
    let type_code = reader.read_i8()?;

    match type_code {
        codes::BYTE => Ok(reader.read_u8()? as i64),
        codes::SBYTE => Ok(reader.read_i8()? as i64),
        codes::UINT16 => Ok(reader.read_u16()? as i64),
        codes::INT16 => Ok(reader.read_i16()? as i64),
        codes::UINT32 => Ok(reader.read_u32()? as i64),
        codes::INT32 => Ok(reader.read_i32()? as i64),
        codes::UINT64 => Ok(reader.read_u64()? as i64),
        codes::INT64 => reader.read_i64(),
        _ => Ok(type_code as i64),
    }
}
