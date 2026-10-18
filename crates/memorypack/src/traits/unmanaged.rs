/// # Safety
/// Implementations must have no padding and accept every bit pattern. Raw
/// copying is necessary to serialize numeric collections without per-element
/// conversions. Only numeric primitives on little-endian targets implement this
/// internal trait.
pub unsafe trait MemoryPackUnmanaged: Copy + Sized + 'static {}

macro_rules! impl_unmanaged {
    ($($ty:ty),+ $(,)?) => {
        $(// SAFETY: these numeric primitives have no padding and accept every bit pattern.
        unsafe impl MemoryPackUnmanaged for $ty {})+
    };
}

#[cfg(target_endian = "little")]
impl_unmanaged!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64);
