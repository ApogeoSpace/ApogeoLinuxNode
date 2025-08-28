
#![allow(dead_code)]

use crate::registers::all::frequency::LowFrequencyMode;
use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;
use modular_bitfield::prelude::*;

/// Device operating mode selection
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Lowest power mode
    Sleep = 0,
    /// Crystal oscillator and baseband active
    Standby = 1,
    /// Frequency synthesis TX
    Fstx = 2,
    /// Transmit
    Tx = 3,
    /// Frequency synthesis RX
    Fsrx = 4,
    /// Continuous reception
    RxContinuous = 5,
    /// Single packet reception
    RxSingle = 6,
    /// Channel activity detection
    Cad = 7,
}


/// LoRa mode enable. Must be modified only in Sleep mode
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy, Debug)]
pub enum LongRangeMode {
    /// This selects FSK/OOK modulation mode
    FskOokMode = 0,
    /// This selects LoRa modulation mode
    LoRaMode = 1,
}

/// Operating mode & LoRa / FSK selection
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct OpMode {
    /// Device operating mode selection
     #[bits = 3] pub mode: Mode,
    /// Access Low Frequency Mode test registers
     #[bits = 1] pub low_frequency_mode: LowFrequencyMode,
    #[skip] __: B3,
    /// LoRa mode enable. Must be modified only in Sleep mode
     #[bits = 1] pub long_range_mode: LongRangeMode,
}

impl Register<LORA> for OpMode{}

impl ReadableRegister<LORA> for OpMode{
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

impl WritableRegister<LORA> for OpMode{
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
