
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;
use modular_bitfield::prelude::*;

/// Enables automatic gain control
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum AgcMode {
    /// Manual gain control
    Manual = 0,
    /// Automatic gain control
    Auto = 1,
}

/// LoRa modem configuration 3
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct ModemConfigExt {
    #[skip] __: B2,
    /// Enables automatic gain control
     #[bits = 1] pub agc_mode: AgcMode,
     pub low_data_rate_optimize: bool,
    #[skip] __: B4,
}

impl Register<LORA> for ModemConfigExt{}

impl ReadableRegister<LORA> for ModemConfigExt{
    const ADDRESS: u8 = 38;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for ModemConfigExt {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<LORA> for ModemConfigExt{
    const ADDRESS: u8 = 38;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for ModemConfigExt {
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
