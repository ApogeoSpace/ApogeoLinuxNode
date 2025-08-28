
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;

/// Sets PLL loop bandwidth
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PllBandwidthValue {
    /// 75 kHz
    Bw75kHz = 0,
    /// 150 kHz
    Bw150kHz = 1,
    /// 225 kHz
    Bw225kHz = 2,
    /// 300 kHz
    Bw300kHz = 3,
}

impl TryFrom<u8> for PllBandwidthValue {
    type Error = SX1278Error;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(PllBandwidthValue::Bw75kHz),
            1 => Ok(PllBandwidthValue::Bw150kHz),
            2 => Ok(PllBandwidthValue::Bw225kHz),
            3 => Ok(PllBandwidthValue::Bw300kHz),
            _ => Err(SX1278Error::ParseError)
        }
    }
}

/// PLL bandwidth setting
#[derive(Clone, Copy)]
pub struct Pll (u8);

impl<MODE: SX1278Mode> Register<MODE> for Pll{}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for Pll{
    const ADDRESS: u8 = 112;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for Pll {
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

impl TryInto<PllBandwidthValue> for Pll {
    type Error = SX1278Error;
    
    fn try_into(self) -> Result<PllBandwidthValue, Self::Error> {
        PllBandwidthValue::try_from((self.0 & 192) >> 6)
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for Pll{
    const ADDRESS: u8 = 112;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for Pll {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..1]);
        return Ok(1)
    }
}
impl From<PllBandwidthValue> for Pll {
    fn from(v: PllBandwidthValue) -> Self {
        Self(((v as u8) << 6) & 192)
    }
}
