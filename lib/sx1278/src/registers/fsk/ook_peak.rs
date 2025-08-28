
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Step size for threshold change
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum OokPeakThreshStep {
    /// Step Size 0.5dB
    Step0db5 = 0,
    /// Step Size 1dB
    Step1db0 = 1,
    /// Step Size 1.5dB
    Step1db5 = 2,
    /// Step Size 2dB
    Step2db0 = 3,
    /// Step Size 3dB
    Step3db0 = 4,
    /// Step Size 4dB
    Step4db0 = 5,
    /// Step Size 5dB
    Step5db0 = 6,
    /// Step Size 6dB
    Step6db0 = 7,
}

/// Type of OOK thresholding
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum OokThreshType {
    /// Fixed threshold
    Fixed = 0,
    /// Peak thresholding
    Peak = 1,
    /// Average threshold
    Average = 2,
}

/// OOK demodulator peak mode
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct OokPeak {
    /// Step size for threshold change
     #[bits = 3] ook_peak_thresh_step: OokPeakThreshStep,
    /// Type of OOK thresholding
     #[bits = 2] ook_thresh_type: OokThreshType,
 bit_sync: bool,
    #[skip] __: B2,
}

impl Register<FSK> for OokPeak{}

impl ReadableRegister<FSK> for OokPeak{
    const ADDRESS: u8 = 20;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for OokPeak {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for OokPeak{
    const ADDRESS: u8 = 20;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for OokPeak {
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
