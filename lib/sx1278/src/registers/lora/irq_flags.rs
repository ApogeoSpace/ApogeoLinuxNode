
#![allow(dead_code)]

use crate::registers::{ReadableRegister, Register, TryWriteInto, WritableRegister};
use crate::error::SX1278Error;
use crate::LORA;
use bitflags::bitflags;


/// IRQ flags
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IrqFlags(u8);

bitflags! {
    impl IrqFlags: u8 {
        const CadDetected = 1;
        const FHSSChannelChange = 2;
        const CadDone = 4;
        const TXDone = 8;
        const HeaderValid = 16;
        const PayloadCRCError = 32;
        const RXDone = 64;
        const RXTimeout = 128;
    }
}


impl Register<LORA> for IrqFlags {}

impl ReadableRegister<LORA> for IrqFlags{
    const ADDRESS: u8 = 18;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for IrqFlags {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        Ok(Self::from_bits(v[0]).unwrap()) // We have all the bits!
    }
}


impl WritableRegister<LORA> for IrqFlags{
    const ADDRESS: u8 = 18;
    const SIZE: usize = 1;
}

impl TryWriteInto<[u8], usize> for IrqFlags {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest[0] = self.bits();
        return Ok(1)
    }
}
