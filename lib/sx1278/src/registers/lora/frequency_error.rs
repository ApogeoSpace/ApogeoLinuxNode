
#![allow(dead_code)]

use crate::registers::ReadableRegister;
use crate::error::SX1278Error;
use crate::LORA;

use super::modem_config::Bw;

/// Frequency error indicator
#[derive(Clone, Copy)]
pub struct FrequencyError (u32);

impl FrequencyError {
    pub fn get_value(&self, xtal: u32, bw: Bw) -> f32 {
        let bw: u32 = bw.into();
        ((self.0 as f64) * ((1 << 24) as f64 / xtal as f64) * (bw as f64 / 500000f64)) as f32
    }
}

impl ReadableRegister<LORA> for FrequencyError{
    const ADDRESS: u8 = 40;
    const SIZE: usize = 3;
}
impl<'x> TryFrom<&'x[u8]> for FrequencyError {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 3 {
            return Err(SX1278Error::ParseError)
        }
        let mut buf = [0u8; 4];
        buf[1..].copy_from_slice(v);
        Ok(Self(u32::from_be_bytes(buf)))
    }
}

