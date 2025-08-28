
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::FSK;
use modular_bitfield::prelude::*;

/// Controls the state-machine transition from the PacketReceived state
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum FromPacketReceived {
    /// .
    SequencerOff = 0,
    /// .
    TransmitOnFifoEmpty = 1,
    /// .
    LowPowerSelection = 2,
    /// .
    ReceiveViaFSMode = 3,
    /// .
    Receive = 4,
}

/// Controls the state-machine transition from the Receive state on aRxTimeout Interrupt
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum FromRxTimeout {
    /// .
    Receive = 0,
    /// .
    Transmit = 1,
    /// .
    LowPowerSelection = 2,
    /// .
    SequencerOff = 3,
}

/// Controls the Sequencer transition from the Receive state
#[derive(Specifier)]
#[bits = 3]
#[derive(Clone, Copy)]
pub enum FromReceive {
    /// .
    ToPacketReceivedOnPayloadReady = 1,
    /// .
    ToLowPowerSelectionOnPayloadReady = 2,
    /// .
    ToPacketReceivedOnCrcOk = 3,
    /// .
    ToSequencerOffOnRssi = 4,
    /// .
    ToSequencerOffOnSyncAddress = 5,
    /// .
    ToSequencerOffOnPreambleDetect = 6,
}

/// Controls the Sequencer transition from the Transmit state
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum FromTransmit {
    /// to LowPowerSelection
    LowPowerSel = 0,
    /// to Receive state
    Receive = 1,
}

/// Controls the Sequencer transition from the Idle state on a T1 interrupt
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum FromIdle {
    /// To Transmit
    Transmit = 0,
    /// To Receive
    Receive = 1,
}

/// Selects the Sequencer LowPower state after a to LowPowerSelection transition
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum LowPowerSelection {
    /// SequencerOff
    SequencerOff = 0,
    /// Idle state with chip on Standby or Sleep mode depending on IdleMode
    Idle = 1,
}

/// Controls the Sequencer transition when SequencerStart is set to 1 in Sleep or Standby mode
#[derive(Specifier)]
#[bits = 2]
#[derive(Clone, Copy)]
pub enum FromStart {
    /// To LowPowerSelection
    LowPower = 0,
    /// To RX
    Receive = 1,
    /// To Tx
    Transmit = 2,
    /// to Transmit state on a FifoLevel interrupt
    TransmitOnFifoIrq = 3,
}

/// Selects chip mode during the state
#[derive(Specifier)]
#[bits = 1]
#[derive(Clone, Copy)]
pub enum IdleMode {
    /// Go to Standby mode
    Standby = 0,
    /// Go to sleep mode
    Sleep = 1,
}

/// Sequencer configuration
#[bitfield(bits = 16)]
#[derive(Clone, Copy)]
pub struct SeqConfig {
    /// Controls the state-machine transition from the PacketReceived state
     #[bits = 3] from_packet_received: FromPacketReceived,
    /// Controls the state-machine transition from the Receive state on aRxTimeout Interrupt
     #[bits = 2] from_rx_timeout: FromRxTimeout,
    /// Controls the Sequencer transition from the Receive state
     #[bits = 3] from_receive: FromReceive,
    /// Controls the Sequencer transition from the Transmit state
     #[bits = 1] from_transmit: FromTransmit,
    /// Controls the Sequencer transition from the Idle state on a T1 interrupt
     #[bits = 1] from_idle: FromIdle,
    /// Selects the Sequencer LowPower state after a to LowPowerSelection transition
     #[bits = 1] low_power_selection: LowPowerSelection,
    /// Controls the Sequencer transition when SequencerStart is set to 1 in Sleep or Standby mode
     #[bits = 2] from_start: FromStart,
    /// Selects chip mode during the state
     #[bits = 1] idle_mode: IdleMode,
 sequencer_stop: bool,
 sequencer_start: bool,
}

impl Register<FSK> for SeqConfig{}

impl ReadableRegister<FSK> for SeqConfig{
    const ADDRESS: u8 = 54;
    const SIZE: usize = 2;
}
impl<'x> TryFrom<&'x[u8]> for SeqConfig {
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

impl WritableRegister<FSK> for SeqConfig{
    const ADDRESS: u8 = 54;
    const SIZE: usize = 2;
}
impl TryWriteInto<[u8], usize> for SeqConfig {
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
