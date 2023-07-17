//! Card Registers

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bitfield::bitfield;
use core::{fmt, panic};

use super::super::common::{self, BusWidth, VoltageWindow};
use super::{CardState, SDSpec};

bitfield! {
    /// Operation Conditions Register
    #[derive(Copy, Clone)]
    pub struct OCR(u32);

    impl Debug;

    /// Card power up status bit (busy)
    ///
    /// This bit is set if the card power up preceduer has been finished.
    pub card_power_up_status, _: 31;

    /// Card Capacity Status (CCS)
    ///
    /// This bit is valid after the card power up procedure is completed and the card power up status bit is set to 1.
    ///
    /// - 0 indicates that the card is SDSC.
    /// - 1 indicates that the card is SDHC/SDXC.
    pub u8, into CCS, CCS, _: 30, 30;

    // [29:25] reserved

    /// Switching to 1.8V Accepted (S18A)
    pub S18A, _: 26;

    /// VDD Voltage Window
    pub u32, into VoltageWindow, voltage_window, _: 23, 0;
}

/// Card Capacity Status
#[derive(Debug)]
pub enum CCS {
    SDSC = 0,
    Other = 1, // SDHC/SDXC
}
impl From<u8> for CCS {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::SDSC,
            1 => Self::Other,
            _ => unreachable!(),
        }
    }
}

bitfield! {
    /// Card Identification Register
    #[derive(Copy, Clone)]
    pub struct CID(u128);

    impl Debug;

    /// Manufacturer ID
    pub u8, MID, _: 127, 120;

    /// OEM/Application ID
    pub u16, OID, _: 119, 104;

    /// Product name
    pub u64, PNM, _: 103, 64;

    /// Product revision
    pub u8, PRV, _: 63, 56;

    /// Product serial number
    pub u32, PSN, _: 55, 24;

    // [23:20] reserved

    /// Manufacturing date
    pub u8, MDT, _: 19, 18;

    /// CRC7 checksum
    pub u8, CRC, _: 7, 1;

    // [0:0] not used, always 1
}

/// Card-Specific Data Register
///
/// The Card-Specific Data register provides information regarding access to the card contents. The CSD
/// defines the data format, error correction type, maximum data access time, whether the DSR register can
/// be used, etc. The programmable part of the register can be changed by CMD27.
///
/// Field structures of the CSD register are different depend on the Physical Layer Specification Version
/// and Card Capacity.
#[derive(Clone, Copy)]
pub union CSD {
    v1: CSDv1,
    v2: CSDv2,
}
impl CSD {
    fn version(&self) -> CSDVersion {
        unsafe { self.v1.CSD_STRUCTURE() }
    }
}
impl From<u128> for CSD {
    fn from(v: u128) -> Self {
        CSD { v1: CSDv1(v) }
    }
}
impl fmt::Debug for CSD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.version() {
            CSDVersion::V1 => f.write_fmt(format_args!("CSDv1: {:?}", unsafe { self.v1 })),
            CSDVersion::V2 => f.write_fmt(format_args!("CSDv2: {:?}", unsafe { self.v2 })),
            CSDVersion::Reserved => f.write_str("Reserved CSD"),
        }
    }
}

/// CSD Structure Version
#[derive(Debug, Clone, Copy)]
pub enum CSDVersion {
    V1 = 0, // Standard Capacity
    V2 = 1, // High Capacity and Extended Capacity
    Reserved,
}
impl From<u8> for CSDVersion {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::V1,
            1 => Self::V2,
            _ => Self::Reserved,
        }
    }
}

