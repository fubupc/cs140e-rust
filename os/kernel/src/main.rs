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
use sd::command::CMD0;
use sd::host::reg::{self, NormalInterruptStatusEnable, PowerControl};
use sd::host::SDHost;
use sd::interface::Timer;

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

    let pin48 = pi::gpio::Gpio::new(48);

    let mut sd = SDHost::new(EMMC_BASE, fs::sd::SpinTimer);
    sd.set_debugger(&fs::sd::ConsoleDebugger);

    sd.reset_host().unwrap();
    // sd.card_detect().unwrap();
    sd.clock_supply(400_000).unwrap();
    sd.bus_power_control().unwrap();

    loop {}
}
