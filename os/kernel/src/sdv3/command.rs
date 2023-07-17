//! SD Commands to control the SD Memory Card
//!
//! All commands and responses are sent over the CMD line of the SD Memory Card.

#![allow(non_snake_case)]

use bitfield::bitfield;

use super::card::reg::CID;
use super::common::{SupplyVoltage, VoltageWindow, RCA};
use super::host::reg;
use super::response::{NoResponse, R1b, Response, R1, R2, R3, R6, R7};
use reg::CommandType::*;
use CommandType::*;

/// Trait represents *Command* (CMD/ACMD) concept in SD specification.
pub trait Command {
    /// Command Index
    const INDEX: u8;
    /// Command Type
    const TYPE: CommandType;

    /// Another *Command Type* (field of [`Command Register`](reg::Command) )
    const OPERATION: reg::CommandType = Normal;

    /// Corresponding Response type like [`R1`], [`R2`] for Command
    type RESPONSE: Response;

    /// Constructs command argument. Default value 0 means no argument.
    fn argument(&self) -> Argument {
        Argument(0) // No argument
    }

    /// FIXME: Is this correct?
    fn data_present() -> bool {
        match Self::TYPE {
            CommandType::ADTC => true,
            _ => false,
        }
    }
}

/// Command Types
///
/// There are four kinds of commands defined to control the SD Memory Card.
/// All commands and responses are sent over the CMD line of the SD Memory Card.
pub enum CommandType {
    /// Broadcast commands (bc), no response
    ///
    /// The broadcast feature is only if all the CMD lines are connected together in the host.
    /// If they are separated, then each card will accept it separately in its turn.
    BC,

    /// Broadcast commands with response (bcr), response from all cards simultaneously
    ///
    /// Since there is no Open Drain mode in SD Memory Card, this type of command shall be used
    /// only if all the CMD lines are separated - the command will be accepted and responded by
    /// every card separately.
    BCR,

    /// Addressed (point-to-point) commands (ac), no data transfer on DAT.
    AC,

    /// Addressed (point-to-point) data transfer commands (adtc), data transfer on DAT.
    ADTC,
}

/// Command Argument
#[derive(Debug, Copy, Clone)]
pub struct Argument(u32);
impl From<u32> for Argument {
    fn from(v: u32) -> Self {
        Argument(v)
    }
}
impl From<Argument> for u32 {
    fn from(v: Argument) -> Self {
        v.0
    }
}

// Standard Commands

/// CMD0 (GO_IDLE_STATE)
///
/// Resets all cards to idle state. When card supports boot functionalities and receives
/// this command as the first one in idle state after power up, the argument is regarded
/// as the bus mode in Fast Boot.
#[derive(Debug, Copy, Clone)]
pub struct CMD0;
impl Command for CMD0 {
    const INDEX: u8 = 0;
    const TYPE: CommandType = BC;
    type RESPONSE = NoResponse;
}

/// CMD2 (ALL_SEND_CID)
///
/// Asks any card to send the CID numbers on the CMD line (any card that is connected to the
/// host will respond)
#[derive(Debug, Copy, Clone)]
pub struct CMD2;
impl Command for CMD2 {
    const INDEX: u8 = 2;
    const TYPE: CommandType = BCR;
    type RESPONSE = R2<CID>;
}

/// CMD3 (SEND_RELATIVE_ADDR)
///
/// Ask the card to publish a new relative address ([`RCA`])
#[derive(Debug, Copy, Clone)]
pub struct CMD3;
impl Command for CMD3 {
    const INDEX: u8 = 3;
    const TYPE: CommandType = BCR;
    type RESPONSE = R6;
}

/// CMD7 (SELECT/DESELECT_CARD)
///
/// Command toggles a card between the stand-by and transfer states or between the programming
/// and disconnect states. In both cases, the card is selected by its own relative address and
/// gets deselected by any other address; address 0 deselects all.
///
/// In the case that the RCA equals 0, then the host may do one of the following:
/// - Use other RCA number to perform card de-selection.
/// - Re-send CMD3 to change its RCA number to other than 0 and then use CMD7 with RCA=0 for
///   card de-selection.
#[derive(Debug, Copy, Clone)]
pub struct CMD7(RCA);
impl Command for CMD7 {
    const INDEX: u8 = 7;
    const TYPE: CommandType = AC;
    type RESPONSE = R1b;

