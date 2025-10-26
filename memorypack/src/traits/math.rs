use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

#[cfg(feature = "num-complex")]
impl MemoryPackSerialize for num_complex::Complex<f64> {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_f64(self.re)?;
        writer.write_f64(self.im)
    }
}

#[cfg(feature = "num-complex")]
impl MemoryPackDeserialize for num_complex::Complex<f64> {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let re = reader.read_f64()?;
        let im = reader.read_f64()?;
        Ok(num_complex::Complex::new(re, im))
    }
}

#[cfg(feature = "glam")]
impl MemoryPackSerialize for glam::Vec2 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let arr = self.to_array();
        writer.write_f32(arr[0])?;
        writer.write_f32(arr[1])
    }
}

#[cfg(feature = "glam")]
impl MemoryPackDeserialize for glam::Vec2 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(glam::Vec2::from_array([
            reader.read_f32()?,
            reader.read_f32()?,
        ]))
    }
}

#[cfg(feature = "glam")]
impl MemoryPackSerialize for glam::Vec3 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let arr = self.to_array();
        writer.write_f32(arr[0])?;
        writer.write_f32(arr[1])?;
        writer.write_f32(arr[2])
    }
}

#[cfg(feature = "glam")]
impl MemoryPackDeserialize for glam::Vec3 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(glam::Vec3::from_array([
            reader.read_f32()?,
            reader.read_f32()?,
            reader.read_f32()?,
        ]))
    }
}

#[cfg(feature = "glam")]
impl MemoryPackSerialize for glam::Vec4 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let arr = self.to_array();
        writer.write_f32(arr[0])?;
        writer.write_f32(arr[1])?;
        writer.write_f32(arr[2])?;
        writer.write_f32(arr[3])
    }
}

#[cfg(feature = "glam")]
impl MemoryPackDeserialize for glam::Vec4 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(glam::Vec4::from_array([
            reader.read_f32()?,
            reader.read_f32()?,
            reader.read_f32()?,
            reader.read_f32()?,
        ]))
    }
}

#[cfg(feature = "glam")]
impl MemoryPackSerialize for glam::Quat {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let arr = self.to_array();
        writer.write_f32(arr[0])?;
        writer.write_f32(arr[1])?;
        writer.write_f32(arr[2])?;
        writer.write_f32(arr[3])
    }
}

#[cfg(feature = "glam")]
impl MemoryPackDeserialize for glam::Quat {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(glam::Quat::from_array([
            reader.read_f32()?,
            reader.read_f32()?,
            reader.read_f32()?,
            reader.read_f32()?,
        ]))
    }
}

#[cfg(feature = "glam")]
impl MemoryPackSerialize for glam::Mat3A {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let cols = self.to_cols_array();
        writer.write_f32(cols[0])?;
        writer.write_f32(cols[3])?;
        writer.write_f32(cols[1])?;
        writer.write_f32(cols[4])?;
        writer.write_f32(cols[2])?;
        writer.write_f32(cols[5])
    }
}

#[cfg(feature = "glam")]
impl MemoryPackDeserialize for glam::Mat3A {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let m11 = reader.read_f32()?;
        let m12 = reader.read_f32()?;
        let m21 = reader.read_f32()?;
        let m22 = reader.read_f32()?;
        let m31 = reader.read_f32()?;
        let m32 = reader.read_f32()?;
        Ok(glam::Mat3A::from_cols(
            glam::Vec3A::new(m11, m12, 0.0),
            glam::Vec3A::new(m21, m22, 0.0),
            glam::Vec3A::new(m31, m32, 1.0),
        ))
    }
}

#[cfg(feature = "glam")]
impl MemoryPackSerialize for glam::Mat4 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let arr = self.to_cols_array();
        for &val in &arr {
            writer.write_f32(val)?;
        }
        Ok(())
    }
}

#[cfg(feature = "glam")]
impl MemoryPackDeserialize for glam::Mat4 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let mut arr = [0.0f32; 16];
        for val in &mut arr {
            *val = reader.read_f32()?;
        }
        Ok(glam::Mat4::from_cols_array(&arr))
    }
}
