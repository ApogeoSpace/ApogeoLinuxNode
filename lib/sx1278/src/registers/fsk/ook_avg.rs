
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Averaging filter coefficient
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum OokAvgThreshFilt {
    /// Fc = chipRate / 32pi
    ChipRateOver32Pi = 0,
    /// Fc = chipRate / 8pi
    ChipRateOver8Pi = 1,
    /// Fc = chipRate / 4pi
    ChipRateOver4Pi = 2,
    /// Fc = chipRate / 2pi
    ChipRateOver2Pi = 3,
}

/// Offset applied to average
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum OokAvgOffset {
    /// 0dB offset
    Offset0dB = 0,
    /// 2dB Offset
    Offset2dB = 1,
    /// 4dB Offset
    Offset4dB = 2,
    /// 6dB Offset
    Offset6dB = 3,
}

/// Period of decrement of the RSSI threshold in the OOK
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum OokPeakThreshDec {
    /// Once per chip
    Once = 0,
    /// Once every two chip
    OnceTwo = 1,
    /// Once every four chip
    OnceFour = 2,
    /// Once every eight chip
    OnceEight = 3,
    /// Twice per chip
    Twice = 4,
    /// Four times per chip
    FourTimes = 5,
    /// Eight times per chip
    EightTimes = 6,
    /// Sixteen times per chip
    Sixteen = 7,
}

/// OOK average thresholding
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct OokAvg {
    /// Averaging filter coefficient
     #[bits = 2] ook_avg_thresh_filt: OokAvgThreshFilt,
    /// Offset applied to average
     #[bits = 2] ook_avg_offset: OokAvgOffset,
    #[skip] __: bool,
    /// Period of decrement of the RSSI threshold in the OOK
     #[bits = 3] ook_peak_thresh_dec: OokPeakThreshDec,
}

impl Register<FSK> for OokAvg{}

impl ReadableRegister<FSK> for OokAvg{
    const ADDRESS: u8 = 22;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for OokAvg {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for OokAvg{
    const ADDRESS: u8 = 22;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for OokAvg {
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
