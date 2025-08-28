#![cfg_attr(not(test), no_std)]

use crate::crypto::CryptoContext;

pub mod crypto;

#[derive(Default, Debug, Clone)]
#[repr(packed)]
pub struct IotPacket {
    _pvn: [u8; 3],
    _node_id: [u8; 4],
    _timestamp: [u8; 4],    
    _len: u8,
    _payload: [u8; 10],
    _tag: [u8; 8]
}

impl IotPacket {

    fn _new(mut payload: [u8; 10], node_id: u32, timestamp: u32, crypto_context: &CryptoContext) -> Result<Self, ()> {
        let node_id = node_id.to_be_bytes();
        let timestamp = timestamp.to_be_bytes();       
        let tag = crypto_context.authenticate(&node_id, &timestamp, &payload);
        crypto_context.encrypt(&mut payload, &tag)?;
        return Ok(Self {
            _pvn: [0u8; 3],
            _node_id: node_id,
            _timestamp: timestamp,
            _len: 10,
            _payload: payload,
            _tag: tag
        })
    }

    pub fn new<T: Sized>(data: &T, node_id: u32, timestamp: u32, crypto_context: &CryptoContext) -> Result<Self, ()> {
        let pldsz = size_of::<T>();
        if pldsz > 10 {
            return Err(())
        }
        
        let mut payload = [0u8; 10];
        payload[..pldsz].copy_from_slice(unsafe { core::slice::from_raw_parts(data as *const T as _, pldsz) });
        Self::_new(payload, node_id, timestamp, crypto_context)
    }

    pub fn new_from_slice(data: &[u8], node_id: u32, timestamp: u32, crypto_context: &CryptoContext) -> Result<Self, ()> {
        if data.len() > 10 {
            return Err(())
        }
        let mut payload = [0u8; 10];
        payload[..data.len()].copy_from_slice(data);
        Self::_new(payload, node_id, timestamp, crypto_context)
    }

    pub fn as_slice<'a>(&self) -> &'a[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, 30) }
    }

    pub fn serialize(&self) -> [u8; 30] {
        let mut r = [0u8; 30];
        r.copy_from_slice(self.as_slice());
        r
    }

    pub fn serialize_to(&self, buf: &mut [u8]) -> Result<(), ()> {
        if buf.len() < 30 {
            return Err(())
        }
        buf[..30].copy_from_slice(self.as_slice());
        return Ok(())
    }
}

