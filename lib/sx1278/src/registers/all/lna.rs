
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::SX1278Mode;
use modular_bitfield::prelude::*;

/// Low Frequency LNA boost
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum LnaBoostLf {
    /// Default gain
    Default = 0,
}

/// Sets gain of LNA
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum LnaGain {
    /// Maximum gain
    G1 = 0,
    /// G2
    G2 = 1,
    /// G3
    G3 = 2,
    /// G3
    G4 = 3,
    /// G5
    G5 = 4,
    /// Minimum gain
    G6 = 5,
}

/// LNA configuration
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct Lna {
 lna_boost_hf: B2,
    #[skip] __: bool,
    /// Low Frequency LNA boost
     #[bits = 2] pub lna_boost_lf: LnaBoostLf,
    /// Sets gain of LNA
     #[bits = 3] pub lna_gain: LnaGain,
}

impl<MODE: SX1278Mode> Register<MODE> for Lna{}

impl<MODE: SX1278Mode> ReadableRegister<MODE> for Lna{
    const ADDRESS: u8 = 12;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for Lna {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl<MODE: SX1278Mode> WritableRegister<MODE> for Lna{
    const ADDRESS: u8 = 12;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for Lna {
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
