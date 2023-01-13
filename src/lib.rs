#![no_std]

pub mod registers;
use embedded_hal::blocking::i2c;

pub struct INA3221<I> {
    i2c: I,
    address: u8,
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

    fn read<const N: usize>(&mut self, addr: u8) -> Result<[u8; N], Error<E>> {}
}
