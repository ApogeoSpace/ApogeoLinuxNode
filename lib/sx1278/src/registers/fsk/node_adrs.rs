
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;

/// Node address
#[derive(Clone, Copy)]
pub struct NodeAdrs (u8);

impl Register<FSK> for NodeAdrs{}

impl ReadableRegister<FSK> for NodeAdrs{
    const ADDRESS: u8 = 51;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for NodeAdrs {
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

impl Into<u8> for NodeAdrs {
    fn into(self) -> u8 {
        (self.0 & 255) >> 0
    }
}

impl WritableRegister<FSK> for NodeAdrs{
    const ADDRESS: u8 = 51;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for NodeAdrs {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..1]);
        return Ok(1)
    }
}
impl From<u8> for NodeAdrs {
    fn from(v: u8) -> Self {
        Self(((v as u8) << 0) & 255)
    }
}
