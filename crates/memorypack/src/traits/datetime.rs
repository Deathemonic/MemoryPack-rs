#[cfg(feature = "chrono")]
use chrono::Timelike;

use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

const TICKS_PER_SECOND: i64 = 10_000_000;
const TICKS_PER_NANOSECOND: i64 = 100;
const DOTNET_EPOCH_TICKS: i64 = 621_355_968_000_000_000;
const TICKS_MASK: i64 = 0x3FFF_FFFF_FFFF_FFFF;
const UTC_KIND_FLAG: i64 = 1_i64 << 62;

#[cfg(feature = "chrono")]
impl MemoryPackSerialize for chrono::TimeDelta {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let ticks = self
            .num_nanoseconds()
            .ok_or_else(|| MemoryPackError::SerializationError("Duration out of range".into()))?
            / TICKS_PER_NANOSECOND;
        writer.write_i64(ticks)
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackDeserialize for chrono::TimeDelta {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let ticks = reader.read_i64()?;
        Ok(Self::nanoseconds(ticks * TICKS_PER_NANOSECOND))
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackSerialize for chrono::DateTime<chrono::Utc> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let unix_nanos = self
            .timestamp_nanos_opt()
            .ok_or_else(|| MemoryPackError::SerializationError("DateTime out of range".into()))?;
        let ticks = (unix_nanos / TICKS_PER_NANOSECOND) + DOTNET_EPOCH_TICKS;
        writer.write_i64(ticks | UTC_KIND_FLAG)
    }

    fn serialize_nullable(
        value: Option<&Self>,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError> {
        writer.write_u8(u8::from(value.is_some()))?;
        for _ in 0..7 {
            writer.write_u8(0)?;
        }
        if let Some(value) = value {
            value.serialize(writer)
        } else {
            writer.write_i64(0)?;
            Ok(())
        }
    }

    fn nullable_size_hint(_: Option<&Self>) -> usize { 16 }
}

#[cfg(feature = "chrono")]
impl MemoryPackDeserialize for chrono::DateTime<chrono::Utc> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let ticks_with_kind = reader.read_i64()?;
        let ticks = ticks_with_kind & TICKS_MASK;
        let unix_nanos = (ticks - DOTNET_EPOCH_TICKS).saturating_mul(TICKS_PER_NANOSECOND);
        Ok(Self::from_timestamp_nanos(unix_nanos))
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader
    ) -> Result<Option<Self>, MemoryPackError> {
        let has_value = reader.read_u8()? != 0;
        reader.skip(7)?;
        let ticks_with_kind = reader.read_i64()?;
        if !has_value {
            return Ok(None);
        }
        let ticks = ticks_with_kind & TICKS_MASK;
        let unix_nanos = (ticks - DOTNET_EPOCH_TICKS).saturating_mul(TICKS_PER_NANOSECOND);
        Ok(Some(Self::from_timestamp_nanos(unix_nanos)))
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackSerialize for chrono::DateTime<chrono::Local> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        self.with_timezone(&chrono::Utc).serialize(writer)
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackDeserialize for chrono::DateTime<chrono::Local> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let utc = chrono::DateTime::<chrono::Utc>::deserialize(reader)?;
        Ok(utc.with_timezone(&chrono::Local))
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackSerialize for chrono::DateTime<chrono::FixedOffset> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let offset_minutes = (self.offset().local_minus_utc() / 60) as i16;
        let utc = self.with_timezone(&chrono::Utc);
        let unix_nanos = utc
            .timestamp_nanos_opt()
            .ok_or_else(|| MemoryPackError::SerializationError("DateTime out of range".into()))?;
        let ticks = (unix_nanos / TICKS_PER_NANOSECOND) + DOTNET_EPOCH_TICKS;

        writer.write_i16(offset_minutes)?;
        writer.buffer.extend_from_slice(&[0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]);
        writer.write_i64(ticks)
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackDeserialize for chrono::DateTime<chrono::FixedOffset> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let offset_minutes = reader.read_i16()?;
        reader.read_fixed_bytes::<6>()?;
        let ticks = reader.read_i64()?;

        let unix_nanos = (ticks - DOTNET_EPOCH_TICKS).saturating_mul(TICKS_PER_NANOSECOND);
        let offset_seconds = (offset_minutes as i32) * 60;

        let utc = chrono::DateTime::from_timestamp_nanos(unix_nanos);
        let offset = chrono::FixedOffset::east_opt(offset_seconds)
            .ok_or_else(|| MemoryPackError::DeserializationError("Invalid offset".into()))?;

        Ok(utc.with_timezone(&offset))
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackSerialize for chrono::NaiveTime {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let ticks = (self.num_seconds_from_midnight() as i64 * TICKS_PER_SECOND)
            + (self.nanosecond() as i64 / TICKS_PER_NANOSECOND);
        writer.write_i64(ticks)
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackDeserialize for chrono::NaiveTime {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let ticks = reader.read_i64()?;
        let total_nanos = ticks * TICKS_PER_NANOSECOND;
        let secs = (total_nanos / 1_000_000_000) as u32;
        let nanos = (total_nanos % 1_000_000_000) as u32;

        Self::from_num_seconds_from_midnight_opt(secs, nanos)
            .ok_or_else(|| MemoryPackError::DeserializationError("Invalid time ticks".into()))
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackSerialize for chrono::NaiveDate {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let days = self
            .signed_duration_since(Self::from_ymd_opt(1, 1, 1).expect("valid chrono epoch date"))
            .num_days() as i32;
        writer.write_i32(days)
    }
}

#[cfg(feature = "chrono")]
impl MemoryPackDeserialize for chrono::NaiveDate {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let days = reader.read_i32()?;
        Self::from_ymd_opt(1, 1, 1)
            .and_then(|base| base.checked_add_days(chrono::Days::new(days as u64)))
            .ok_or_else(|| MemoryPackError::DeserializationError("Invalid date".into()))
    }
}
