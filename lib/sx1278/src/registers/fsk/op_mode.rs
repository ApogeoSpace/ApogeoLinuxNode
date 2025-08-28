
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Transceiver mode control
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum Mode {
    /// Sleep mode
    Sleep = 0,
    /// Standby mode
    Stdby = 1,
    /// Frequency synthesis TX
    FsTx = 2,
    /// Transmit mode
    Tx = 3,
    /// Frequency synthesis RX
    FsRx = 4,
    /// Receive mode
    Rx = 5,
}

/// Enables access to LF test registers
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum LowFrequencyMode {
    /// Access HF test registers
    HighFrequency = 0,
    /// Access LF test registers
    LowFrequency = 1,
}

/// Select modulation scheme
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum ModulationType {
    /// Frequency Shift Keying
    Fsk = 0,
    /// On-Off Keying
    Ook = 1,
}

/// LoRa mode enable flag (read-only)
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum LongRangeMode {
    /// FSK/OOK Mode
    FskOokMode = 0,
    /// LoRa Mode
    LoRaMode = 1,
}

/// Operating mode & modulation selection
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct OpMode {
    /// Transceiver mode control
     #[bits = 3] mode: Mode,
    /// Enables access to LF test registers
     #[bits = 1] low_frequency_mode: LowFrequencyMode,
    #[skip] __: bool,
    /// Select modulation scheme
     #[bits = 2] modulation_type: ModulationType,
    /// LoRa mode enable flag (read-only)
     #[bits = 1] long_range_mode: LongRangeMode,
}

impl Register<FSK> for OpMode{}

impl ReadableRegister<FSK> for OpMode{
    const ADDRESS: u8 = 1;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for OpMode {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for OpMode{
    const ADDRESS: u8 = 1;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for OpMode {
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
