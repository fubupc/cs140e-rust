//! Host Controller specific

// pub mod meta;
pub mod reg;

use core::marker::PhantomData;
use core::time::Duration;

use self::reg::RegMap;
use super::command::Command;
use super::command::{CMD0, CMD8};
use super::common::SupplyVoltage;
use super::response::Response;
use super::timer::Timer;

// States
pub enum Uninitialized {}
pub enum Idle {}
pub enum CardIdentification {}
pub enum StandBy {}

pub trait State {}
impl State for Uninitialized {}
impl State for Idle {}
impl State for CardIdentification {}
impl State for StandBy {}

pub struct SDHost<S: State, T: Timer> {
    regmap: &'static mut reg::RegMap,
    timer: T,
    _state: PhantomData<S>,
}

impl<S: State, T: Timer> SDHost<S, T> {
    pub fn issue_cmd<C: Command<RESPONSE = R>, R: Response>(&mut self, c: C) -> R {
        let mut cmd = reg::Command(0);
        cmd.set_command_index(C::INDEX);
        cmd.set_command_type(C::OPERATION);
        cmd.set_command_index_check(R::COMMAND_INDEX_CHECK);
        cmd.set_command_crc_check(R::COMMAND_CRC_CHECK);
        cmd.set_response_type(R::TYPE);
        cmd.set_data_present(C::data_present());

        // TODO: process data present field

        let arg1_ptr = core::ptr::addr_of_mut!(self.regmap.Argument1);
        let cmd_ptr = core::ptr::addr_of_mut!(self.regmap.Command);
        let resp_ptr = core::ptr::addr_of!(self.regmap.Response);
        unsafe {
            core::ptr::write_volatile(arg1_ptr, c.argument().into());
            core::ptr::write_volatile(cmd_ptr, cmd);

            R::read(core::ptr::read_volatile(resp_ptr))
        }
    }

    pub fn regmap(&mut self) -> &mut RegMap {
        self.regmap
    }

    pub fn timer(&self) -> &T {
        &self.timer
    }

    fn transition<S2: State>(self) -> SDHost<S2, T> {
        SDHost {
            regmap: self.regmap,
            timer: self.timer,
            _state: PhantomData,
        }
    }
}

impl<T: Timer> SDHost<Uninitialized, T> {
    pub fn new(base_addr: usize, timer: T) -> SDHost<Uninitialized, T> {
        SDHost {
            regmap: unsafe { &mut *(base_addr as *mut RegMap) },
            timer: timer,
            _state: PhantomData,
        }
    }

    /// Reset Host Controller
    pub fn reset_host(&mut self) -> Result<Duration, ()> {
        let mut r = reg::SoftwareReset(0);
        r.set_srst_all(true);
        self.regmap.SoftwareReset = r;

        self.timer.wait_for(
            || !self.regmap.SoftwareReset.srst_all(),
            Duration::from_millis(1000),
        )

        // wait for reset to finish

        // self.issue_cmd(CMD0);
        // self.transition()
    }
}

// impl SDHost<Idle> {
//     pub fn check_voltage(mut self) {
//         let mut cmd = CMD8(0);
//         cmd.set_VHS(SupplyVoltage::HighVoltage);
//         let resp = self.issue_cmd(cmd);
//     }
// }
