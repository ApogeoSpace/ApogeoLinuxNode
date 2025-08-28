
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::FSK;

/// AFC correction value
#[derive(Clone, Copy)]
pub struct AfcValue (u16);

impl ReadableRegister<FSK> for AfcValue{
    const ADDRESS: u8 = 27;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for AfcValue {
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

impl Into<u16> for AfcValue {
    fn into(self) -> u16 {
        (self.0 & 65535) >> 0
    }
}
