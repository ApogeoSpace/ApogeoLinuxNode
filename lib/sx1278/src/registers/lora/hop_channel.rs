
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::LORA;
use modular_bitfield::prelude::*;

/// Frequency hopping information
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct HopChannel {
    pub fhss_present_channel: B6,
    pub crc_on_payload: bool,
    pub pll_timeout: bool,
}

impl ReadableRegister<LORA> for HopChannel{
    const ADDRESS: u8 = 28;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for HopChannel {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}
