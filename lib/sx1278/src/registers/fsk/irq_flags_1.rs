
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// IRQ status flags 1
#[bitfield(bits = 16)]
#[derive(Clone, Copy)]
pub struct IrqFlags1 {
 low_bat: bool,
 crc_ok: bool,
 payload_ready: bool,
 packet_sent: bool,
 fifo_overrun: bool,
 fifo_level: bool,
 fifo_empty: bool,
 fifo_full: bool,
 sync_address_match: bool,
 preamble_detected: bool,
 timeout: bool,
 rssi: bool,
 pll_lock: bool,
 tx_ready: bool,
 rx_ready: bool,
 mode_ready: bool,
}

impl Register<FSK> for IrqFlags1{}

impl ReadableRegister<FSK> for IrqFlags1{
    const ADDRESS: u8 = 62;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for IrqFlags1 {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 2] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for IrqFlags1{
    const ADDRESS: u8 = 62;
    const SIZE: usize = 2;
}
impl TryWriteInto<[u8], usize> for IrqFlags1 {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        let repr = self.into_bytes();
        
        dest[..1].copy_from_slice(&repr);
        return Ok(1)
    }
}
