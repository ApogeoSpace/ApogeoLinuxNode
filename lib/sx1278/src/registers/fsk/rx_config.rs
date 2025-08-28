
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// RX trigger mechanism
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum RxTrigger {
    /// No triggering event
    None = 0,
    /// Change on rssi irq
    RSSIIrq = 1,
    /// Change on preamble detect
    PreambleDetect = 6,
    /// Cange on either RSSI IRQ or Preamble detection
    RSSIAndPreamble = 7,
}

/// RX configuration
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct RxConfig {
    /// RX trigger mechanism
     #[bits = 3] rx_trigger: RxTrigger,
 agc_auto: bool,
 afc_auto: bool,
 restart_rx_with_pll_lock: bool,
 restart_rx_without_pll_lock: bool,
 restart_rx_on_collision: bool,
}

impl Register<FSK> for RxConfig{}

impl ReadableRegister<FSK> for RxConfig{
    const ADDRESS: u8 = 13;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for RxConfig {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for RxConfig{
    const ADDRESS: u8 = 13;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for RxConfig {
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
