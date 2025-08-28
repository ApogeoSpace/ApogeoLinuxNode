
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;

/// Pointer to current FIFO read/write address
#[derive(Clone, Copy)]
pub struct FifoAddrPtr (u8);

impl Register<LORA> for FifoAddrPtr{}

impl ReadableRegister<LORA> for FifoAddrPtr{
    const ADDRESS: u8 = 13;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for FifoAddrPtr {
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

impl Into<u8> for FifoAddrPtr {
    fn into(self) -> u8 {
        (self.0 & 255) >> 0
    }
}

impl WritableRegister<LORA> for FifoAddrPtr{
    const ADDRESS: u8 = 13;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for FifoAddrPtr {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..1]);
        return Ok(1)
    }
}
impl From<u8> for FifoAddrPtr {
    fn from(v: u8) -> Self {
        Self(((v as u8) << 0) & 255)
    }
}
