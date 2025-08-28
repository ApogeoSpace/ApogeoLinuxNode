
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;

/// Adjusts threshold for preamble detection
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DetectionThresholdValue {
    /// Default threshold
    Default = 10,
    /// Recommended value for SF6 mode
    Sf6Threshold = 12,
}

impl TryFrom<u8> for DetectionThresholdValue {
    type Error = SX1278Error;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            10 => Ok(DetectionThresholdValue::Default),
            12 => Ok(DetectionThresholdValue::Sf6Threshold),
            _ => Err(SX1278Error::ParseError)
        }
    }
}

/// Detection threshold for SF6
#[derive(Clone, Copy)]
pub struct DetectionThreshold (u8);

impl Register<LORA> for DetectionThreshold{}

impl ReadableRegister<LORA> for DetectionThreshold{
    const ADDRESS: u8 = 55;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for DetectionThreshold {
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

impl TryInto<DetectionThresholdValue> for DetectionThreshold {
    type Error = SX1278Error;
    
    fn try_into(self) -> Result<DetectionThresholdValue, Self::Error> {
        DetectionThresholdValue::try_from((self.0 & 255) >> 0)
    }
}

impl WritableRegister<LORA> for DetectionThreshold{
    const ADDRESS: u8 = 55;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for DetectionThreshold {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..1]);
        return Ok(1)
    }
}
impl From<DetectionThresholdValue> for DetectionThreshold {
    fn from(v: DetectionThresholdValue) -> Self {
        Self(((v as u8) << 0) & 255)
    }
}
