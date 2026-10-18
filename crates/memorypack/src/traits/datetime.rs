use jiff::civil::{Date, DateTime, Time};
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Span, Timestamp, Zoned};

use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;
const NANOSECONDS_PER_TICK: i64 = 100;
const NANOSECONDS_PER_SECOND: i64 = 1_000_000_000;
const SECONDS_PER_MINUTE: i32 = 60;
const SECONDS_PER_HOUR: i32 = 3600;
const DOTNET_EPOCH_TICKS: i64 = 621_355_968_000_000_000;
const TICKS_MASK: i64 = 0x3FFF_FFFF_FFFF_FFFF;
const UTC_KIND_FLAG: i64 = 1_i64 << 62;

impl MemoryPackSerialize for SignedDuration {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let ticks = i64::try_from(self.as_nanos() / i128::from(NANOSECONDS_PER_TICK))
            .map_err(|_| MemoryPackError::DurationOutOfRange)?;

        writer.write_i64(ticks)
    }
}

impl<'de> MemoryPackDeserialize<'de> for SignedDuration {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let ticks = reader.read_i64()?;

        Ok(Self::from_nanos_i128(i128::from(ticks) * i128::from(NANOSECONDS_PER_TICK)))
    }
}

impl MemoryPackSerialize for Timestamp {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let ticks = i64::try_from(self.as_nanosecond() / i128::from(NANOSECONDS_PER_TICK))
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

        if let Some(value) = value {
            value.serialize(writer)
        } else {
            writer.write_i64(0)
        }
    }

    fn nullable_size_hint(_: Option<&Self>) -> usize { 16 }
}

impl<'de> MemoryPackDeserialize<'de> for Timestamp {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        ticks_to_timestamp(reader.read_i64()? & TICKS_MASK)
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader<'de>
    ) -> Result<Option<Self>, MemoryPackError> {
        let has_value = reader.read_u8()? != 0;

        reader.skip(7)?;
        let ticks = reader.read_i64()?;

        if has_value {
            ticks_to_timestamp(ticks & TICKS_MASK).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl MemoryPackSerialize for DateTime {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let timestamp = TimeZone::UTC
            .to_timestamp(*self)
            .map_err(|_| MemoryPackError::DateTimeOutOfRange)?;
        let ticks = i64::try_from(timestamp.as_nanosecond() / i128::from(NANOSECONDS_PER_TICK))
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

        if let Some(value) = value {
            value.serialize(writer)
        } else {
            writer.write_i64(0)
        }
    }

    fn nullable_size_hint(_: Option<&Self>) -> usize { 16 }
}

impl<'de> MemoryPackDeserialize<'de> for DateTime {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let timestamp = ticks_to_timestamp(reader.read_i64()? & TICKS_MASK)?;

        Ok(TimeZone::UTC.to_datetime(timestamp))
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader<'de>
    ) -> Result<Option<Self>, MemoryPackError> {
        let has_value = reader.read_u8()? != 0;

        reader.skip(7)?;
        let ticks = reader.read_i64()?;

        if has_value {
            ticks_to_timestamp(ticks & TICKS_MASK)
                .map(|timestamp| Some(TimeZone::UTC.to_datetime(timestamp)))
        } else {
            Ok(None)
        }
    }
}

impl MemoryPackSerialize for Zoned {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let offset_minutes = (self.offset().seconds() / SECONDS_PER_MINUTE) as i16;
        let ticks =
            i64::try_from(self.timestamp().as_nanosecond() / i128::from(NANOSECONDS_PER_TICK))
                .map_err(|_| MemoryPackError::DateTimeOutOfRange)?
                .checked_add(DOTNET_EPOCH_TICKS)
                .ok_or(MemoryPackError::DateTimeOutOfRange)?;

        writer.write_i16(offset_minutes)?;
        writer
            .buffer
            .extend_from_slice(&[0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]);

        writer.write_i64(ticks)
    }
}

impl<'de> MemoryPackDeserialize<'de> for Zoned {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let offset_minutes = reader.read_i16()?;

        reader.read_fixed_bytes::<6>()?;
        let timestamp = ticks_to_timestamp(reader.read_i64()?)?;
        let offset = Offset::from_seconds(i32::from(offset_minutes) * SECONDS_PER_MINUTE)
            .map_err(|_| MemoryPackError::InvalidOffset)?;

        Ok(Self::new(timestamp, TimeZone::fixed(offset)))
    }
}

impl MemoryPackSerialize for Time {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let nanos = (i64::from(self.hour()) * i64::from(SECONDS_PER_HOUR)
            + i64::from(self.minute()) * i64::from(SECONDS_PER_MINUTE)
            + i64::from(self.second()))
            * NANOSECONDS_PER_SECOND
            + i64::from(self.subsec_nanosecond());

        writer.write_i64(nanos / NANOSECONDS_PER_TICK)
    }
}

impl<'de> MemoryPackDeserialize<'de> for Time {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let nanos = reader
            .read_i64()?
            .checked_mul(NANOSECONDS_PER_TICK)
            .ok_or(MemoryPackError::InvalidTimeTicks)?;
        let secs = nanos.div_euclid(NANOSECONDS_PER_SECOND);
        let subsec = nanos.rem_euclid(NANOSECONDS_PER_SECOND);

        Self::new(
            (secs / i64::from(SECONDS_PER_HOUR)) as i8,
            ((secs % i64::from(SECONDS_PER_HOUR)) / i64::from(SECONDS_PER_MINUTE)) as i8,
            (secs % i64::from(SECONDS_PER_MINUTE)) as i8,
            subsec as i32
        )
        .map_err(|_| MemoryPackError::InvalidTimeTicks)
    }
}

impl MemoryPackSerialize for Date {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let epoch = Self::new(1, 1, 1).expect("valid jiff epoch date");
        let days = self
            .since(epoch)
            .map_err(|_| MemoryPackError::DateOutOfRange)?
            .get_days();

        writer.write_i32(days)
    }
}

impl<'de> MemoryPackDeserialize<'de> for Date {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let days = reader.read_i32()?;

        Self::new(1, 1, 1)
            .expect("valid jiff epoch date")
            .checked_add(Span::new().days(i64::from(days)))
            .map_err(|_| MemoryPackError::InvalidDate)
    }
}

#[inline(always)]
fn ticks_to_timestamp(ticks: i64) -> Result<Timestamp, MemoryPackError> {
    let unix_ticks = ticks
        .checked_sub(DOTNET_EPOCH_TICKS)
        .ok_or(MemoryPackError::DateTimeOutOfRange)?;
    let unix_nanos = i128::from(unix_ticks) * i128::from(NANOSECONDS_PER_TICK);

    Timestamp::from_nanosecond(unix_nanos).map_err(|_| MemoryPackError::DateTimeOutOfRange)
}
