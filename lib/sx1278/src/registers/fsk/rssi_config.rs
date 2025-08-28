
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// RSSI averaging factor
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum RssiSmoothing {
    /// 2 Samples smoothing
    Smoothing2Samples = 0,
    /// 4 Samples smoothing
    Smoothing4Samples = 1,
    /// 8 Samples smoothing
    Smoothing8Samples = 2,
    /// 16 Samples smoothing
    Smoothing16Samples = 3,
    /// 32 Samples smoothing
    Smoothing32Samples = 4,
    /// 64 Samples smoothing
    Smoothing64Samples = 5,
    /// 128 Samples smoothing
    Smoothing128Samples = 6,
    /// 256 Samples smoothing
    Smoothing256Samples = 7,
}

/// RSSI configuration
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct RssiConfig {
    /// RSSI averaging factor
     #[bits = 3] rssi_smoothing: RssiSmoothing,
 rssi_offset: B5,
}

impl Register<FSK> for RssiConfig{}

impl ReadableRegister<FSK> for RssiConfig{
    const ADDRESS: u8 = 14;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for RssiConfig {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for RssiConfig{
    const ADDRESS: u8 = 14;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for RssiConfig {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        let repr = self.into_bytes();
        
        dest[..1].copy_from_slice(&repr);
        return Ok(1)
    }
}
