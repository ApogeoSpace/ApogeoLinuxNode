
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::SX1278Mode;

/// Semtech chip version
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Version (u8);

impl Version {
    pub const SX1276_VERSION: Self = Version(0x12u8);
    pub const SX1277_VERSION: Self = Version(0x12u8);
    pub const SX1278_VERSION: Self = Version(0x12u8);
    pub const SX1279_VERSION: Self = Version(0x12u8);
}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for Version{
    const ADDRESS: u8 = 66;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for Version {
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
impl Into<u8> for Version {
    fn into(self) -> u8 {
        (self.0 & 255) >> 0
    }
}
