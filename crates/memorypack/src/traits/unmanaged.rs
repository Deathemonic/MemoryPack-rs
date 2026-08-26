#[allow(clippy::missing_safety_doc)]
pub unsafe trait MemoryPackUnmanaged: Copy + Sized + 'static {}

macro_rules! impl_unmanaged {
    ($($ty:ty),+ $(,)?) => {
        $(unsafe impl MemoryPackUnmanaged for $ty {})+
    };
}

#[cfg(target_endian = "little")]
impl_unmanaged!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64);