    fn argument(&self) -> Argument {
        Argument((u16::from(self.0) as u32) << 16)
    }
}

bitfield! {
    /// CMD8 (SEND_IF_COND)
    ///
    /// Sends SD Memory Card interface condition, which includes host supply voltage information and
    /// asks the card whether card supports voltage. Reserved bits shall be set to '0'.
    #[derive(Copy, Clone)]
    pub struct CMD8(u32);

    impl Debug;

    /// Host asks whether card supports VDD3 (1.2V power rail)
    ///
    /// - 0b: Not asking 1.2V support
    /// - 1b: Asking 1.2V support (VDD3 is supported by host. VDD3 shall be used if card supports it, too.)
    pub pcie_1_2v_support, set_pcie1_2v_support: 21;

    /// Host asks card’s PCIe availability
    ///
    /// - 0b: Not asking PCIe availability
    /// - 1b: Asking PCIe availability (PCIe interface is supported by host. PCIe interface shall be used if card
    /// supports it, too.)
    pub pcie_availability, set_pcie_availability: 20;

    /// Host Supplied Voltage (VHS)
    pub u8, from into SupplyVoltage, VHS, set_VHS: 19, 16;
}
impl Command for CMD8 {
    const INDEX: u8 = 8;
    const TYPE: CommandType = BCR;
    type RESPONSE = R7;

    fn argument(&self) -> Argument {
        Argument(self.0)
    }
}

/// CMD55 (APP CMD)
///
/// Indicates to the card that the next command is an application specific command rather than a
/// standard command.
#[derive(Debug, Copy, Clone)]
pub struct CMD55(RCA);
impl Command for CMD55 {
    const INDEX: u8 = 55;
    const TYPE: CommandType = AC;
    type RESPONSE = R1;

    fn argument(&self) -> Argument {
        Argument((u16::from(self.0) as u32) << 16)
    }
}

// Application-specific Commands

bitfield! {
    /// ACMD41 (SD_SEND_OP_COND)
    ///
    /// - Sends host capacity support information (HCS) and asks the accessed card to send its operating
    ///   condition register ([`OCR`](super::card::reg::OCR)) content in the response on the CMD line.
    /// - HCS is effective when card receives SEND_IF_COND command.
    /// - Sends request to switch to 1.8V signaling (S18R).
    /// - Reserved bit shall be set to '0'.
    /// - CCS bit is assigned to [`OCR`](super::card::reg::OCR)[30].
    /// - XPC controls the maximum current in the default speed mode of SDXC card:
    ///   - XPC=0 means 100mA (max.) but speed class is not supported
    ///   - XPC=1 means 150mA (max.) and speed class is supported.
    #[derive(Copy, Clone)]
    pub struct ACMD41(u32);

    impl Debug;

    /// Host Capacity Support ([`OCR`](super::card::reg::OCR)[30])
    ///
    /// - 0b: SDSC Only Host
    /// - 1b: SDHC or SDXC Supported
    pub HCS, set_HCS: 30;

    /// SDXC Power Control
    ///
    /// - 0b: Power Saving
    /// - 1b: Maximum Performance
    pub XPC, set_XPC: 28;

    /// S18R : Switching to 1.8V Request
    ///
    /// - 0b: Use current signal voltage
    /// - 1b: Switch to 1.8V signal voltage
    pub S18R, set_S18R: 24;

    /// VDD Voltage Window ([`OCR`](super::card::reg::OCR)[23:8])
    pub u32, from into VoltageWindow, voltage_window, set_voltage_window: 23, 8;
}
impl Command for ACMD41 {
    const INDEX: u8 = 41;
    const TYPE: CommandType = BCR;
    type RESPONSE = R3;

    fn argument(&self) -> Argument {
        Argument(self.0)
    }
}
