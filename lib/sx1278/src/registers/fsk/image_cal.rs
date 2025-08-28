
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Temperature change threshold to trigger a new I/Q calibration
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum TempThreshols {
    /// 5C
    Th5Degree = 0,
    /// 10C
    Th10Degree = 1,
    /// 15C
    Th15Degree = 2,
    /// 20C
    Th20Degree = 3,
}

/// Image calibration
#[bitfield(bits = 8)]
#[derive(Clone, Copy)]
pub struct ImageCal {
    pub temp_monitor_disable: bool,
    /// Temperature change threshold to trigger a new I/Q calibration
     #[bits = 2] temp_threshols: TempThreshols,
     pub irq_temp_change: bool,
    #[skip] __: bool,
    pub image_cal_running: bool,
    pub image_cal_start: bool,
    pub auto_image_cal: bool,
}

impl Register<FSK> for ImageCal{}

impl ReadableRegister<FSK> for ImageCal{
    const ADDRESS: u8 = 59;
    const SIZE: usize = 1;
}
impl<'x> TryFrom<&'x[u8]> for ImageCal {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 1 {
            return Err(SX1278Error::ParseError)
        }
        let b: [u8; 1] = v.try_into().unwrap();
        
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for ImageCal{
    const ADDRESS: u8 = 59;
    const SIZE: usize = 1;
}
impl TryWriteInto<[u8], usize> for ImageCal {
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
