#![no_std]
#![cfg_attr(not(test), no_main)]
#![allow(unused)]
#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![feature(prelude_import)]

#[cfg(all(test, feature = "custom-std"))]
compile_error!(
    "feature \"custom-std\" cannot be enabled when test (which depends on built-in `std`)"
);

#[cfg(not(feature = "custom-std"))]
#[macro_use]
extern crate std;
#[cfg(feature = "custom-std")]
#[macro_use]
extern crate custom_std as std;

#[prelude_import]
use std::prelude::v1::*;

#[macro_use]
extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
#[cfg(feature = "custom-std")]
pub mod lang_items;
pub mod mutex;
pub mod shell;

use core::{arch::global_asm, time::Duration};
#[cfg(not(test))]
global_asm!(include_str!("../ext/init.S"));

#[cfg(not(test))]
use allocator::Allocator;
use console::kprintln;
use fs::FileSystem;
use sd::spec::{
    command::CMD0,
    host::{
        reg::{self, NormalInterruptStatusEnable},
        SDHost,
    },
    timer::Timer,
};

#[cfg(not(test))]
pub static _ALLOCATOR: Allocator = Allocator::uninitialized();
#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: &Allocator = &_ALLOCATOR;

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

const EMMC_BASE: usize = pi::common::IO_BASE + 0x00300000;

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    #[cfg(not(test))]
    ALLOCATOR.initialize();

    let mut uart = pi::uart::MiniUart::new();

    let mut sd = SDHost::new(EMMC_BASE, fs::sd::SpinTimer);

    kprintln!("== RegMap ==");
    // kprintln!("{:#?}", { sd.regmap().SlotInterruptStatus });
    // kprintln!("{:#?}", { sd.regmap().HostControllerVersion });
    // kprintln!("{:#?}", { sd.regmap().PresentState });
    // kprintln!("{:#?}", { sd.regmap().NormalInterruptStatus });
    // kprintln!("{:#?}", { sd.regmap().NormalInterruptStatusEnable });
    kprintln!("{:#?}", { sd.regmap().HostControl1 });
    kprintln!("{:#?}", { sd.regmap().PowerControl });
    kprintln!("{:#?}", { sd.regmap().ClockControl });
    kprintln!("{:#?}", { sd.regmap().HostControl2 });
    // kprintln!("{:#?}", cap);

    kprintln!("Press any key to set NormalInterruptStatusEnable");
    uart.wait_for_byte();

    let mut status_enable = NormalInterruptStatusEnable(0);
    status_enable.set_card(true);
    status_enable.set_card_insertion(true);
    status_enable.set_card_removal(true);
    status_enable.set_transfer_complete(true);
    status_enable.set_command_complete(true);

    sd.regmap().NormalInterruptStatusEnable = status_enable;
    kprintln!("{:#?}", { sd.regmap().NormalInterruptStatusEnable });
    kprintln!("{:#?}", { sd.regmap().NormalInterruptStatus });

    kprintln!("Press any key to send CMD0");
    uart.wait_for_byte();
    let r = sd.issue_cmd(CMD0);

    kprintln!("After CMD0");
    kprintln!("{:#?}", { sd.regmap().Command });
    kprintln!("{:#?}", { sd.regmap().NormalInterruptStatusEnable });
    kprintln!("{:#?}", { sd.regmap().NormalInterruptStatus });

    kprintln!("Press any key to reset Host Controller");
    uart.wait_for_byte();
    let ctrl_ptr = core::ptr::addr_of_mut!(sd.regmap().ClockControl) as *mut u32;
    kprintln!("R: {:#?}", sd.regmap().SoftwareReset);
    kprintln!("R(ptr): {:#X}", core::ptr::read_volatile(ctrl_ptr));
    let mut r = reg::SoftwareReset(0);
    r.set_srst_all(true);
    kprintln!("W: {:#?}", r);
    core::ptr::write_volatile(ctrl_ptr, (r.0 as u32) << 24);

    // sd.timer().wait_for(
    //     || !sd.regmap().SoftwareReset.srst_all(),
    //     Duration::from_millis(1000),
    // );
    kprintln!("R: {:#?}", sd.regmap().SoftwareReset);
    kprintln!("R(ptr): {:#X}", core::ptr::read_volatile(ctrl_ptr));
    kprintln!("{:#?}", { sd.regmap().PresentState });

    loop {}
}
