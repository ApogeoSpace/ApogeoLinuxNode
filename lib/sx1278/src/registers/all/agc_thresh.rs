
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;
use modular_bitfield::prelude::*;

/// AGC threshold
#[bitfield(bits = 24)]
#[derive(Clone, Copy)]
pub struct AgcThresh {
 pub agc_step_5: B4,
 pub agc_step_4: B4,
 pub agc_step_3: B4,
 pub agc_step_2: B4,
 pub agc_step_1: B5,
    #[skip] __: B3,
}

impl<MODE: SX1278Mode> Register<MODE> for AgcThresh{}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for AgcThresh{
    const ADDRESS: u8 = 98;
    const SIZE: usize = 3;
}
impl<'x> TryFrom<&'x[u8]> for AgcThresh {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 3 {
            return Err(SX1278Error::ParseError)
        }
        let mut b: [u8; 3] = v.try_into().unwrap();
        b.reverse();
        Ok(Self::from_bytes(b))
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for AgcThresh{
    const ADDRESS: u8 = 98;
    const SIZE: usize = 3;
}
impl TryWriteInto<[u8], usize> for AgcThresh {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 3 {
            return Err(SX1278Error::SerializationError)
        }
        let mut repr = self.into_bytes();
        repr.reverse();
        dest[..3].copy_from_slice(&repr);
        return Ok(3)
    }
}
