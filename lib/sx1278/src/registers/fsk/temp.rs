
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::FSK;

/// Temperature measurement result
#[derive(Clone, Copy)]
pub struct Temp (u8);

impl ReadableRegister<FSK> for Temp{
    const ADDRESS: u8 = 60;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for Temp {
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

impl Into<u8> for Temp {
    fn into(self) -> u8 {
        (self.0 & 255) >> 0
    }
}
