
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Mantissa of RX bandwidth
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum RxBwMant {
    /// Mantissa 16
    Mant16 = 0,
    /// Mantissa 20
    Mant20 = 1,
    /// Mantissa 24
    Mant24 = 2,
}

/// Channel filter bandwidth control
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct RxBw {
 rx_bw_exp: B3,
    /// Mantissa of RX bandwidth
     #[bits = 2] rx_bw_mant: RxBwMant,
    #[skip] __: B3,
}

impl Register<FSK> for RxBw{}

impl ReadableRegister<FSK> for RxBw{
    const ADDRESS: u8 = 18;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for RxBw {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for RxBw{
    const ADDRESS: u8 = 18;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for RxBw {
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
