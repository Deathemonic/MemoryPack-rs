use std::borrow::Cow;

#[cfg(feature = "jiff")]
use jiff::civil::{Date, DateTime, Time};

use super::{MemoryPackArena, MemoryPackDeserializeIn};
use crate::{MemoryPackDeserialize, MemoryPackError, MemoryPackReader};

macro_rules! deserialize_owned_in {
    ($($value_type:ty),+ $(,)?) => {
        $(
            impl<'arena, Arena: MemoryPackArena + ?Sized> MemoryPackDeserializeIn<'arena, Arena> for $value_type {
                #[inline]
                fn deserialize_in(
                    reader: &mut MemoryPackReader<'_>,
                    _arena: &'arena Arena
                ) -> Result<Self, MemoryPackError> {
                    <Self as MemoryPackDeserialize<'_>>::deserialize(reader)
                }

                #[inline]
                fn deserialize_nullable_in(
                    reader: &mut MemoryPackReader<'_>,
                    _arena: &'arena Arena
                ) -> Result<Option<Self>, MemoryPackError> {
                    <Self as MemoryPackDeserialize<'_>>::deserialize_nullable(reader)
                }
            }
        )+
    };
}

deserialize_owned_in!(
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    f32,
    f64,
    char,
    String,
    Cow<'_, str>
);

#[cfg(feature = "uuid")]
deserialize_owned_in!(uuid::Uuid);

#[cfg(feature = "rust_decimal")]
deserialize_owned_in!(rust_decimal::Decimal);

#[cfg(feature = "half")]
deserialize_owned_in!(half::f16);

#[cfg(feature = "num-bigint")]
deserialize_owned_in!(num_bigint::BigInt, num_bigint::BigUint);

#[cfg(feature = "url")]
deserialize_owned_in!(url::Url);

#[cfg(feature = "jiff")]
deserialize_owned_in!(jiff::SignedDuration, jiff::Timestamp, jiff::Zoned, DateTime, Date, Time);

#[cfg(feature = "num-complex")]
deserialize_owned_in!(num_complex::Complex<f64>);

#[cfg(feature = "glam")]
deserialize_owned_in!(glam::Vec2, glam::Vec3, glam::Vec4, glam::Quat, glam::Mat3A, glam::Mat4);
