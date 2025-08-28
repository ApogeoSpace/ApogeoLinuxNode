
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// AFC/FEI control
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct AfcFei {
    pub afc_auto_clear: bool,
    pub afc_clear: bool,
    #[skip] __: B2,
    pub agc_start: bool,
    #[skip] __: B3,
}

impl Register<FSK> for AfcFei{}

impl ReadableRegister<FSK> for AfcFei{
    const ADDRESS: u8 = 26;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for AfcFei {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for AfcFei{
    const ADDRESS: u8 = 26;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for AfcFei {
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
