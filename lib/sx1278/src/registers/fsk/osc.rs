
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Selects CLKOUT frequency
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum ClkOut {
    /// Clockout frequency is FXOSC / 1
    Div1 = 0,
    /// Clockout frequency is FXOSC / 2
    Div2 = 1,
    /// Clockout frequency is FXOSC / 4
    Div4 = 2,
    /// Clockout frequency is FXOSC / 8
    Div8 = 3,
    /// Clockout frequency is FXOSC / 16
    Div16 = 4,
    /// Clockout frequency is FXOSC / 32
    Div32 = 5,
    /// Clockout frequency is RC clock
    Rc = 6,
    /// Clockout is Off
    Off = 7,
}

/// Oscillator tuning
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct Osc {
    /// Selects CLKOUT frequency
     #[bits = 3] clk_out: ClkOut,
 rc_cal_start: bool,
    #[skip] __: B4,
}

impl Register<FSK> for Osc{}

impl ReadableRegister<FSK> for Osc{
    const ADDRESS: u8 = 36;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for Osc {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for Osc{
    const ADDRESS: u8 = 36;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for Osc {
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
