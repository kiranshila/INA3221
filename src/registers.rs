use packed_struct::prelude::*;

#[derive(PackedStruct)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "2")]
pub struct Configuration {
    #[packed_field(bits = "15")]
    rst: bool,
    #[packed_field(bits = "12..=14")]
    enable: [bool; 3],
    #[packed_field(bits = "9..=11", ty = "enum")]
    avg: Averages,
    #[packed_field(bits = "6..=8", ty = "enum")]
    vbus_ct: ConversionTime,
    #[packed_field(bits = "3..=5", ty = "enum")]
    vsh_ct: ConversionTime,
    #[packed_field(bits = "0..=2", ty = "enum")]
    mode: Mode,
}

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
