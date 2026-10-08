use jiff::civil::{Date, DateTime, Time};
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Span, Timestamp, Zoned};

use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

const TICKS_PER_NANOSECOND: i64 = 100;
const NANOSECONDS_PER_SECOND: i64 = 1_000_000_000;
const DOTNET_EPOCH_TICKS: i64 = 621_355_968_000_000_000;
const TICKS_MASK: i64 = 0x3FFF_FFFF_FFFF_FFFF;
const UTC_KIND_FLAG: i64 = 1_i64 << 62;

#[inline(always)]
fn ticks_to_timestamp(ticks: i64) -> Result<Timestamp, MemoryPackError> {
    let unix_ticks =
        ticks.checked_sub(DOTNET_EPOCH_TICKS).ok_or(MemoryPackError::DateTimeOutOfRange)?;
    let unix_nanos =
        unix_ticks.checked_mul(TICKS_PER_NANOSECOND).ok_or(MemoryPackError::DateTimeOutOfRange)?;
    Timestamp::from_nanosecond(unix_nanos as i128).map_err(|_| MemoryPackError::DateTimeOutOfRange)
}

impl MemoryPackSerialize for SignedDuration {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let ticks = i64::try_from(self.as_nanos() / i128::from(TICKS_PER_NANOSECOND))
            .map_err(|_| MemoryPackError::DurationOutOfRange)?;
        writer.write_i64(ticks)
    }
}

impl MemoryPackDeserialize for SignedDuration {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let ticks = reader.read_i64()?;
        Ok(Self::from_nanos_i128(i128::from(ticks) * i128::from(TICKS_PER_NANOSECOND)))
    }
}

impl MemoryPackSerialize for Timestamp {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let ticks = i64::try_from(self.as_nanosecond() / i128::from(TICKS_PER_NANOSECOND))
            .map_err(|_| MemoryPackError::DateTimeOutOfRange)?
            .checked_add(DOTNET_EPOCH_TICKS)
            .ok_or(MemoryPackError::DateTimeOutOfRange)?;
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
        if let Some(value) = value { value.serialize(writer) } else { writer.write_i64(0) }
    }

    fn nullable_size_hint(_: Option<&Self>) -> usize { 16 }
}

impl MemoryPackDeserialize for Timestamp {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        ticks_to_timestamp(reader.read_i64()? & TICKS_MASK)
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader
    ) -> Result<Option<Self>, MemoryPackError> {
        let has_value = reader.read_u8()? != 0;
        reader.skip(7)?;
        let ticks = reader.read_i64()?;
        if !has_value { Ok(None) } else { ticks_to_timestamp(ticks & TICKS_MASK).map(Some) }
    }
}

impl MemoryPackSerialize for DateTime {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let timestamp =
            TimeZone::UTC.to_timestamp(*self).map_err(|_| MemoryPackError::DateTimeOutOfRange)?;
        let ticks = i64::try_from(timestamp.as_nanosecond() / i128::from(TICKS_PER_NANOSECOND))
            .map_err(|_| MemoryPackError::DateTimeOutOfRange)?
            .checked_add(DOTNET_EPOCH_TICKS)
            .ok_or(MemoryPackError::DateTimeOutOfRange)?;
        writer.write_i64(ticks)
    }

    fn serialize_nullable(
        value: Option<&Self>,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError> {
        writer.write_u8(u8::from(value.is_some()))?;
        for _ in 0..7 {
            writer.write_u8(0)?;
        }
        if let Some(value) = value { value.serialize(writer) } else { writer.write_i64(0) }
    }

    fn nullable_size_hint(_: Option<&Self>) -> usize { 16 }
}

impl MemoryPackDeserialize for DateTime {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let timestamp = ticks_to_timestamp(reader.read_i64()? & TICKS_MASK)?;
        Ok(TimeZone::UTC.to_datetime(timestamp))
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader
    ) -> Result<Option<Self>, MemoryPackError> {
        let has_value = reader.read_u8()? != 0;
        reader.skip(7)?;
        let ticks = reader.read_i64()?;
        if !has_value {
            Ok(None)
        } else {
            ticks_to_timestamp(ticks & TICKS_MASK)
                .map(|timestamp| Some(TimeZone::UTC.to_datetime(timestamp)))
        }
    }
}

impl MemoryPackSerialize for Zoned {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let offset_minutes = (self.offset().seconds() / 60) as i16;
        let ticks =
            i64::try_from(self.timestamp().as_nanosecond() / i128::from(TICKS_PER_NANOSECOND))
                .map_err(|_| MemoryPackError::DateTimeOutOfRange)?
                .checked_add(DOTNET_EPOCH_TICKS)
                .ok_or(MemoryPackError::DateTimeOutOfRange)?;
        writer.write_i16(offset_minutes)?;
        writer.buffer.extend_from_slice(&[0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]);
        writer.write_i64(ticks)
    }
}

impl MemoryPackDeserialize for Zoned {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let offset_minutes = reader.read_i16()?;
        reader.read_fixed_bytes::<6>()?;
        let timestamp = ticks_to_timestamp(reader.read_i64()?)?;
        let offset = Offset::from_seconds(i32::from(offset_minutes) * 60)
            .map_err(|_| MemoryPackError::InvalidOffset)?;
        Ok(Zoned::new(timestamp, TimeZone::fixed(offset)))
    }
}

impl MemoryPackSerialize for Time {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let nanos = (i64::from(self.hour()) * 3600
            + i64::from(self.minute()) * 60
            + i64::from(self.second()))
            * NANOSECONDS_PER_SECOND
            + i64::from(self.subsec_nanosecond());
        writer.write_i64(nanos / TICKS_PER_NANOSECOND)
    }
}

impl MemoryPackDeserialize for Time {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let nanos = reader
            .read_i64()?
            .checked_mul(TICKS_PER_NANOSECOND)
            .ok_or(MemoryPackError::InvalidTimeTicks)?;
        let secs = nanos.div_euclid(NANOSECONDS_PER_SECOND);
        let subsec = nanos.rem_euclid(NANOSECONDS_PER_SECOND);
        Time::new((secs / 3600) as i8, ((secs % 3600) / 60) as i8, (secs % 60) as i8, subsec as i32)
            .map_err(|_| MemoryPackError::InvalidTimeTicks)
    }
}

impl MemoryPackSerialize for Date {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let epoch = Date::new(1, 1, 1).expect("valid jiff epoch date");
        let days = self.since(epoch).map_err(|_| MemoryPackError::DateOutOfRange)?.get_days();
        writer.write_i32(days)
    }
}

impl MemoryPackDeserialize for Date {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let days = reader.read_i32()?;
        Date::new(1, 1, 1)
            .expect("valid jiff epoch date")
            .checked_add(Span::new().days(i64::from(days)))
            .map_err(|_| MemoryPackError::InvalidDate)
    }
}
