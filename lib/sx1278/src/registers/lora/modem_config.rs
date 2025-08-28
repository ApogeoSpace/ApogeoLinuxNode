
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;
use modular_bitfield::prelude::*;

/// Spreading factor
#[derive(Specifier)]
#[bits = 4]
#[derive(Clone, Copy)]
pub enum SpreadingFactor {
    /// SF6
    Sf6 = 6,
    /// SF7
    Sf7 = 7,
    /// SF8
    Sf8 = 8,
    /// SF9
    Sf9 = 9,
    /// SF10
    Sf10 = 10,
    /// SF11
    Sf11 = 11,
    /// SF12
    Sf12 = 12,
}

/// Error coding rate
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum CodingRate {
    /// 4/5
    Cr4_5 = 1,
    /// 4/6
    Cr4_6 = 2,
    /// 4/7
    Cr4_7 = 3,
    /// 4/8
    Cr4_8 = 4,
}

/// Signal bandwidth
#[derive(Specifier)]
#[bits = 4]
#[derive(Clone, Copy)]
pub enum Bw {
    /// 7.8 kHz
    Bw7_8KHz = 0,
    /// 10.4 kHz
    Bw10_4KHz = 1,
    /// 15.6 kHz
    Bw15_6KHz = 2,
    /// 20.8 kHz
    Bw20_8KHz = 3,
    /// 31.25 kHz
    Bw31_25KHz = 4,
    /// 41.7 kHz
    Bw41_7KHz = 5,
    /// 62.5 kHz
    Bw62_5KHz = 6,
    /// 125 kHz
    Bw125KHz = 7,
    /// 250 kHz
    Bw250KHz = 8,
    /// 500 kHz
    Bw500KHz = 9,
}

impl Into<u32> for Bw {
    fn into(self) -> u32 {
        match self {
            Bw::Bw7_8KHz => 7800,
            Bw::Bw10_4KHz => 10400,
            Bw::Bw15_6KHz => 15600,
            Bw::Bw20_8KHz => 20800,
            Bw::Bw31_25KHz => 31250,
            Bw::Bw41_7KHz => 41700,
            Bw::Bw62_5KHz => 62500,
            Bw::Bw125KHz => 125000,
            Bw::Bw250KHz => 250000,
            Bw::Bw500KHz => 500000,
        }
    }
}

impl TryFrom<u8> for Bw{
    type Error = SX1278Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return Self::from_bytes(value).map_err(|_| SX1278Error::ParseError)
    }
}

/// LoRa modem configuration. In the lower band (169MHz), signal bandwidths 8&9 are not supported
#[bitfield(bits = 16)]
#[derive(Clone, Copy)]
pub struct ModemConfig {
    pub symb_timeout_msb: B2,
    pub rx_payload_crc: bool,
    pub tx_continuous_mode: bool,
    /// Spreading factor
     #[bits = 4] pub spreading_factor: SpreadingFactor,
     pub implicit_header: bool,
    /// Error coding rate
     #[bits = 3] pub coding_rate: CodingRate,
    /// Signal bandwidth
     #[bits = 4] pub bw: Bw,
}

impl Register<LORA> for ModemConfig{}

impl ReadableRegister<LORA> for ModemConfig{
    const ADDRESS: u8 = 29;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for ModemConfig {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 2 {
            return Err(SX1278Error::ParseError)
        }
        let mut b: [u8; 2] = v.try_into().unwrap();
        b.reverse();
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<LORA> for ModemConfig{
    const ADDRESS: u8 = 29;
    const SIZE: usize = 2;
}
impl TryWriteInto<[u8], usize> for ModemConfig {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 2 {
            return Err(SX1278Error::SerializationError)
        }
        let mut repr = self.into_bytes();
        repr.reverse();
        dest[..2].copy_from_slice(&repr);
        return Ok(2)
    }
}
