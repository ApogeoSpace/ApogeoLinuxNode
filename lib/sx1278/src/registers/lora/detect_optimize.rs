
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;
use modular_bitfield::prelude::*;

/// Adjusts detection algorithm
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum DetectionOptimize {
    /// Default setting
    Default = 0,
    /// Required setting for SF6 mode
    OptimizeSf6 = 5,
}

/// Detection optimization settings (used for SF6)
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct DetectOptimize {
    /// Adjusts detection algorithm
     #[bits = 3] pub detection_optimize: DetectionOptimize,
    #[skip] __: B4,
 automatic_if: bool,
}

impl Register<LORA> for DetectOptimize{}

impl ReadableRegister<LORA> for DetectOptimize{
    const ADDRESS: u8 = 49;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for DetectOptimize {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<LORA> for DetectOptimize{
    const ADDRESS: u8 = 49;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for DetectOptimize {
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
