#![no_std]

pub mod registers;
pub mod error;

mod lut;

use core::marker::PhantomData;

use crate::error::SX1278Error;
use crate::registers::all::frequency::Frequency;
use crate::registers::all::pa_config::{PaConfig, PaSelect};
use crate::registers::lora::fifo_addr_ptr::FifoAddrPtr;
use crate::registers::lora::modem_config::{Bw, CodingRate, ModemConfig, SpreadingFactor};
use crate::registers::lora::modem_config_ext::{AgcMode, ModemConfigExt};
use crate::registers::{ReadableRegister, WritableRegister, TryWriteInto};

use crate::registers::all::version::Version;
use crate::registers::all::tcxo::{Tcxo, TcxoInputValue};
use crate::registers::all::op_mode::{OpMode, Mode, LongRangeMode};
use crate::registers::lora::op_mode::{OpMode as LoRaOpMode, Mode as LoRaMode};

pub trait SX1278Mode {}

pub struct Uninitialized;
impl SX1278Mode for Uninitialized{}

pub struct LORA;
impl SX1278Mode for LORA{}

pub struct FSK;
impl SX1278Mode for FSK{}


pub trait SpiAdapter {
    fn transaction(&mut self, out_buf: &[u8], in_buf: &mut[u8], concurrent: bool) -> Result<(), SX1278Error>;
    fn read(&mut self, in_buf: &mut[u8]) -> Result<(), SX1278Error>;
    fn write(&mut self, out_buf: &[u8]) -> Result<(), SX1278Error>;
}


pub struct SX1278<MODE: SX1278Mode, SPI: SpiAdapter> {
    spi: SPI,
    xtal: u32,
    _phantom: PhantomData<MODE>
}

fn power_bsearch(lut: &[i16], tgt: i16) -> usize {
    let (mut start, mut end) = (0usize, lut.len() - 1);
    for _ in 0..20 {
        let idx = (end + start) / 2;
        if lut[idx] == tgt { return idx };

        if idx == 0 { return 0 }
        if idx == lut.len() - 1 { return lut.len() - 1 }
        
        if lut[idx] <= tgt && lut[idx + 1] > tgt { return idx };
        if lut[idx] > tgt && lut[idx - 1] <= tgt { return idx - 1 };
        
        if lut[idx] < tgt { 
            if start == idx { start = idx+1 }
            else { start = idx }
        } else { 
            if end == idx { end = idx-1 }
            else { end = idx }
        }
    }
    return 0;
}



impl<MODE: SX1278Mode, SPI: SpiAdapter> SX1278<MODE, SPI> {
    pub fn new(spi: SPI, xtal: u32, is_tcxo: bool) -> Result<SX1278<Uninitialized, SPI>, SX1278Error>{
        let mut dev = SX1278 {
            spi,
            xtal,
            _phantom: PhantomData{ }
        };

        let version = dev.read_register::<Version>()?;
        if Version::SX1278_VERSION != version {
            return Err(SX1278Error::InvalidDevice(0x12, version.into()))
        }

        // set xtal registers
        dev.write_register::<Tcxo>(&(if is_tcxo { TcxoInputValue::On } else { TcxoInputValue::Off }).into())?;
        return Ok(dev)
    }

    pub fn lora(mut self) -> Result<SX1278<LORA, SPI>, SX1278Error> {
        self.set_device_mode(Mode::Sleep)?;
        self.write_register(&OpMode::new().with_mode(Mode::Sleep).with_long_range_mode(LongRangeMode::LoRaMode))?;

        return Ok(SX1278 {
            spi: self.spi,
            xtal: self.xtal,
            _phantom: PhantomData{ }
        })
    }

    pub fn fsk(mut self) -> Result<SX1278<FSK, SPI>, SX1278Error> {
        self.set_device_mode(Mode::Sleep)?;
        self.write_register(&OpMode::new().with_mode(Mode::Sleep).with_long_range_mode(LongRangeMode::FskOokMode))?;

        return Ok(SX1278 {
            spi: self.spi,
            xtal: self.xtal,
            _phantom: PhantomData{ }
        })
    }

    pub fn reset(mut self) -> Result<SX1278<Uninitialized, SPI>, SX1278Error> {
        /* todo: reset toggle */
        self.set_device_mode(Mode::Sleep)?;

        return Ok(SX1278 {
            spi: self.spi,
            xtal: self.xtal,
            _phantom: PhantomData{ }
        })
    }



    pub fn set_device_mode(&mut self, mode: Mode) -> Result<(), SX1278Error> {
        let mut reg = self.read_register::<OpMode>()?;
        reg.set_mode(mode);
        self.write_register(&reg)
    }