bitfield! {
    /// CSD Version 1.0
    #[derive(Clone, Copy)]
    pub struct CSDv1(u128);

    impl Debug;

    /// CSD structure version
    pub u8, from into CSDVersion, CSD_STRUCTURE, _: 127, 126;

    // [125:120] reserved

    /// Data read access-time-1
    pub u8, TAAC, _: 119, 112;

    /// Data read access-time-2 in CLK cycles (NSAC*100)
    pub u8, NSAC, _: 111, 104;

    /// Max. data transfer rate
    pub u8, TRAN_SPEED, _: 103, 96;

    /// Card command classes
    pub u16, CCC, _: 95, 84;

    /// Max. read data block length
    pub u8, READ_BL_LEN, _: 83, 80;

    /// Partial blocks for read allowed
    pub READ_BL_PARTIAL, _: 79;

    /// Write block misalignment
    pub WRITE_BLK_MISALIGN, _: 78;

    /// Read block misalignment
    pub READ_BLK_MISALIGN, _: 77;

    // DSR implemented
    pub DSR_IMP, _: 76;

    // [75:74] reserved

    /// Device size
    pub u16, C_SIZE, _: 73, 62;

    /// Max. read current @VDD min
    pub u8, VDD_R_CURR_MIN, _: 61, 59;

    /// Max. read current @VDD max
    pub u8, VDD_R_CURR_MAX, _: 58, 56;

    /// Max. write current @VDD min
    pub u8, VDD_W_CURR_MIN, _: 55, 53;

    /// Max. write current @VDD max
    pub u8, VDD_W_CURR_MAX, _: 52, 50;

    /// Device size multiplier
    pub u8, C_SIZE_MULT, _: 49, 47;

    /// Erase single block enable
    pub ERASE_BLK_EN, _: 46;

    /// Erase sector size
    pub u8, SECTOR_SIZE, _: 45, 39;

    /// Write protect group size
    pub u8, WP_GRP_SIZE, _: 38, 32;

    /// Write protect group enable
    pub WP_GRP_ENABLE, _: 31;

    // [30:29] reserved

    /// Write speed factor
    pub u8, R2W_FACTOR, _: 28, 26;

    /// Max. write data block length
    pub u8, WRITE_BL_LEN, _: 25, 22;

    /// Partial blocks for write allowed
    pub WRITE_BL_PARTIAL, _: 21;

    // [20:16] reserved

    /// File format group
    pub FILE_FORMAT_GRP, _: 15;

    /// Copy flag
    pub COPY, _: 14;

    /// Permanent write protection
    pub PERM_WRITE_PROTECT, _: 13;

    /// Temporary write protection
    pub TMP_WRITE_PROTECT, _: 12;

    /// File format
    pub u8, FILE_FORMAT, _: 11, 10;

    // [9:8] reserved

    /// CRC
    pub u8, CRC, _: 7, 1;

    // [0;0] not used, always'1'
}

bitfield! {
    /// CSD Version 2.0
    #[derive(Clone, Copy)]
    pub struct CSDv2(u128);

    impl Debug;

    /// CSD structure
    pub u8, from into CSDVersion, CSD_STRUCTURE, _: 127, 126;

    // [125:120] reserved

    /// Data read access-time-1
    ///
    /// This field is fixed to 0Eh, which indicates 1 ms.
    pub u8, TAAC, _: 119, 112;

    /// Data read access-time-2 in CLK cycles (NSAC*100)
    ///
    /// This field is fixed to 00h.
    pub u8, NSAC, _: 111, 104;

    /// Max. data transfer rate
    pub u8, TRAN_SPEED, _: 103, 96;

    /// Card command classes
    pub u16, CCC, _: 95, 84;

    /// Max. read data block length
    /// This field is fixed to 9h, which indicates READ_BL_LEN=512 Byte.
    pub u8, READ_BL_LEN, _: 83, 80;

    /// Partial blocks for read allowed
    ///
    /// This field is fixed to 0, which indicates partial block read is inhibited and only unit of block access is allowed.
    pub READ_BL_PARTIAL, _: 79;

    /// Write block misalignment
    ///
    /// This field is fixed to 0, which indicates that write access crossing physical block boundaries is always disabled in
    /// SDHC and SDXC Cards.
    pub WRITE_BLK_MISALIGN, _: 78;

    /// Read block misalignment
    ///
    /// This field is fixed to 0, which indicates that read access crossing physical block boundaries is always disabled in
    /// SDHC and SDXC Cards.
    pub READ_BLK_MISALIGN, _: 77;

    /// DSR implemented
    pub DSR_IMP, _: 76;

    // [75:70] reserved

    /// Device size
    ///
    /// This field is expanded to 22 bits and can indicate up to 2 TBytes.
    pub u32, C_SIZE, _: 69, 48; /// Different between version

    // [47:47] reserved

    /// Erase single block enable
    ///
    /// This field is fixed to 1, which means the host can erase one or multiple units of 512 bytes.
    pub ERASE_BLK_EN, _: 46;

    /// Erase sector size
    ///
    /// This field is fixed to 7Fh, which indicates 64 KBytes.
    pub u8, SECTOR_SIZE, _: 45, 39;

    /// Write protect group size
    ///
    /// This field is fixed to 00h. SDHC and SDXC Cards do not support write protected groups.
    pub u8, WP_GRP_SIZE, _: 38, 32;

    /// Write protect group enable
    ///
    /// This field is fixed to 0. SDHC and SDXC Cards do not support write protected groups.
    pub WP_GRP_ENABLE, _: 31;

    // [30:29] reserved

    /// Write speed factor
    ///
    /// This field is fixed to 2h, which indicates 4 multiples.
    pub u8, R2W_FACTOR, _: 28, 26;

    /// Max. write data block length
    ///
    /// This field is fixed to 9h, which indicates WRITE_BL_LEN=512 Byte.
    pub u8, WRITE_BL_LEN, _: 25, 22;

    /// Partial blocks for write allowed
    ///
    /// This field is fixed to 0, which indicates partial block read is inhibited and only unit of block access is allowed.
    pub WRITE_BL_PARTIAL, _: 21;

    // [20:16] reserved

    /// File format group
    ///
    /// This field is set to 0. Host should not use this field.
    pub FILE_FORMAT_GRP, _: 15;

    /// Copy flag
    pub COPY, _: 14;

    /// Permanent write protection
    pub PERM_WRITE_PROTECT, _: 13;

    /// Temporary write protection
    pub TMP_WRITE_PROTECT, _: 12;

    /// File format
    ///
    /// This field is set to 0. Host should not use this field.
    pub u8, FILE_FORMAT, _: 11, 10;

    // [9:8] reserved

    /// CRC
    pub u8, CRC, _: 7, 1;

    // [0;0] not used, always'1'
}

