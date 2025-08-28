
use crate::SX1278Mode;
use crate::error::SX1278Error;

pub mod all;
pub mod lora;
pub mod fsk;

pub use modular_bitfield::Specifier;

pub trait TryWriteInto<T: ?Sized, R = ()> {
    type Error;
    fn try_write_into(&self, dest: &mut T) -> Result<R, Self::Error>;
}

pub trait ReadableRegister<MODE: SX1278Mode>: for<'x> TryFrom<&'x [u8], Error=SX1278Error> {
    const ADDRESS: u8;
    const SIZE: usize;
}

pub trait WritableRegister<MODE: SX1278Mode>: TryWriteInto<[u8], usize, Error=SX1278Error> {
    const ADDRESS: u8;
    const SIZE: usize;
}


pub trait Register<MODE: SX1278Mode>: ReadableRegister<MODE> + WritableRegister<MODE>{
}