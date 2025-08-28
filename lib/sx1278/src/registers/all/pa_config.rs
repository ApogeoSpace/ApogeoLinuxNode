
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;
use modular_bitfield::prelude::*;

/// Selects PA output pin
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum PaSelect {
    /// RFO pin. Output power limited to +14 dBm
    RfoPin = 0,
    /// PA_BOOST pin. Output power limited to +20 dBm
    PaBoostPin = 1,
}

/// PA configuration
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct PaConfig {
    pub output_power: B4,
    pub max_power: B3,
    /// Selects PA output pin
     #[bits = 1] pub pa_select: PaSelect,
}

impl<MODE: SX1278Mode> Register<MODE> for PaConfig{}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for PaConfig{
    const ADDRESS: u8 = 9;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for PaConfig {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for PaConfig{
    const ADDRESS: u8 = 9;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for PaConfig {
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