/// RCA register
///
/// The writable 16-bit relative card address register carries the card address that is published by the card
/// during the card identification. This address is used for the addressed host-card communication after the
/// card identification procedure. The default value of the RCA register is 0x0000. The value 0x0000 is
/// reserved to set all cards into the Stand-by State with CMD7.
#[derive(Debug, Clone, Copy)]
pub struct RCA(common::RCA);
impl From<common::RCA> for RCA {
    fn from(v: common::RCA) -> Self {
        RCA(v)
    }
}
impl From<RCA> for common::RCA {
    fn from(v: RCA) -> Self {
        common::RCA::from(v.0)
    }
}

bitfield! {
    /// Driver Stage Register (Optional)
    ///
    /// To configure the card's output drivers.
    #[derive(Clone, Copy)]
    pub struct DSR(u16);

    impl Debug;

    // TODO: implement fields
}

/// SD CARD Configuration Register
///
/// Information about the SD Memory Card's special features capabilities.
#[derive(Clone, Copy)]
pub union SCR {
    v1: SCRv1,
}
impl SCR {
    fn version(&self) -> SCRVersion {
        unsafe { self.v1.SCR_STRUCTURE() }
    }
}
impl fmt::Debug for SCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.version() {
            SCRVersion::V1 => f.write_fmt(format_args!("SCRv1: {:?}", unsafe { self.v1 })),
            SCRVersion::Reserved => f.write_str("Reserved SCR"),
        }
    }
}

/// SCR Register Structure Version
#[derive(Debug)]
pub enum SCRVersion {
    V1 = 0,
    Reserved,
}
impl From<u8> for SCRVersion {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::V1,
            _ => Self::Reserved,
        }
    }
}

bitfield! {
    /// SCR Version 1.0
    #[derive(Clone, Copy)]
    pub struct SCRv1(u64);

    impl Debug;

    /// SCR Structure
    pub u8, into SCRVersion, SCR_STRUCTURE, _: 63, 60;

    /// SD Memory Card - Spec. Version
    u8, SD_SPEC, _: 59, 56;

    /// Data_Status_After erases
    pub DATA_STAT_AFTER_ERASE, _: 55;

    /// CPRM Security Support
    pub u8, SD_SECURITY, _: 54, 52;

    /// DAT Bus widths supported
    pub _1bit_bus_support, _: 50;
    pub _4bit_bus_support, _: 48;

    /// Spec. Version 3.00 or higher
    u8, SD_SPEC3, _: 47, 47;

    /// Extended Security Support
    pub u8, EX_SECURITY, _: 46, 43;

    // [42:34] Reserved

    /// Command Support bits
    pub u8, CMD_SUPPORT, _: 33, 32;

    // [31:0] reserved for manufacturer usage
}
impl SCRv1 {
    pub fn sd_spec(&self) -> SDSpec {
        match (self.SD_SPEC(), self.SD_SPEC3()) {
            (0, 0) => SDSpec::V1_01,
            (1, 0) => SDSpec::V1_10,
            (2, 0) => SDSpec::V2_00,
            (2, 1) => SDSpec::V3_00,
            _ => SDSpec::Unknown,
        }
    }
}

