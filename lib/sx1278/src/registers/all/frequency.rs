
#![allow(dead_code)]

use modular_bitfield::Specifier;

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;

#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy, Debug)]
pub enum LowFrequencyMode {
    /// High Frequency test registers enabled
    HighFrequency = 0,
    /// Low Frequency test registers enabled
    LowFrequency = 1,
}

/// RF carrier frequency setting. F_{rf} = F_{xtal} * REG / 2^19
#[derive(Clone, Copy, Debug)]
pub struct Frequency (u32);

impl<MODE: SX1278Mode> Register<MODE> for Frequency{}

impl Frequency {
    pub fn get_value(&self, xtal: u32) -> f64 {
        return ((xtal >> 19) as f64) * (self.0 as f64)
    }

    pub fn steps(freq: f64, xtal: u32) -> Self {
        Self((((1u32 << 19) as f64) / (xtal as f64) * freq) as u32)
    }

    pub fn frequency_mode(self, xtal: u32) -> Option<LowFrequencyMode> {
        let f = self.get_value(xtal);
        if f <= 525e6 {
            return Some(LowFrequencyMode::LowFrequency)
        }
        if f >= 779e6 {
            return Some(LowFrequencyMode::HighFrequency)
        }
        return None
    }
}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for Frequency{
    const ADDRESS: u8 = 6;
    const SIZE: usize = 3;
}
impl<'x> TryFrom<&'x[u8]> for Frequency {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 3 {
            return Err(SX1278Error::ParseError)
        }
        let mut buf = [0u8; 4];
        buf[..3].copy_from_slice(v);
        Ok(Self(u32::from_be_bytes(buf)))
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for Frequency{
    const ADDRESS: u8 = 6;
    const SIZE: usize = 3;
}
impl TryWriteInto<[u8], usize> for Frequency {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 3 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[1..]);
        return Ok(3)
    }
}
