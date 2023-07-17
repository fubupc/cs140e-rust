//! Card specific concepts and types

pub mod reg;

use core::ops::Deref;

use super::common::OpMode;
use reg::CSR;

/// Physical Layer Specification Version
#[derive(Debug)]
pub enum SDSpec {
    /// Version 1.01
    V1_01,
    /// Version 1.10
    V1_10,
    /// Version 2.00
    V2_00,
    /// Version 3.00
    V3_00,
    Unknown,
}

/// Card State
#[derive(Debug)]
pub enum CardState {
    Inactive,
    Idle,
    Ready,
    Identification,
    StandBy,
    Transfer,
    SendingData,
    ReceiveData,
    Programming,
    Disconnect,
}
impl CardState {
    pub fn mode(&self) -> OpMode {
        match self {
            CardState::Inactive => OpMode::Inactive,

            CardState::Idle => OpMode::CardIdentification,
            CardState::Ready => OpMode::CardIdentification,
            CardState::Identification => OpMode::CardIdentification,

            CardState::StandBy => OpMode::DataTransfer,
            CardState::Transfer => OpMode::DataTransfer,
            CardState::SendingData => OpMode::DataTransfer,
            CardState::ReceiveData => OpMode::DataTransfer,
            CardState::Programming => OpMode::DataTransfer,
            CardState::Disconnect => OpMode::DataTransfer,
        }
    }
}

/// Card Status
///
/// Error and state information of a executed command, indicated in the response.
#[derive(Debug, Clone, Copy)]
pub struct CardStatus(CSR);
impl Deref for CardStatus {
    type Target = CSR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
