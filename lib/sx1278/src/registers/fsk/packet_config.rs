
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Data mode selection
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum DataMode {
    /// Continuous transmission
    Continuous = 0,
    /// Packet-based
    Packet = 1,
}

/// Address filtering enable
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum CrcWhiteningType {
    /// CCITT CRC Implementation
    CCITT = 0,
    /// IBM CRC Implementation
    IBM = 1,
}

/// Address filtering enable
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum AddressFiltering {
    /// None
    None = 0,
    /// Address field must match NodeAddress
    AddressMatch = 1,
    /// Address field must match NodeAddress or BroadcastAddress
    AddressMatchBroadcast = 2,
}

/// DC-free encoding scheme
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum DcFree {
    /// No encoding
    None = 0,
    /// Manchester encoding
    Manchester = 1,
    /// Data whitening
    Whitening = 2,
}

/// Packet format mode
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum PacketFormat {
    /// Fixed length packets
    Fixed = 0,
    /// Variable length packets
    Variable = 1,
}

/// Packet mode configuration 1
#[bitfield(bits = 16)]
#[derive(Clone, Copy)]
pub struct PacketConfig {
 payload_length_msb: B3,
 beacon: bool,
 io_home_power_frame: bool,
 io_home: bool,
    /// Data mode selection
     #[bits = 1] data_mode: DataMode,
    #[skip] __: bool,
    /// Address filtering enable
     #[bits = 1] crc_whitening_type: CrcWhiteningType,
    /// Address filtering enable
     #[bits = 2] address_filtering: AddressFiltering,
 crc_auto_clear_off: bool,
 crc: bool,
    /// DC-free encoding scheme
     #[bits = 2] dc_free: DcFree,
    /// Packet format mode
     #[bits = 1] packet_format: PacketFormat,
}

impl Register<FSK> for PacketConfig{}

impl ReadableRegister<FSK> for PacketConfig{
    const ADDRESS: u8 = 48;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for PacketConfig {
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < 2 {
            return Err(SX1278Error::ParseError)
        }
        let mut b: [u8; 2] = v.try_into().unwrap();
        b.reverse();
        Ok(Self::from_bytes(b))
    }
}

impl WritableRegister<FSK> for PacketConfig{
    const ADDRESS: u8 = 48;
    const SIZE: usize = 2;
}
impl TryWriteInto<[u8], usize> for PacketConfig {
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{
        if dest.len() < 2 {
            return Err(SX1278Error::SerializationError)
        }
        let mut repr = self.into_bytes();
        repr.reverse();
        dest[..2].copy_from_slice(&repr);
        return Ok(2)
    }
}
