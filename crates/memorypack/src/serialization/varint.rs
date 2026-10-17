use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::writer::MemoryPackWriter;
const MAX_SINGLE_VALUE: i8 = 127;
const MIN_SINGLE_VALUE: i8 = -120;
const BYTE: i8 = -121;
const SBYTE: i8 = -122;
const UINT16: i8 = -123;
const INT16: i8 = -124;
const UINT32: i8 = -125;
const INT32: i8 = -126;
const UINT64: i8 = -127;
const INT64: i8 = -128;

pub fn write_varint(writer: &mut MemoryPackWriter, value: i64) -> Result<(), MemoryPackError> {
    if value >= 0 {
        if value <= MAX_SINGLE_VALUE as i64 {
            writer.write_i8(value as i8)?;
        } else if value <= i16::MAX as i64 {
            writer.write_i8(INT16)?;
            writer.write_i16(value as i16)?;
        } else if value <= i32::MAX as i64 {
            writer.write_i8(INT32)?;
            writer.write_i32(value as i32)?;
        } else {
            writer.write_i8(INT64)?;
            writer.write_i64(value)?;
        }
    } else if value >= MIN_SINGLE_VALUE as i64 {
        writer.write_i8(value as i8)?;
    } else if value >= i8::MIN as i64 {
        writer.write_i8(SBYTE)?;
        writer.write_i8(value as i8)?;
    } else if value >= i16::MIN as i64 {
        writer.write_i8(INT16)?;
        writer.write_i16(value as i16)?;
    } else if value >= i32::MIN as i64 {
        writer.write_i8(INT32)?;
        writer.write_i32(value as i32)?;
    } else {
        writer.write_i8(INT64)?;
        writer.write_i64(value)?;
    }

    Ok(())
}

pub fn read_varint(reader: &mut MemoryPackReader) -> Result<i64, MemoryPackError> {
    let type_code = reader.read_i8()?;

    match type_code {
        BYTE => Ok(reader.read_u8()? as i64),
        SBYTE => Ok(reader.read_i8()? as i64),
        UINT16 => Ok(reader.read_u16()? as i64),
        INT16 => Ok(reader.read_i16()? as i64),
        UINT32 => Ok(reader.read_u32()? as i64),
        INT32 => Ok(reader.read_i32()? as i64),
        UINT64 => Ok(reader.read_u64()? as i64),
        INT64 => reader.read_i64(),
        _ => Ok(type_code as i64)
    }
}
