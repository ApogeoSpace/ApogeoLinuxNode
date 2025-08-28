#![no_std]
#![feature(c_size_t)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

pub mod panic;
pub mod allocator;
pub mod logging;
pub mod interface;
pub mod pwr_rules;

use core::cmp::min;
use core::ffi::c_void;
use aps_crypto::crypto::key::Key;
use aps_crypto::crypto::CryptoContext;
use aps_crypto::IotPacket;
use sx1278::registers::all::frequency::{Frequency, LowFrequencyMode};
use sx1278::registers::all::pa_config::PaSelect;
use sx1278::registers::lora::dio_mapping::{DioMapping, DioMappingSelection};
use sx1278::registers::lora::fifo_rx_base_addr::FifoRxBaseAddr;
use sx1278::registers::lora::fifo_tx_base_addr::FifoTxBaseAddr;
use sx1278::registers::lora::frequency_error::FrequencyError;
use sx1278::registers::lora::irq_flags::IrqFlags;
use sx1278::registers::lora::irq_flags_mask::IrqFlagsMask;
use sx1278::registers::lora::modem_config::{Bw, CodingRate, SpreadingFactor};
use sx1278::registers::lora::modem_config_ext::AgcMode;
use sx1278::registers::lora::op_mode::{Mode, OpMode};
use sx1278::registers::lora::packet_snr_value::PacketSnrValue;
use sx1278::registers::lora::payload_length::PayloadLength;
use sx1278::registers::lora::preamble::Preamble;
use sx1278::registers::lora::rssi_value::RssiValueRaw;
use sx1278::registers::lora::rx_nb_bytes::RxNbBytes;
use sx1278::registers::Specifier;
use sx1278::{SX1278, LORA, Uninitialized, SpiAdapter};
use sx1278::error::SX1278Error;

use crate::interface::callbacks::{__shim_mutex_lock, __shim_mutex_unlock, __shim_usleep_range, __shim_get_timestamp, spi_transaction};
use crate::interface::interface::IotdConfig;
use crate::pwr_rules::COUNTRIES_LUT;

struct KernelSpiAdapter {
    opaque_context: *mut c_void
}


#[derive(Debug)]
enum IoTdState {
    Idle,
    ChannelActivityDetection,
    Transmit,
    Receive,
    Closed
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OperationResult {
    Success,
    Error,
    Pending
}

pub struct IoTd {
    opaque_context: *mut c_void,

    polling: bool,
    aps_proto: bool,
    cad: bool,

    cad_retries: usize,
    carrier_freq: Frequency,
    bw: Bw,
    xtal: u32,
    pa_boost: bool,

    device: SX1278<LORA, KernelSpiAdapter>,

    node_id: u32,
    crypto_context: CryptoContext,

    state: IoTdState,
    last_op_result: Option<OperationResult>,
}


impl SpiAdapter for KernelSpiAdapter {
    fn transaction(&mut self, out_buf: &[u8], in_buf: &mut[u8], concurrent: bool) -> Result<(), SX1278Error>{
        let ret = unsafe { spi_transaction(
            self.opaque_context, 
            out_buf as *const _  as *const c_void, 
            in_buf as *mut _ as *mut c_void,
            out_buf.len() as u32, 
            in_buf.len() as u32, 
            if concurrent { 1 } else { 0 }
        ) };

        if ret < 0 {
            return Err(SX1278Error::SPIError(ret as i32))
        }
        return Ok(())
    }
    fn read(&mut self, in_buf: &mut[u8]) -> Result<(), SX1278Error>{
        let ret = unsafe { spi_transaction(
            self.opaque_context, 
            0 as *const c_void, 
            in_buf as *mut _ as *mut c_void, 
            0, 
            in_buf.len() as u32, 
            0
        ) };

        if ret < 0 {
            return Err(SX1278Error::SPIError(ret as i32))
        }
        return Ok(())
    }
    fn write(&mut self, out_buf: &[u8]) -> Result<(), SX1278Error>{
        let ret = unsafe { spi_transaction(
            self.opaque_context, 
            out_buf as *const _ as *const c_void, 
            0 as *mut c_void, 
            out_buf.len() as u32,
            0, 
            0
        ) };

        if ret < 0 {
            return Err(SX1278Error::SPIError(ret as i32))
        }
        return Ok(())
    }
}


impl IoTd {
    pub fn from_context(ctx: *mut c_void, polling: bool, aps_proto: bool, config: &IotdConfig) -> Result<Self, SX1278Error> {
        let spi = KernelSpiAdapter{ opaque_context: ctx.clone() };
        let device = SX1278::<Uninitialized, KernelSpiAdapter>::new(spi, config.xtal_freq, config.xtal_is_tcxo)?;
        
        let mut device = device.lora()?;
        // device.set_lora_device_mode(Mode::Standby)?;
        device.write_register::<FifoRxBaseAddr>(&0.into())?;
        device.write_register::<FifoTxBaseAddr>(&0.into())?;

        device.write_register(&DioMapping::new())?;

        
        Ok(Self {
            opaque_context: ctx,
            polling,
            aps_proto,
            cad: config.use_cad,
            cad_retries: 0,
            carrier_freq: Frequency::steps(config.carrier_frequency as f64, config.xtal_freq),
            bw: config.bandwidth.try_into()?,
            pa_boost: config.pa_boost != 0,
            xtal: config.xtal_freq,
            node_id: config.node_id,
            crypto_context: CryptoContext::new(Key::from_slice(&config.node_key).unwrap()),
            device,
            state: IoTdState::Idle,
            last_op_result: None
        })
    }

