use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

#[cfg(feature = "uuid")]
impl MemoryPackSerialize for uuid::Uuid {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.buffer.extend_from_slice(self.as_bytes());
        Ok(())
    }
}

#[cfg(feature = "uuid")]
impl MemoryPackDeserialize for uuid::Uuid {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(uuid::Uuid::from_bytes(reader.read_fixed_bytes::<16>()?))
    }
}

#[cfg(feature = "rust_decimal")]
impl MemoryPackSerialize for rust_decimal::Decimal {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.buffer.extend_from_slice(&self.serialize());
        Ok(())
    }
}

#[cfg(feature = "rust_decimal")]
impl MemoryPackDeserialize for rust_decimal::Decimal {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(rust_decimal::Decimal::deserialize(reader.read_fixed_bytes::<16>()?))
    }
}

#[cfg(feature = "half")]
impl MemoryPackSerialize for half::f16 {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_u16(self.to_bits())
    }
}

#[cfg(feature = "half")]
impl MemoryPackDeserialize for half::f16 {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(half::f16::from_bits(reader.read_u16()?))
    }
}

#[cfg(feature = "num-bigint")]
#[inline]
fn twos_complement_invert(bytes: &mut [u8]) {
    let mut carry = true;
    for byte in bytes.iter_mut() {
        *byte = !*byte;
        if carry {
            let (new_byte, new_carry) = byte.overflowing_add(1);
            *byte = new_byte;
            carry = new_carry;
        }
    }
}

#[cfg(feature = "num-bigint")]
impl MemoryPackSerialize for num_bigint::BigInt {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let (sign, mut bytes) = self.to_bytes_le();
        
        if sign == num_bigint::Sign::Minus {
            twos_complement_invert(&mut bytes);
        }
        
        writer.write_i32(bytes.len() as i32)?;
        writer.buffer.extend_from_slice(&bytes);
        Ok(())
    }
}

#[cfg(feature = "num-bigint")]
impl MemoryPackDeserialize for num_bigint::BigInt {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let len = reader.read_i32()?;
        if len < 0 {
            return Err(MemoryPackError::DeserializationError(
                "Negative length in BigInteger".to_string(),
            ));
        }
        
        let mut bytes = reader.read_bytes_vec(len as usize)?;
        let is_negative = bytes.last().map_or(false, |&b| b & 0x80 != 0);
        
        if is_negative {
            twos_complement_invert(&mut bytes);
            Ok(num_bigint::BigInt::from_bytes_le(num_bigint::Sign::Minus, &bytes))
        } else {
            Ok(num_bigint::BigInt::from_bytes_le(num_bigint::Sign::Plus, &bytes))
        }
    }
}

#[cfg(feature = "num-bigint")]
impl MemoryPackSerialize for num_bigint::BigUint {
    #[inline]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let bytes = self.to_bytes_le();
        writer.write_i32(bytes.len() as i32)?;
        writer.buffer.extend_from_slice(&bytes);
        Ok(())
    }
}

#[cfg(feature = "num-bigint")]
impl MemoryPackDeserialize for num_bigint::BigUint {
    #[inline]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let len = reader.read_i32()?;
        if len < 0 {
            return Err(MemoryPackError::DeserializationError(
                "Negative length in BigUint".to_string(),
            ));
        }
        
        Ok(num_bigint::BigUint::from_bytes_le(&reader.read_bytes_vec(len as usize)?))
    }
}
