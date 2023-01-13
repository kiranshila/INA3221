use packed_struct::prelude::*;

pub trait Address {
    fn addr() -> u8;
}

macro_rules! reg_addr {
    ($reg:ident,$addr:literal) => {
        impl Address for $reg {
            fn addr() -> u8 {
                $addr
            }
        }
    };
}

#[derive(PackedStruct, Default, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Configuration {
    #[packed_field(bits = "15")]
    pub(crate) rst: bool,
    #[packed_field(bits = "14")]
    pub(crate) enable_ch1: bool,
    #[packed_field(bits = "13")]
    pub(crate) enable_ch2: bool,
    #[packed_field(bits = "12")]
    pub(crate) enable_ch3: bool,
    #[packed_field(bits = "9..=11", ty = "enum")]
    pub(crate) avg: Averages,
    #[packed_field(bits = "6..=8", ty = "enum")]
    pub(crate) vbus_ct: ConversionTime,
    #[packed_field(bits = "3..=5", ty = "enum")]
    pub(crate) vsh_ct: ConversionTime,
    #[packed_field(bits = "0..=2", ty = "enum")]
    pub(crate) mode: Mode,
}

reg_addr!(Configuration, 0x00);

#[derive(Default, PrimitiveEnum, Copy, Clone, Debug)]
/// Averaging mode. This sets the number of samples that are collected and averaged.
pub enum Averages {
    #[default]
    _1 = 0,
    _4 = 1,
    _16 = 2,
    _64 = 3,
    _128 = 4,
    _256 = 5,
    _512 = 6,
    _1024 = 7,
}

#[derive(Default, PrimitiveEnum, Copy, Clone, Debug)]
/// Cconversion time in ms.
pub enum ConversionTime {
    _0_140 = 0,
    _0_204 = 1,
    _0_332 = 2,
    _0_588 = 3,
    #[default]
    _1_1 = 4,
    _2_116 = 5,
    _4_1566 = 6,
    _8_244 = 7,
}

#[derive(Default, PrimitiveEnum, Copy, Clone, Debug)]
pub enum Mode {
    PowerDown = 0,
    ShuntVoltSS = 1,
    BusVoltSS = 2,
    ShuntBusSS = 3,
    ShuntVoltCont = 5,
    BusVoltCont = 6,
    #[default]
    ShutBusCont = 7,
}

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Ch1ShuntVoltage {
    #[packed_field(bits = "3..=15", endian = "msb")]
    pub(crate) voltage: i16,
}

reg_addr!(Ch1ShuntVoltage, 0x01);

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Ch1BusVoltage {
    #[packed_field(bits = "3..=15", endian = "msb")]
    pub(crate) voltage: i16,
}

reg_addr!(Ch1BusVoltage, 0x02);

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Ch2ShuntVoltage {
    #[packed_field(bits = "3..=15", endian = "msb")]
    pub(crate) voltage: i16,
}

reg_addr!(Ch2ShuntVoltage, 0x03);

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Ch2BusVoltage {
    #[packed_field(bits = "3..=15", endian = "msb")]
    pub(crate) voltage: i16,
}

reg_addr!(Ch2BusVoltage, 0x04);

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Ch3ShuntVoltage {
    #[packed_field(bits = "3..=15", endian = "msb")]
    pub(crate) voltage: i16,
}

reg_addr!(Ch3ShuntVoltage, 0x05);

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Ch3BusVoltage {
    #[packed_field(bits = "3..=15", endian = "msb")]
    pub(crate) voltage: i16,
}

reg_addr!(Ch3BusVoltage, 0x06);

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct ManufacturerId {
    #[packed_field(bits = "0..=15", endian = "msb")]
    pub(crate) id: u16,
}

reg_addr!(ManufacturerId, 0xFE);

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct DieId {
    #[packed_field(bits = "0..=15", endian = "msb")]
    pub(crate) id: u16,
}

reg_addr!(DieId, 0xFF);
