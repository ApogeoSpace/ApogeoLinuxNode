
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;

use super::irq_flags::IrqFlags;

/// IRQ flags mask
#[derive(Clone, Copy)]
pub struct IrqFlagsMask(IrqFlags);

impl Register<LORA> for IrqFlagsMask{}
impl ReadableRegister<LORA> for IrqFlagsMask{
    const ADDRESS: u8 = 17;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for IrqFlagsMask {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(IrqFlags::try_from(v)?))
    }
}
impl Into<IrqFlags> for IrqFlagsMask {
    fn into(self) -> IrqFlags {
        self.0
    }
}

impl WritableRegister<LORA> for IrqFlagsMask{
    const ADDRESS: u8 = 17;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for IrqFlagsMask {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 1 {
            return Err(SX1278Error::SerializationError)
        }
        dest[0] = self.0.bits();
        return Ok(1)
    }
}
impl From<IrqFlags> for IrqFlagsMask {
    fn from(v: IrqFlags) -> Self {
        Self(v)
    }
}
