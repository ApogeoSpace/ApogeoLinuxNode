
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Timer 2 resolution
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum Timer2Res {
    /// Disabled
    Disabled = 0,
    /// 64us
    Res64us = 1,
    /// 4.1ms
    Res4ms1 = 2,
    /// 262ms
    Res262ms = 3,
}

/// Timer 1 resolution
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum Timer1Res {
    /// Disabled
    Disabled = 0,
    /// 64us
    Res64us = 1,
    /// 4.1ms
    Res4ms1 = 2,
    /// 262ms
    Res262ms = 3,
}

/// Timer resolution
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct TimerResol {
    /// Timer 2 resolution
     #[bits = 2] timer_2_res: Timer2Res,
    /// Timer 1 resolution
     #[bits = 2] timer_1_res: Timer1Res,
    #[skip] __: B4,
}

impl Register<FSK> for TimerResol{}

impl ReadableRegister<FSK> for TimerResol{
    const ADDRESS: u8 = 56;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for TimerResol {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for TimerResol{
    const ADDRESS: u8 = 56;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for TimerResol {
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