    pub fn config(&mut self, cfg: &IotdConfig) -> Result<(), SX1278Error> {
        log_debug!("Configuring device... {:?}", cfg);
        let mode = self.device.read_register::<OpMode>()?;
        
        if !mode.mode().eq(&Mode::Sleep) && !mode.mode().eq(&Mode::Standby){
            log_warning!("Cannot change configuration while operating. Close the device first.\n");
            return Err(SX1278Error::Other)
        }
        
        self.device.set_modem_config(
            cfg.use_payload_crc, 
            SpreadingFactor::from_bytes(cfg.spreading_factor).map_err(|_| SX1278Error::ConfigurationError)?, 
            cfg.use_header, 
            CodingRate::from_bytes(cfg.coding_rate).map_err(|_| SX1278Error::ConfigurationError)?, 
            Bw::from_bytes(cfg.bandwidth).map_err(|_| SX1278Error::ConfigurationError)?, 
            AgcMode::Auto, 
            cfg.use_ldro
        )?;
        self.device.set_carrier_frequency(cfg.carrier_frequency as f64)?;
        self.device.write_register::<Preamble>(&cfg.preamble_length.into())?;

        self.carrier_freq = Frequency::steps(cfg.carrier_frequency as f64, cfg.xtal_freq);
        self.bw = cfg.bandwidth.try_into()?;
        self.xtal = cfg.xtal_freq;

        self.node_id = cfg.node_id;
        self.crypto_context = CryptoContext::new(cfg.node_key.into());
        self.cad = cfg.use_cad;

        let mut p = -100.0f32;
        let nid = min(cfg.nation_id, (COUNTRIES_LUT.len() - 1) as u16) as usize;
        let nation = &COUNTRIES_LUT[nid];

        log_info!("Setting country limits for {}", nation.0);
        for limit in nation.1 {
            if let Some(v) = limit.limit(cfg.output_power as f32, cfg.carrier_frequency){
                p = v;
            }
        }
        log_debug!("Setting power output to {}dBm\n", p);
        self.device.set_tx_power(if self.pa_boost { PaSelect::PaBoostPin } else {  PaSelect::RfoPin }, p)?;
        
        Ok(())
    }
    
    pub fn dev_drop(&mut self) -> Result<(), SX1278Error> {
        self.device.set_lora_device_mode(Mode::Sleep)?;
        self.state = IoTdState::Closed;
        return Ok(());
    }

    pub fn dev_opened(&mut self) -> Result<(), SX1278Error>{
        self.state = IoTdState::Idle;
        self.device.set_lora_device_mode(Mode::Standby)
    }

    pub fn dev_closed(&mut self) -> Result<(), SX1278Error>{
        self.device.set_lora_device_mode(Mode::Sleep)
    }



    fn poll_irq(&mut self, flags: IrqFlags, attempts: usize) -> Result<IrqFlags, SX1278Error> {
        for _ in 0..attempts {
            let f = self.device.read_register::<IrqFlags>()?;
            if f.intersects(flags) {
                return Ok(f)
            }
            unsafe { __shim_usleep_range(800, 2000) };
        }

        Err(SX1278Error::Other)
    }



