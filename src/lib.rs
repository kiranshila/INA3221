#![doc = include_str!("../README.md")]
#![no_std]

pub mod registers;

use embedded_hal::blocking::i2c;
use packed_struct::{PackedStruct, PackingError};
use registers::*;

#[derive(Debug)]
pub struct INA3221<I> {
    i2c: I,
    address: u8,
}

#[derive(Debug)]
pub enum Error<E> {
    I2C(E),
    RegisterUnpacking(PackingError),
    RegisterPacking(PackingError),
}

#[derive(Debug, Copy, Clone)]
pub enum AddressPin {
    Gnd,
    Vs,
    Sda,
    Scl,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The three measurement channels of the INA3221
pub enum Channel {
    Ch1,
    Ch2,
    Ch3,
}

const SHUNT_LSB: f32 = 40e-6;
const BUS_LSB: f32 = 8e-3;

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

    /// Read `N` bytes from device at addr pointer `addr`
    fn read<const N: usize>(&mut self, addr: u8) -> Result<[u8; N], Error<E>> {
        let mut buf = [0u8; N];
        self.i2c
            .write_read(self.address, &[addr], &mut buf)
            .map_err(|e| Error::I2C(e))?;
        Ok(buf)
    }

    fn write<const N: usize>(&mut self, addr: u8, payload: &[u8; N]) -> Result<(), Error<E>> {
        // Set the pointer
        self.i2c.write(self.address, &[addr]);
        // Write the payload
        self.i2c
            .write(self.address, payload)
            .map_err(|e| Error::I2C(e))?;
        Ok(())
    }

    fn read_reg<T, const N: usize>(&mut self) -> Result<T, Error<E>>
    where
        T: Address + PackedStruct<ByteArray = [u8; N]>,
    {
        T::unpack(&self.read(T::addr())?).map_err(|e| Error::RegisterUnpacking(e))
    }

    fn write_reg<T, const N: usize>(&mut self, reg: T) -> Result<(), Error<E>>
    where
        T: Address + PackedStruct<ByteArray = [u8; N]>,
    {
        let bytes = reg.pack().map_err(|e| Error::RegisterPacking(e))?;
        self.write(T::addr(), &bytes)?;
        Ok(())
    }

    /// Generate a system reset that is the same as the power on reset.
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        let payload = Configuration {
            rst: true,
            ..Default::default()
        };
        self.write_reg(payload)?;
        Ok(())
    }

    /// Sets the channel enable bit for a given channel
    pub fn enable_channel(&mut self, channel: Channel, enabled: bool) -> Result<(), Error<E>> {
        // Read the current configuration
        let mut config: Configuration = self.read_reg()?;
        // Set the bit
        match channel {
            Channel::Ch1 => config.enable_ch1 = enabled,
            Channel::Ch2 => config.enable_ch2 = enabled,
            Channel::Ch3 => config.enable_ch3 = enabled,
        };
        self.write_reg(config)?;
        Ok(())
    }

    /// Gets the current bus-voltage conversion time
    pub fn get_bus_ct(&mut self) -> Result<ConversionTime, Error<E>> {
        let config: Configuration = self.read_reg()?;
        Ok(config.vbus_ct)
    }

    /// Gets the current shunt-voltage conversion time
    pub fn get_shunt_ct(&mut self) -> Result<ConversionTime, Error<E>> {
        let config: Configuration = self.read_reg()?;
        Ok(config.vsh_ct)
    }

    /// Set the bus-voltage conversion time
    pub fn set_bus_ct(&mut self, ct: ConversionTime) -> Result<(), Error<E>> {
        // Read the current configuration
        let mut config: Configuration = self.read_reg()?;
        config.vbus_ct = ct;
        self.write_reg(config)?;
        Ok(())
    }

    /// Set the shunt-voltage conversion time
    pub fn set_shunt_ct(&mut self, ct: ConversionTime) -> Result<(), Error<E>> {
        // Read the current configuration
        let mut config: Configuration = self.read_reg()?;
        config.vsh_ct = ct;
        self.write_reg(config)?;
        Ok(())
    }

    /// Gets the current operating mode
    pub fn get_mode(&mut self) -> Result<Mode, Error<E>> {
        let config: Configuration = self.read_reg()?;
        Ok(config.mode)
    }

    /// Sets the current operating mode
    pub fn set_mode(&mut self, mode: Mode) -> Result<(), Error<E>> {
        let mut config: Configuration = self.read_reg()?;
        config.mode = mode;
        self.write_reg(config)?;
        Ok(())
    }

    /// Gets the shunt voltage of a given channel
    pub fn shunt_voltage(&mut self, channel: Channel) -> Result<f32, Error<E>> {
        Ok(match channel {
            Channel::Ch1 => self.read_reg::<Ch1ShuntVoltage, 2>()?.voltage,
            Channel::Ch2 => self.read_reg::<Ch2ShuntVoltage, 2>()?.voltage,
            Channel::Ch3 => self.read_reg::<Ch3ShuntVoltage, 2>()?.voltage,
        } as f32
            * SHUNT_LSB)
    }

    /// Gets the bus voltage of a given channel
    pub fn bus_voltage(&mut self, channel: Channel) -> Result<f32, Error<E>> {
        Ok(match channel {
            Channel::Ch1 => self.read_reg::<Ch1BusVoltage, 2>()?.voltage,
            Channel::Ch2 => self.read_reg::<Ch2BusVoltage, 2>()?.voltage,
            Channel::Ch3 => self.read_reg::<Ch3BusVoltage, 2>()?.voltage,
        } as f32
            * BUS_LSB)
    }

    /// Get the manufacturer id. This should equal 0x5449
    pub fn manufacturer_id(&mut self) -> Result<u16, Error<E>> {
        let mid: ManufacturerId = self.read_reg()?;
        Ok(mid.id)
    }

    /// Get the die id. This should equal 0x3220
    pub fn die_id(&mut self) -> Result<u16, Error<E>> {
        let did: DieId = self.read_reg()?;
        Ok(did.id)
    }
}
