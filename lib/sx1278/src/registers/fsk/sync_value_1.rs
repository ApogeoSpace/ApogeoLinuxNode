
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;

/// Sync word values
#[derive(Clone, Copy)]
pub struct SyncValue1 (u64);

impl Register<FSK> for SyncValue1{}

impl ReadableRegister<FSK> for SyncValue1{
    const ADDRESS: u8 = 40;
    const SIZE: usize = 8;
}
impl<'x> TryFrom<&'x[u8]> for SyncValue1 {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 8 {
            return Err(SX1278Error::ParseError)
        }
        let mut buf = [0u8; 8];
        buf[..8].copy_from_slice(v);
        Ok(Self(u64::from_be_bytes(buf)))
    }
}

impl Into<u64> for SyncValue1 {
    fn into(self) -> u64 {
        (self.0 & 18446744073709551615) >> 0
    }
}

impl WritableRegister<FSK> for SyncValue1{
    const ADDRESS: u8 = 40;
    const SIZE: usize = 8;
}
impl TryWriteInto<[u8], usize> for SyncValue1 {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 8 {
            return Err(SX1278Error::SerializationError)
        }
        dest.copy_from_slice(&self.0.to_be_bytes()[0..8]);
        return Ok(8)
    }
}
impl From<u64> for SyncValue1 {
    fn from(v: u64) -> Self {
        Self(((v as u64) << 0) & 18446744073709551615)
    }
}
