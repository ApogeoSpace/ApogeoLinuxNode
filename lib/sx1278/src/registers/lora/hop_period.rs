
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;

/// Frequency hopping period
#[derive(Clone, Copy)]
pub struct HopPeriod (u8);

impl Register<LORA> for HopPeriod{}

impl ReadableRegister<LORA> for HopPeriod{
    const ADDRESS: u8 = 36;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for HopPeriod {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let mut buf = [0u8; 1];
        buf[..1].copy_from_slice(v);
        Ok(Self(u8::from_be_bytes(buf)))
    }
}

impl Into<u8> for HopPeriod {
    fn into(self) -> u8 {
        (self.0 & 255) >> 0
    }
}

impl WritableRegister<LORA> for HopPeriod{
    const ADDRESS: u8 = 36;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for HopPeriod {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..1]);
        return Ok(1)
    }
}
impl From<u8> for HopPeriod {
    fn from(v: u8) -> Self {
        Self(((v as u8) << 0) & 255)
    }
}
