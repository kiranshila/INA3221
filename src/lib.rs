#![doc = include_str!("../README.md")]
#![no_std]

pub mod registers;

use embedded_hal::blocking::i2c;
use packed_struct::{PackedStruct, PackingError};
use registers::*;

pub struct INA3221<I> {
    i2c: I,
    address: u8,
}

pub enum Error<E> {
    I2C(E),
    RegisterUnpacking(PackingError),
}

#[derive(Debug, Copy, Clone)]
pub enum AddressPin {
    Gnd,
    Vs,
    Sda,
    Scl,
}

impl AddressPin {
    fn addr(&self) -> u8 {
        match self {
            AddressPin::Gnd => 0b1000000,
            AddressPin::Vs => 0b1000001,
            AddressPin::Sda => 0b1000010,
            AddressPin::Scl => 0b1000011,
        }
    }
}

impl<E, I> INA3221<I>
where
    I: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I, addr_pin: AddressPin) -> Self {
        Self {
            i2c,
            address: addr_pin.addr(),
        }
    }

    fn read<const N: usize>(&mut self, addr: u8) -> Result<[u8; N], Error<E>> {
        let mut buf = [0u8; N];
        self.i2c.read(addr, &mut buf).map_err(|e| Error::I2C(e))?;
        Ok(buf)
    }

    fn read_reg<T, const N: usize>(&mut self) -> Result<T, Error<E>>
    where
        T: Address + PackedStruct<ByteArray = [u8; N]>,
    {
        T::unpack(&self.read(T::addr())?).map_err(|e| Error::RegisterUnpacking(e))
    }

    /// Get the manufacturer id. This should equal 0x5449
    pub fn manufacturer_id(&mut self) -> Result<u16, Error<E>> {
        let mid: ManufacturerId = self.read_reg()?;
        Ok(mid.id)
    }

    /// Get the die id. This should equal 0x3220
    pub fn die(&mut self) -> Result<u16, Error<E>> {
        let did: DieId = self.read_reg()?;
        Ok(did.id)
    }
}
