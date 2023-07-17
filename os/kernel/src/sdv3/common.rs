//! Common types for both Card and Host

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bitfield::bitfield;

/// Operation Mode
///
/// Two operation modes are defined for the SD Memory Card system (host and cards):
/// - Card identification mode
///   The host will be in card identification mode after reset and while it is looking for new cards on the
///   bus. Cards will be in this mode after reset until the SEND_RCA command (CMD3) is received.
/// - Data transfer mode
///   Cards will enter data transfer mode after their RCA is first published. The host will enter data transfer
///   mode after identifying all the cards on the bus.
#[derive(Debug)]
pub enum OpMode {
    Inactive,
    CardIdentification,
    DataTransfer,
}

/// Relative Card Address
//
/// Local system address of a card, dynamically suggested by the card and approved by the
/// host during initialization. (Not available in SPI mode)
#[derive(Debug, Clone, Copy)]
pub struct RCA(u16);
impl From<u16> for RCA {
    fn from(v: u16) -> Self {
        RCA(v)
    }
}
impl From<RCA> for u16 {
    fn from(v: RCA) -> Self {
        v.0
    }
}

/// Supply Voltage
///
/// In terms of operating supply voltage, two types of SD Memory Cards are defined:
/// - High Voltage SD Memory Cards that can operate the voltage range of 2.7-3.6 V.
#[derive(Debug)]
pub enum SupplyVoltage {
    NotDefined = 0b0000,
    HighVoltage = 0b0001, // 2.7-3.6V
    LowVoltage = 0b0010,  // Reserved for Low Voltage Range
    Reserved,
}
impl From<SupplyVoltage> for u8 {
    fn from(v: SupplyVoltage) -> Self {
        v as u8
    }
}
impl From<u8> for SupplyVoltage {
    fn from(v: u8) -> Self {
        match v {
            0b0000 => Self::NotDefined,
            0b0001 => Self::HighVoltage,
            0b0010 => Self::LowVoltage,
            _ => Self::Reserved,
        }
    }
}

bitfield! {
    /// VDD Voltage Window
    #[derive(Clone, Copy)]
    pub struct VoltageWindow(u32);

    impl Debug;

    pub _3_5_to_3_6, _: 23; // 3.5-3.6
    pub _3_4_to_3_5, _: 22; // 3.4-3.5
    pub _3_3_to_3_4, _: 21; // 3.3-3.4
    pub _3_2_to_3_3, _: 20; // 3.2-3.3
    pub _3_1_to_3_2, _: 19; // 3.1-3.2
    pub _3_0_to_3_1, _: 18; // 3.0-3.1
    pub _2_9_to_3_0, _: 17; // 2.9-3.0
    pub _2_8_to_2_9, _: 16; // 2.8-2.9
    pub _2_7_to_2_8, _: 15; // 2.7-2.8

    // [14:8] reserved

    // [7:0] reserved for Low Voltage Range
}
impl From<u32> for VoltageWindow {
    fn from(v: u32) -> Self {
        VoltageWindow(v)
    }
}
impl From<VoltageWindow> for u32 {
    fn from(v: VoltageWindow) -> Self {
        v.0
    }
}

/// DAT Bus Width
#[derive(Debug)]
pub enum BusWidth {
    _1Bit,
    _4Bit,
    _8Bit, // 8-bit Support for Embedded Device
}

/// Check Pattern
#[derive(Debug)]
pub struct CheckPattern(u8);
impl CheckPattern {
    const DEFAULT: u8 = 0b10101010;
}
impl From<CheckPattern> for u8 {
    fn from(v: CheckPattern) -> Self {
        v.0
    }
}
impl From<u8> for CheckPattern {
    fn from(v: u8) -> Self {
        CheckPattern(v)
    }
}
