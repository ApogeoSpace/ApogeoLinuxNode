
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// RX timeout
#[bitfield(bits = 24)]
#[derive(Clone, Copy)]
pub struct RxTimeout {
 timeout_signal_sync: B8,
 timeout_rx_preamble: B8,
 timeout_rx_rssi: B8,
}

impl Register<FSK> for RxTimeout{}

impl ReadableRegister<FSK> for RxTimeout{
    const ADDRESS: u8 = 32;
    const SIZE: usize = 3;
}
impl<'x> TryFrom<&'x[u8]> for RxTimeout {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 3 {
            return Err(SX1278Error::ParseError)
        }
        let mut b: [u8; 3] = v.try_into().unwrap();
        b.reverse();
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for RxTimeout{
    const ADDRESS: u8 = 32;
    const SIZE: usize = 3;
}
impl TryWriteInto<[u8], usize> for RxTimeout {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 3 {
            return Err(SX1278Error::SerializationError)
        }
        let mut repr = self.into_bytes();
        repr.reverse();
        dest[..3].copy_from_slice(&repr);
        return Ok(3)
    }
}
