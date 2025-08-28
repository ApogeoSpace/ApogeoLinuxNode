
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Number of 0-1 transitions
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum PreambleDetectorSize {
    /// One Byte
    Size1B = 0,
    /// Two bytes
    Size2B = 1,
    /// Three bytes
    Size3B = 2,
}

/// Preamble detection
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct PreambleDetect {
 preamble_detector_tol: B5,
    /// Number of 0-1 transitions
     #[bits = 2] preamble_detector_size: PreambleDetectorSize,
 preamble_detector: bool,
}

impl Register<FSK> for PreambleDetect{}

impl ReadableRegister<FSK> for PreambleDetect{
    const ADDRESS: u8 = 31;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for PreambleDetect {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for PreambleDetect{
    const ADDRESS: u8 = 31;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for PreambleDetect {
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
