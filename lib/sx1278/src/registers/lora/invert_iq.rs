
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;
use modular_bitfield::prelude::*;

/// IQ signal inversion
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct InvertIQ {
    pub invert_iqtx: bool,
    #[skip] __: B5,
    pub invert_iqrx: bool,
    #[skip] __: bool,
}

impl Register<LORA> for InvertIQ{}

impl ReadableRegister<LORA> for InvertIQ{
    const ADDRESS: u8 = 51;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for InvertIQ {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<LORA> for InvertIQ{
    const ADDRESS: u8 = 51;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for InvertIQ {
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