    fn handle_transmission(&mut self) -> Result<OperationResult, SX1278Error>{
        log_debug!("Handling TRX...\n");

        if !self.polling {
            self.device.write_register(DioMapping::new().set_mapping(0, DioMappingSelection::Irq(IrqFlags::TXDone)))?;
        }
        self.device.write_register::<IrqFlagsMask>(&IrqFlags::TXDone.complement().into())?;
        self.device.write_register::<IrqFlags>(&IrqFlags::all())?;

        self.state = IoTdState::Transmit;
        self.device.set_lora_device_mode(Mode::Tx)?;
        if !self.polling {
            return Ok(OperationResult::Pending);
        } 

        let irqs = self.poll_irq(IrqFlags::TXDone, 1000)?;
        self.handle_transmission_irq(irqs)?;

        if irqs.intersects(IrqFlags::TXDone){
            Ok(OperationResult::Success)
        } else {
            Ok(OperationResult::Error)
        }
    }

    fn handle_transmission_irq(&mut self, irqs: IrqFlags) -> Result<(), SX1278Error> {
        self.state = IoTdState::Idle;
        self.last_op_result = if irqs.intersects(IrqFlags::TXDone) { Some(OperationResult::Success) } else { Some(OperationResult::Error) };
        self.device.write_register(&irqs)?;
        return Ok(())
    }

    fn handle_cad(&mut self) -> Result<OperationResult, SX1278Error> {

        log_debug!("Handling CAD...\n");

        self.cad_retries = 0;

        if !self.polling {
            self.device.write_register(DioMapping::new().set_mapping(0, DioMappingSelection::Irq(IrqFlags::CadDone)))?;
        }
        self.device.write_register::<IrqFlagsMask>(&(IrqFlags::CadDone | IrqFlags::CadDetected).complement().into())?;
        self.device.write_register::<IrqFlags>(&IrqFlags::all())?;

        self.state = IoTdState::ChannelActivityDetection;
        self.device.set_lora_device_mode(Mode::Cad)?;
        if !self.polling {
            return Ok(OperationResult::Pending);
        }

        let mut cad_result = false;
        while !cad_result {
            let irqs: IrqFlags = self.poll_irq(IrqFlags::CadDone, 1000)?;
            cad_result = self.handle_cad_irq(irqs)?;
            if self.cad_retries >= 10 {
                return Ok(OperationResult::Error)
            }
        }
        Ok(OperationResult::Success)
    }

    fn handle_cad_irq(&mut self, irqs: IrqFlags) -> Result<bool, SX1278Error> {
        self.device.write_register(&irqs)?;

        if irqs.intersects(IrqFlags::CadDetected) {
            self.cad_retries += 1;

            if self.cad_retries < 10 {
                self.device.set_lora_device_mode(Mode::Cad)?;
            } else {
                self.state = IoTdState::Idle;
                self.last_op_result = Some(OperationResult::Error);
            }
            return Ok(false)
        }
        self.state = IoTdState::Idle;
        Ok(true)
    }


    pub fn irq(&mut self) -> Result<(), SX1278Error> {
        let irqs = self.device.read_register::<IrqFlags>()?;
        log_debug!("Handling IRQ...\n");

        match self.state {
            IoTdState::ChannelActivityDetection => {
                if self.handle_cad_irq(irqs)? {
                    self.handle_transmission()?;
                }
            },
            IoTdState::Transmit => {
                let r = self.handle_transmission_irq(irqs);
                unsafe { __shim_mutex_unlock(self.opaque_context) };
                r?;
            },
            IoTdState::Receive => {
                let r = self.handle_reception_irq(irqs);
                unsafe { __shim_mutex_unlock(self.opaque_context) };
                r?;
            },
            _ => {
                log_error!("Cannot serve IRQ when in state {:?}\n", self.state);
                self.state = IoTdState::Idle;
                self.last_op_result = None;
                return Err(SX1278Error::Other)
            },
        }

        Ok(())
    }


