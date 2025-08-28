
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;

/// Enables external TCXO control
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum TcxoInputValue {
    /// Internal crystal oscillator
    Off = 0,
    /// External TCXO enabled
    On = 1,
}

impl TryFrom<u8> for TcxoInputValue {
    type Error = SX1278Error;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(TcxoInputValue::Off),
            1 => Ok(TcxoInputValue::On),
            _ => Err(SX1278Error::ParseError)
        }
    }
}

/// TCXO control
#[derive(Clone, Copy)]
pub struct Tcxo (u8);

impl<MODE: SX1278Mode> Register<MODE> for Tcxo{}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for Tcxo{
    const ADDRESS: u8 = 75;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for Tcxo {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let mut buf = [0u8; 1];
        buf[..1].copy_from_slice(v);
        Ok(Self(u8::from_be_bytes(buf)))
    }
}

impl TryInto<TcxoInputValue> for Tcxo {
    type Error = SX1278Error;
    
    fn try_into(self) -> Result<TcxoInputValue, Self::Error> {
        TcxoInputValue::try_from((self.0 & 16) >> 4)
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for Tcxo{
    const ADDRESS: u8 = 75;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for Tcxo {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&(self.0 | 9).to_be_bytes()[0..1]);
        return Ok(1)
    }
}
impl From<TcxoInputValue> for Tcxo {
    fn from(v: TcxoInputValue) -> Self {
        Self(((v as u8) << 4) & 16)
    }
}
