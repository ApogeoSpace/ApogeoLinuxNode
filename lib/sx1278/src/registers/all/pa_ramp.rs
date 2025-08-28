
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;

/// PA ramp-up time
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PaRampValue {
    /// 3.4 us
    Ramp3_4us = 0,
    /// 2 us
    Ramp2us = 1,
    /// 1 ms
    Ramp1ms = 2,
    /// 500 us
    Ramp500us = 3,
    /// 250 us
    Ramp250us = 4,
    /// 125 us
    Ramp125us = 5,
    /// 100 us
    Ramp100us = 6,
    /// 62 us
    Ramp62us = 7,
    /// 50 us
    Ramp50us = 8,
    /// 40 us
    Ramp40us = 9,
    /// 31 us
    Ramp31us = 10,
    /// 25 us
    Ramp25us = 11,
    /// 20 us
    Ramp20us = 12,
    /// 15 us
    Ramp15us = 13,
    /// 12 us
    Ramp12us = 14,
    /// 10 us
    Ramp10us = 15,
}

impl TryFrom<u8> for PaRampValue {
    type Error = SX1278Error;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(PaRampValue::Ramp3_4us),
            1 => Ok(PaRampValue::Ramp2us),
            2 => Ok(PaRampValue::Ramp1ms),
            3 => Ok(PaRampValue::Ramp500us),
            4 => Ok(PaRampValue::Ramp250us),
            5 => Ok(PaRampValue::Ramp125us),
            6 => Ok(PaRampValue::Ramp100us),
            7 => Ok(PaRampValue::Ramp62us),
            8 => Ok(PaRampValue::Ramp50us),
            9 => Ok(PaRampValue::Ramp40us),
            10 => Ok(PaRampValue::Ramp31us),
            11 => Ok(PaRampValue::Ramp25us),
            12 => Ok(PaRampValue::Ramp20us),
            13 => Ok(PaRampValue::Ramp15us),
            14 => Ok(PaRampValue::Ramp12us),
            15 => Ok(PaRampValue::Ramp10us),
            _ => Err(SX1278Error::ParseError)
        }
    }
}

/// PA ramp-up timeout
#[derive(Clone, Copy)]
pub struct PaRamp (u8);

impl<MODE: SX1278Mode> Register<MODE> for PaRamp{}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for PaRamp{
    const ADDRESS: u8 = 10;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for PaRamp {
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

impl TryInto<PaRampValue> for PaRamp {
    type Error = SX1278Error;
    
    fn try_into(self) -> Result<PaRampValue, Self::Error> {
        PaRampValue::try_from((self.0 & 15) >> 0)
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for PaRamp{
    const ADDRESS: u8 = 10;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for PaRamp {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..1]);
        return Ok(1)
    }
}
impl From<PaRampValue> for PaRamp {
    fn from(v: PaRampValue) -> Self {
        Self(((v as u8) << 0) & 15)
    }
}