    pub fn trx(&mut self, buf: &[u8]) -> Result<OperationResult, SX1278Error> {
        match self.state {
            IoTdState::Idle => {},
            _ => {
                log_alert!("Already in an operation.\n");
                return Err(SX1278Error::Other);
            }
        }

        if self.aps_proto {
            let ts = (unsafe { __shim_get_timestamp() } & 0xFFFFFFFF) as u32;
            let pkt = IotPacket::new_from_slice(&buf[..min(10, buf.len())], self.node_id, ts, &self.crypto_context).map_err(|_| SX1278Error::SerializationError)?;
            log_debug!("{:?}\n", pkt);
            self.device.write_fifo(0, pkt.as_slice())?;
            self.device.write_register::<PayloadLength>(&30.into())?;
        } else {
            self.device.write_fifo(0, buf)?;
            self.device.write_register::<PayloadLength>(&(buf.len() as u8).into())?;
        }

        if self.cad {
            match self.handle_cad()? {
                OperationResult::Success => {},
                OperationResult::Pending => {
                    unsafe { __shim_mutex_lock(self.opaque_context) };
                    return Ok(OperationResult::Pending)
                },
                OperationResult::Error => return Ok(OperationResult::Error)
            }
        }
        
        return self.handle_transmission();
    }

    pub fn rx(&mut self, buf: &mut [u8]) -> Result<(OperationResult, usize), SX1278Error> {
        self.device.write_register::<PayloadLength>(&(buf.len() as u8).into())?;

        match self.handle_reception()? {
            OperationResult::Success => {},
            OperationResult::Error => return Ok((OperationResult::Error, 0)),
            OperationResult::Pending => {
                unsafe { __shim_mutex_lock(self.opaque_context) };
            }
        }

        unsafe { 
            __shim_mutex_lock(self.opaque_context);
            __shim_mutex_unlock(self.opaque_context);
        };

        if let Some(ref r) = self.last_op_result {
            if !r.eq(&OperationResult::Success){
                return Ok((OperationResult::Error, 0))
            }
        }

        let mut data_len = Into::<u8>::into(self.device.read_register::<RxNbBytes>()?) as usize;

        let blen = buf.len();
        if blen >= 7 {
            let rssi = self.device.read_register::<RssiValueRaw>()?.get_rssi(self.carrier_freq.frequency_mode(self.xtal).unwrap_or(LowFrequencyMode::LowFrequency));
            let snr: i8  = self.device.read_register::<PacketSnrValue>()?.into();
            let fei = self.device.read_register::<FrequencyError>()?.get_value(self.xtal, self.bw);

            buf[0..2].copy_from_slice(rssi.to_ne_bytes().as_slice());
            buf[2] = snr as u8;
            buf[3..7].copy_from_slice(fei.to_ne_bytes().as_slice());

            let len = min(blen - 7, data_len as usize);
            self.device.read_fifo(0, &mut buf[7..len+7])?;
            data_len = len + 7;
        } else {
            data_len = min(blen, data_len as usize);
            self.device.read_fifo(0, &mut buf[..data_len])?;
        }
        
        Ok((OperationResult::Success, data_len as usize))
    } 


    fn handle_reception(&mut self) -> Result<OperationResult, SX1278Error> {
        log_debug!("Handling RX...\n");
        if !self.polling {
            self.device.write_register(DioMapping::new().set_mapping(0, DioMappingSelection::Irq(IrqFlags::RXDone)))?;
        }

        self.device.write_register::<FifoRxBaseAddr>(&0.into())?;
        self.device.write_register::<IrqFlagsMask>(&(IrqFlags::RXDone | IrqFlags::PayloadCRCError).complement().into())?;
        self.device.write_register::<IrqFlags>(&IrqFlags::all())?;

        self.state = IoTdState::Receive;
        self.device.set_lora_device_mode(Mode::RxContinuous)?;
        
        if !self.polling {
            return Ok(OperationResult::Pending);
        } 

        let irqs = self.poll_irq(IrqFlags::RXDone, 1000)?;
        self.handle_reception_irq(irqs)?;
        if irqs.intersects(IrqFlags::PayloadCRCError){
            Ok(OperationResult::Error)
        } else if irqs.intersects(IrqFlags::RXDone){
            Ok(OperationResult::Success)
        } else {
            Ok(OperationResult::Error)
        }
    }

    fn handle_reception_irq(&mut self, irqs: IrqFlags) -> Result<(), SX1278Error> {
        self.device.set_lora_device_mode(Mode::Standby)?;
        self.last_op_result = if irqs.intersects(IrqFlags::PayloadCRCError) {
            Some(OperationResult::Error)
        } else if irqs.intersects(IrqFlags::RXDone) {
            Some(OperationResult::Success)
        } else {
            Some(OperationResult::Error)
        };

        self.device.write_register(&irqs)?;
        self.state = IoTdState::Idle;
        return Ok(())
    }

}


