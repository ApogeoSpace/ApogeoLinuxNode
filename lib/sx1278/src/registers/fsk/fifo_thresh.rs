
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// When to start TX based on threshold
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum TxStartCondition {
    /// Start TX when FIFO is empty
    FifoEmpty = 0,
    /// Start TX when FIFO reaches threshold
    FifoLevel = 1,
}

/// FIFO threshold
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct FifoThresh {
 fifo_threshold: B6,
    #[skip] __: bool,
    /// When to start TX based on threshold
     #[bits = 1] pub tx_start_condition: TxStartCondition,
}

impl Register<FSK> for FifoThresh{}

impl ReadableRegister<FSK> for FifoThresh{
    const ADDRESS: u8 = 53;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for FifoThresh {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for FifoThresh{
    const ADDRESS: u8 = 53;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for FifoThresh {
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
