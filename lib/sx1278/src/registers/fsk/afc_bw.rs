
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Mantissa of AFC bandwidth
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum AfcBwMant {
    /// Mantissa 16
    Mant16 = 0,
    /// Mantissa 20
    Mant20 = 1,
    /// Mantissa 24
    Mant24 = 2,
}

/// AFC bandwidth
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct AfcBw {
 afc_bw_exp: B3,
    /// Mantissa of AFC bandwidth
     #[bits = 2] pub afc_bw_mant: AfcBwMant,
    #[skip] __: B3,
}

impl Register<FSK> for AfcBw{}

impl ReadableRegister<FSK> for AfcBw{
    const ADDRESS: u8 = 19;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for AfcBw {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for AfcBw{
    const ADDRESS: u8 = 19;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for AfcBw {
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
