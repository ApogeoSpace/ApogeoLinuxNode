
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::LORA;

/// Number of bytes received in the last packet
#[derive(Clone, Copy)]
pub struct RxNbBytes (u8);

impl ReadableRegister<LORA> for RxNbBytes{
    const ADDRESS: u8 = 19;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for RxNbBytes {
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

impl Into<u8> for RxNbBytes {
    fn into(self) -> u8 {
        (self.0 & 255) >> 0
    }
}
