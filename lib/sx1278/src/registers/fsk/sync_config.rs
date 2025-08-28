
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Enables sync word detection
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum PreamblePolarity {
    /// AA Polarity
    PolAA = 0,
    /// 55 Polarity
    Pol55 = 1,
}

/// Controls the automatic restart of the receiver after the reception of a valid packet
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum AutoRestartRxMode {
    /// Do not restart
    Off = 0,
    /// Restart without PLL lock waiting
    NoPllLock = 1,
    /// Restart waiting pll relock
    PllLock = 2,
}

/// Sync word configuration
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct SyncConfig {
 sync_size: B3,
    #[skip] __: bool,
 sync: bool,
    /// Enables sync word detection
     #[bits = 1] preamble_polarity: PreamblePolarity,
    /// Controls the automatic restart of the receiver after the reception of a valid packet
     #[bits = 2] auto_restart_rx_mode: AutoRestartRxMode,
}

impl Register<FSK> for SyncConfig{}

impl ReadableRegister<FSK> for SyncConfig{
    const ADDRESS: u8 = 39;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for SyncConfig {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for SyncConfig{
    const ADDRESS: u8 = 39;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for SyncConfig {
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
