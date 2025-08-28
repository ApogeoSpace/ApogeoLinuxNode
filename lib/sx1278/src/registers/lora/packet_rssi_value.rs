
#![allow(dead_code)]

use crate::registers::all::frequency::LowFrequencyMode;
use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::LORA;


/// RSSI value of last packet received
#[derive(Clone, Copy)]
pub struct PacketRssiValueRaw (u8);

impl PacketRssiValueRaw {
    const RSSI_OFFSET_HF: i16 = -157;
    const RSSI_OFFSET_LF: i16 = -164;

    pub fn get_rssi(&self, mode: LowFrequencyMode) -> i16 {
        match mode {
            LowFrequencyMode::LowFrequency => Self::RSSI_OFFSET_LF + (self.0 as i16),
            LowFrequencyMode::HighFrequency => Self::RSSI_OFFSET_HF + (self.0 as i16),
        }
    }
}

impl ReadableRegister<LORA> for PacketRssiValueRaw{
    const ADDRESS: u8 = 26;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for PacketRssiValueRaw {
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
