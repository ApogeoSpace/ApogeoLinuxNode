
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::LORA;

use super::irq_flags::IrqFlags;

#[derive(Clone, Copy, Debug)]
pub enum DioMappingSelection {
    Irq(IrqFlags),
    PllLock,
    ModeReady,
    ClockOut,
    None
}

/// Interrupt mapping
#[derive(Clone, Copy, Debug)]
pub struct DioMapping(DioMappingSelection, DioMappingSelection, DioMappingSelection, DioMappingSelection, DioMappingSelection, DioMappingSelection);

impl DioMapping {
    pub fn new() -> Self {
        return Self::try_from([0u8, 2].as_slice()).unwrap();
    }

    pub fn set_mapping(&mut self, index: u8, value: DioMappingSelection) -> &mut Self {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            4 => self.4 = value,
            5 => self.5 = value,
            _ => {}
        }
        return self;
    }
}

impl Register<LORA> for DioMapping{}

impl ReadableRegister<LORA> for DioMapping{
    const ADDRESS: u8 = 64;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for DioMapping {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 2 {
            return Err(SX1278Error::ParseError)
        }
       
        let dio0 = match (v[0] & 0xC0) >> 6 {
            0 => Ok(DioMappingSelection::Irq(IrqFlags::RXDone)),
            1 => Ok(DioMappingSelection::Irq(IrqFlags::TXDone)),
            2 => Ok(DioMappingSelection::Irq(IrqFlags::CadDone)),
            3 => Ok(DioMappingSelection::None),
            _ => Err(SX1278Error::ParseError)
        };
        let dio1 = match (v[0] & 0x30) >> 4 {
            0 => Ok(DioMappingSelection::Irq(IrqFlags::RXTimeout)),
            1 => Ok(DioMappingSelection::Irq(IrqFlags::FHSSChannelChange)),
            2 => Ok(DioMappingSelection::Irq(IrqFlags::CadDetected)),
            3 => Ok(DioMappingSelection::None),
            _ => Err(SX1278Error::ParseError)
        };
        let dio2 = match (v[0] & 0x0C) >> 2 {
            0| 1 | 2 => Ok(DioMappingSelection::Irq(IrqFlags::FHSSChannelChange)),
            _ => Err(SX1278Error::ParseError)
        };
        let dio3 = match (v[0] & 0x03) >> 0 {
            0 => Ok(DioMappingSelection::Irq(IrqFlags::CadDone)),
            1 => Ok(DioMappingSelection::Irq(IrqFlags::HeaderValid)),
            2 => Ok(DioMappingSelection::Irq(IrqFlags::PayloadCRCError)),
            3 => Ok(DioMappingSelection::None),
            _ => Err(SX1278Error::ParseError)
        };
        let dio4 = match (v[1] & 0xC0) >> 6 {
            0 => Ok(DioMappingSelection::Irq(IrqFlags::CadDetected)),
            1 | 2 => Ok(DioMappingSelection::PllLock),
            3 => Ok(DioMappingSelection::None),
            _ => Err(SX1278Error::ParseError)
        };
        let dio5 = match (v[1] & 0x30) >> 4 {
            0 => Ok(DioMappingSelection::ModeReady),
            1 | 2 => Ok(DioMappingSelection::ClockOut),
            3 => Ok(DioMappingSelection::None),
            _ => Err(SX1278Error::ParseError)
        };
        
        return Ok(Self(dio0?, dio1?, dio2?, dio3?, dio4?, dio5?))
    }
}

impl WritableRegister<LORA> for DioMapping{
    const ADDRESS: u8 = 64;
    const SIZE: usize = 2;
}
impl TryWriteInto<[u8], usize> for DioMapping {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 2 {
            return Err(SX1278Error::SerializationError)
        }
        
        dest[0] = (match self.0 {
            DioMappingSelection::Irq(IrqFlags::RXDone) => Ok(0x00),
            DioMappingSelection::Irq(IrqFlags::TXDone) => Ok(0x40),
            DioMappingSelection::Irq(IrqFlags::CadDone) => Ok(0x80),
            DioMappingSelection::None => Ok(0xC0),
            _ => Err(SX1278Error::SerializationError)
        })? | (match self.1 {
            DioMappingSelection::Irq(IrqFlags::RXTimeout) => Ok(0x00),
            DioMappingSelection::Irq(IrqFlags::FHSSChannelChange) => Ok(0x10),
            DioMappingSelection::Irq(IrqFlags::CadDetected) => Ok(0x20),
            DioMappingSelection::None => Ok(0x30),
            _ => Err(SX1278Error::SerializationError)
        })? | (match self.2 {
            DioMappingSelection::Irq(IrqFlags::FHSSChannelChange) => Ok(0x04),
            DioMappingSelection::None => Ok(0x0C),
            _ => Err(SX1278Error::SerializationError)
        })? | (match self.3 {
            DioMappingSelection::Irq(IrqFlags::CadDone) => Ok(0x00),
            DioMappingSelection::Irq(IrqFlags::HeaderValid) => Ok(0x01),
            DioMappingSelection::Irq(IrqFlags::PayloadCRCError) => Ok(0x02),
            DioMappingSelection::None => Ok(0x03),
            _ => Err(SX1278Error::SerializationError)
        })?;

        dest[1] = (match self.4 {
            DioMappingSelection::Irq(IrqFlags::CadDetected) => Ok(0x00),
            DioMappingSelection::PllLock => Ok(0x40),
            DioMappingSelection::None => Ok(0xC0),
            _ => Err(SX1278Error::SerializationError)
        })? | (match self.5 {
            DioMappingSelection::ModeReady => Ok(0x00),
            DioMappingSelection::ClockOut => Ok(0x10),
            DioMappingSelection::None => Ok(0x30),
            _ => Err(SX1278Error::SerializationError)
        })?;

        return Ok(2)
    }
}
