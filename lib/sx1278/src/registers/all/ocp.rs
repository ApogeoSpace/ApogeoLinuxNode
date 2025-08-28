
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;
use modular_bitfield::prelude::*;

/// Enables over current protection
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum OcpStatus {
    /// Over current protection disabled
    Disabled = 0,
    /// Over current protection enabled
    Enabled = 1,
}

/// Over current protection control
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct Ocp {
 ocp_trim: B5,
    /// Enables over current protection
     #[bits = 1] pub ocp_status: OcpStatus,
    #[skip] __: B2,
}

impl<MODE: SX1278Mode> Register<MODE> for Ocp{}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for Ocp{
    const ADDRESS: u8 = 11;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for Ocp {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for Ocp{
    const ADDRESS: u8 = 11;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for Ocp {
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
