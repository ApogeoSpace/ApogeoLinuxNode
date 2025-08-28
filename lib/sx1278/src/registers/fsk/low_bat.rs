
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Sets low battery threshold
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum LowBatTrim {
    /// 1.695V
    V1695 = 0,
    /// 1.764V
    V1764 = 1,
    /// 1.835V
    V1835 = 2,
    /// 1.905V
    V1905 = 3,
    /// 1.976V
    V1976 = 4,
    /// 2.045V
    V2045 = 5,
    /// 2.116V
    V2116 = 6,
    /// 2.185V
    V2185 = 7,
}

/// Low battery detection
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct LowBat {
    /// Sets low battery threshold
     #[bits = 3] low_bat_trim: LowBatTrim,
 low_bat: bool,
    #[skip] __: B4,
}

impl Register<FSK> for LowBat{}

impl ReadableRegister<FSK> for LowBat{
    const ADDRESS: u8 = 61;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for LowBat {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for LowBat{
    const ADDRESS: u8 = 61;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for LowBat {
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
