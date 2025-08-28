
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::LORA;
use bitflags::bitflags;
use super::modem_config::CodingRate;
use modular_bitfield::Specifier;

#[derive(Clone, Copy, Debug, Default)]
pub struct ModemStatus(u8);

bitflags! {
    impl ModemStatus: u8 {
        const SignelDetected = 1;
        const SignalSynchronized = 2;
        const RXOngoing = 4;
        const HeaderInfoValid = 8;
        const ModemClear = 16;
    }
}

#[derive(Clone, Copy)]
pub struct ModemStat {
    status: ModemStatus,
    rx_coding_rate: CodingRate,
}

impl ReadableRegister<LORA> for ModemStat{
    const ADDRESS: u8 = 24;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for ModemStat {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
       
        Ok(Self {
            status: ModemStatus::from_bits(v[0] & 31).unwrap(),
            rx_coding_rate: CodingRate::from_bytes((v[0] & 0xE0) >> 5).unwrap()
        })
    }
}
