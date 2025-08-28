
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::LORA;

/// Number of received packets
#[derive(Clone, Copy)]
pub struct RxPacketCntValue (u16);

impl ReadableRegister<LORA> for RxPacketCntValue{
    const ADDRESS: u8 = 22;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for RxPacketCntValue {
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

impl Into<u16> for RxPacketCntValue {
    fn into(self) -> u16 {
        (self.0 & 65535) >> 0
    }
}