/// SD Status Register
///
/// Information about the card proprietary features
#[derive(Debug)]
pub struct SSR([u8; 512]);

bitfield! {
    /// Card Status Register
    #[derive(Clone, Copy)]
    pub struct CSR(u32);

    impl Debug;

    /// The command's argument was out of the allowed range for this card.
    pub OUT_OF_RANGE, _: 31;

    /// A misaligned address which did not match the block length was used in the command.
    pub ADDRESS_ERROR, _: 30;

    /// The transferred block length is not allowed for this card, or the number of transferred
    /// bytes does not match the block length.
    pub BLOCK_LEN_ERROR, _: 29;

    /// An error in the sequence of erase commands occurred.
    pub ERASE_SEQ_ERROR, _: 28;

    /// An invalid selection of write-blocks for erase occurred.
    pub ERASE_PARAM, _: 27;

    /// Set when the host attempts to write to a protected block or to the temporary write protected
    /// card or write protected until power cycle card or permanent write protected card.
    pub WP_VIOLATION, _: 26;

    /// When set, signals that the card is locked by the host.
    pub CARD_IS_LOCKED, _: 25;

    /// Set when a sequence or password error has been detected in lock/unlock card command.
    pub LOCK_UNLOCK_FAILED, _: 24;

    /// The CRC check of the previous command failed.
    pub COM_CRC_ERROR, _: 23;

    /// Command not legal for the card state.
    pub ILLEGAL_COMMAND, _: 22;

    /// Card internal ECC was applied but failed to correct the data.
    pub CARD_ECC_FAILED, _: 21;

    /// Internal card controller error
    pub CC_ERROR, _: 20;

    /// A general or an unknown error occurred during the operation.
    pub ERROR, _: 19;

    // [18:17] reserved

    /// Can be either one of the following errors:
    // - The read only section of the CSD does not match the card content.
    // - An attempt to reverse the copy (set as original) or permanent WP (unprotected) bits was made.
    pub CSD_OVERWRITE, _: 16;

    /// Set when only partial address space was erased due to existing write protected blocks or the temporary
    /// write protected or write protected until power cycle or permanent write protected card was erased.
    pub WP_ERASE_SKIP, _: 15;

    /// The command has been executed without using the internal ECC.
    pub CARD_ECC_DISABLED, _: 14;

    /// An erase sequence was cleared before executing because an out of erase sequence command was received.
    pub ERASE_RESET, _: 13;

    /// CurrentState:
    /// The state of the card when receiving the command. If the command execution causes a state change, it will
    /// be visible to the host in the response to the next command. The four bits are interpreted as a binary coded
    /// number between 0 and 15.
    pub u8, into CurrentState, CURRENT_STATE, _: 12, 9;

    /// Corresponds to buffer empty signaling on the bus
    pub READY_FOR_DATA, _: 8;

    // [7:7] reserved

    /// Extension Functions may set this bit to get host to deal with events.
    pub FX_EVENT, _: 6;

    /// The card will expect ACMD, or an indication that the command has been interpreted as ACMD
    pub APP_CMD, _: 5;

    // [4:4] reserved for SD I/O Card

    /// Error in the sequence of the authentication process
    pub AKE_SEQ_ERROR, _: 3;

    // [2:2] reserved for application specific commands

    // [1:0] reserved for manufacturer test mode
}

/// Encodes a subset of [`CardState`], excluding the Inactive state.
#[derive(Debug)]
pub struct CurrentState(u8);
impl From<u8> for CurrentState {
    fn from(v: u8) -> Self {
        CurrentState(v)
    }
}
impl From<CurrentState> for CardState {
    fn from(v: CurrentState) -> Self {
        match v.0 {
            0 => CardState::Idle,
            1 => CardState::Ready,
            2 => CardState::Identification,
            3 => CardState::StandBy,
            4 => CardState::Transfer,
            5 => CardState::SendingData,
            6 => CardState::ReceiveData,
            7 => CardState::Programming,
            8 => CardState::Disconnect,
            _ => panic!("unknown CURRENT_STATE"),
        }
    }
}
