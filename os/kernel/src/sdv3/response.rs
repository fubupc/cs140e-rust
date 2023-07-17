//! Responses
//!
//! All responses are sent via the command line CMD.
//!
//! There are five types of responses for the SD Memory Card. The SDIO Card supports additional response
//! types named R4 and R5.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::fmt::Debug;

use bitfield::bitfield;

use super::card::reg::{CurrentState, CID, CSD, CSR, OCR};
use super::common::{CheckPattern, SupplyVoltage, RCA};
use super::host::reg::{self, ResponseType};
use ResponseType::*;

/// Trait represents *Response* concept in SD specification
pub trait Response {
    const TYPE: ResponseType;
    const COMMAND_INDEX_CHECK: bool;
    const COMMAND_CRC_CHECK: bool;

    // Read response from [`Response Register`](reg::Response)
    fn read(_: reg::Response) -> Self;
}

/// A special response type means actually no response
#[derive(Debug, Copy, Clone)]
pub struct NoResponse;
impl Response for NoResponse {
    const TYPE: ResponseType = ResponseType::NoResponse;
    const COMMAND_INDEX_CHECK: bool = false;
    const COMMAND_CRC_CHECK: bool = false;

    fn read(_: reg::Response) -> Self {
        NoResponse
    }
}

/// R1 (normal response command): 48 bits
#[derive(Debug, Copy, Clone)]
pub struct R1(pub CSR);
impl Response for R1 {
    const TYPE: ResponseType = _48Bits;
    const COMMAND_INDEX_CHECK: bool = true;
    const COMMAND_CRC_CHECK: bool = true;

    /// R1, R1b (normal response) bit range: [31:0]
    ///
    /// TODO: R1 (Auto CMD23 response) bit range should be [127:96]
    fn read(r: reg::Response) -> Self {
        R1(CSR(r.bit_31_0()))
    }
}

/// R1b is identical to R1 with an optional busy signal transmitted on the data line
#[derive(Debug, Copy, Clone)]
pub struct R1b(pub CSR);
impl Response for R1b {
    const TYPE: ResponseType = _48BitsBusy;
    const COMMAND_INDEX_CHECK: bool = true;
    const COMMAND_CRC_CHECK: bool = true;

    /// R1, R1b (normal response) bit range: [31:0]
    ///
    /// TODO: R1b (Auto CMD12 response) bit range should be [127:96]
    fn read(r: reg::Response) -> Self {
        R1b(CSR(r.bit_31_0()))
    }
}

/// R2 (CID, CSD register): 136 bits
#[derive(Debug, Copy, Clone)]
pub struct R2<I: R2Inner>(I);
impl Response for R2<CID> {
    const TYPE: ResponseType = _48BitsBusy;
    const COMMAND_INDEX_CHECK: bool = true;
    const COMMAND_CRC_CHECK: bool = true;

    /// R2 (CID register) bit range [119:0]
    fn read(r: reg::Response) -> Self {
        R2(CID(r.bit_119_0()))
    }
}
impl Response for R2<CSD> {
    const TYPE: ResponseType = _48BitsBusy;
    const COMMAND_INDEX_CHECK: bool = true;
    const COMMAND_CRC_CHECK: bool = true;

    /// R2 (CSD register) bit range [119:0]
    fn read(r: reg::Response) -> Self {
        R2(CSD::from(r.bit_119_0()))
    }
}

/// Contraint content type of R2: CID or CSD
pub trait R2Inner {}
impl R2Inner for CID {}
impl R2Inner for CSD {}

/// R3 (OCR register): 48 bits
#[derive(Debug, Copy, Clone)]
pub struct R3(pub OCR);
impl Response for R3 {
    const TYPE: ResponseType = _48Bits;
    const COMMAND_INDEX_CHECK: bool = false;
    const COMMAND_CRC_CHECK: bool = false;

    /// R3 (OCR register) bit range [31:0]
    fn read(r: reg::Response) -> Self {
        R3(OCR(r.bit_31_0()))
    }
}

// TODO: R4 (for SDIO)

// TODO: R5 (for SDIO)

bitfield! {
    /// R6 (Published RCA response): 32 bits (48 bits [39:8])
    #[derive(Copy, Clone)]
    pub struct R6(u32);

    impl Debug;

    pub u16, from into RCA, published_rca, _: 31, 16;

    // [15:0] card status bits: 23, 22, 19, [12:0]:

    /// CardStatus[23] The CRC check of the previous command failed.
    pub COM_CRC_ERROR, _: 15;

    /// CardStatus[22] Command not legal for the card state.
    pub ILLEGAL_COMMAND, _: 14;

    /// CardStatus[19] A general or an unknown error occurred during the operation.
    pub ERROR, _: 13;

    /// CardStatus[12:9]
    pub u8, into CurrentState, CURRENT_STATE, _: 12, 9;

    // [8:0] not used?
}
impl Response for R6 {
    const TYPE: ResponseType = _48Bits;
    const COMMAND_INDEX_CHECK: bool = true;
    const COMMAND_CRC_CHECK: bool = true;

    /// R6 (Published RCA response) bit range [31:0]
    fn read(r: reg::Response) -> Self {
        R6(r.bit_31_0())
    }
}

bitfield! {
    /// R7 (Card interface condition): 32 bits (48 bits [39:8])
    #[derive(Copy, Clone)]
    pub struct R7(u32);

    impl Debug;

    // [31:14] reserved

    /// Card responds whether it supports VDD3 (1.2V power rail)
    ///
    /// - 0b: Not supporting 1.2V
    /// - 1b: Supporting 1.2V
    pub pcie_1_2v_support, _: 13;

    /// Card responds PCIe acceptance
    ///
    /// - 0b: Not accepted
    /// - 1b: Accepted
    pub pcie_accepted, _: 12;

    /// Card Accepted Voltage (VCA)
    pub u8, into SupplyVoltage, VCA, _ : 11, 8;

    /// Echo-back of check pattern
    pub u8, into CheckPattern, check_pattern, _: 7, 0;
}
impl Response for R7 {
    const TYPE: ResponseType = _48Bits;
    const COMMAND_INDEX_CHECK: bool = true;
    const COMMAND_CRC_CHECK: bool = true;

    /// R7 bit range [31:0]
    fn read(r: reg::Response) -> Self {
        R7(r.bit_31_0())
    }
}