    pub fn set_carrier_frequency(&mut self, freq: f64) -> Result<&mut Self, SX1278Error> {
        let f = Frequency::steps(freq, self.xtal);
        self.write_register(&f)?;

        let mode = f.frequency_mode(self.xtal);
        if let Some(lfm) = mode {
            let mut reg = self.read_register::<OpMode>()?;
            reg.set_low_frequency_mode(lfm);
            self.write_register(&reg)?
        } else {
            return Err(SX1278Error::ConfigurationError)
        }
        
        return Ok(self)
    }

    pub fn set_tx_power(&mut self, output_pin: PaSelect, power_dbm: f32) -> Result<&mut Self, SX1278Error> {
        let (mp, op) = match output_pin {
            PaSelect::RfoPin => {
                let i = power_bsearch(&lut::lut_pwr_sel::LUT_PWR_SEL_RFO[lut::lut_pwr_sel::LUT_PWE_SEL_VAL], (power_dbm * 10f32) as i16);
                (lut::lut_pwr_sel::LUT_PWR_SEL_RFO[lut::lut_pwr_sel::LUT_PWE_SEL_MPWR][i], lut::lut_pwr_sel::LUT_PWR_SEL_RFO[lut::lut_pwr_sel::LUT_PWE_SEL_OPWR][i])
            },
            PaSelect::PaBoostPin => {
                let i = power_bsearch(&lut::lut_pwr_sel::LUT_PWR_SEL_BOOST[lut::lut_pwr_sel::LUT_PWE_SEL_VAL], (power_dbm * 10f32) as i16);
                (0, lut::lut_pwr_sel::LUT_PWR_SEL_BOOST[lut::lut_pwr_sel::LUT_PWE_SEL_OPWR][i])
            },
        };

        self.write_register(&PaConfig::new().with_max_power(mp as u8).with_output_power(op as u8).with_pa_select(output_pin))?;

        return Ok(self);
    }


    

    fn bus_transaction_read(&mut self, address: u8, dest: &mut [u8]) -> Result<(), SX1278Error> {
        assert!(address < 128);
        self.spi.transaction(&[address], dest, false)
    }

    fn from_transaction_read<T: for<'x> TryFrom<&'x [u8], Error=SX1278Error>>(&mut self, address: u8, len: usize) -> Result<T, SX1278Error>{
        let mut buffer = [0u8; 257];
        self.bus_transaction_read(address, &mut buffer[..len])?;
        return T::try_from(&buffer[..len])
    }

    fn to_transaction_write<T: TryWriteInto<[u8], usize, Error=SX1278Error>>(&mut self, address: u8, elm: &T, size: usize) -> Result<(), SX1278Error>{
        assert!(address < 128);
        let mut buf = [address | 128; 257];
        let _ = elm.try_write_into(&mut buf[1..1 + size])?;
        return self.spi.write(&buf[..size+1])
    }

    pub fn read_register<T: ReadableRegister<MODE>>(&mut self) -> Result<T, SX1278Error> {
        self.from_transaction_read(T::ADDRESS, T::SIZE)
    }

    pub fn write_register<T: WritableRegister<MODE>>(&mut self, reg: &T) -> Result<(), SX1278Error> {
        self.to_transaction_write(T::ADDRESS, reg, T::SIZE)
    }

}


impl<SPI: SpiAdapter> SX1278<LORA, SPI> {
    pub fn set_lora_device_mode(&mut self, mode: LoRaMode) -> Result<(), SX1278Error> {
        let mut reg = self.read_register::<LoRaOpMode>()?;
        reg.set_mode(mode);
        self.write_register(&reg)
    }


    pub fn set_modem_config(&mut self, has_crc: bool, sf: SpreadingFactor, header: bool, cr: CodingRate, bw: Bw, agc: AgcMode, ldro: bool) -> Result<(), SX1278Error> {
        self.write_register(
            &ModemConfig::new()
                .with_bw(bw)
                .with_coding_rate(cr)
                .with_implicit_header(!header)
                .with_spreading_factor(sf)
                .with_rx_payload_crc(has_crc)
        )?;
        self.write_register(
            &ModemConfigExt::new()
                .with_agc_mode(agc)
                .with_low_data_rate_optimize(ldro)
        )
    }
    
    pub fn write_fifo(&mut self, addr: u8, data: &[u8]) -> Result<(), SX1278Error> {
        assert!((data.len() + addr as usize) <= 256);

        self.write_register::<FifoAddrPtr>(&addr.into())?;

        let mut buf = [128u8; 257];
        buf[1..data.len() + 1].copy_from_slice(data);
        return self.spi.write(&buf[..data.len() + 1])
    }

    pub fn read_fifo(&mut self, addr: u8, data: &mut [u8]) -> Result<(), SX1278Error> {
        assert!((data.len() + addr as usize) <= 256);

        self.write_register::<FifoAddrPtr>(&addr.into())?;
        return self.spi.transaction(&[128u8], data, false)
    }
}