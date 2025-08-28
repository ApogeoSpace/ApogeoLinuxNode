
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;

/// Preamble length
#[derive(Clone, Copy)]
pub struct Preamble (u16);

impl Register<FSK> for Preamble{}

impl ReadableRegister<FSK> for Preamble{
    const ADDRESS: u8 = 37;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for Preamble {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 2 {
            return Err(SX1278Error::ParseError)
        }
        let mut buf = [0u8; 2];
        buf[..2].copy_from_slice(v);
        Ok(Self(u16::from_be_bytes(buf)))
    }
}

impl Into<u16> for Preamble {
    fn into(self) -> u16 {
        (self.0 & 65535) >> 0
    }
}

impl WritableRegister<FSK> for Preamble{
    const ADDRESS: u8 = 37;
    const SIZE: usize = 2;
}
impl TryWriteInto<[u8], usize> for Preamble {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 2 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..2]);
        return Ok(2)
    }
}
impl From<u16> for Preamble {
    fn from(v: u16) -> Self {
        Self(((v as u16) << 0) & 65535)
    }
}
