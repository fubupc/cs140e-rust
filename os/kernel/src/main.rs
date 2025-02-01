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
use pi::uart::MiniUart;
use sd::command::CMD0;
use sd::host::reg::{self, NormalInterruptStatusEnable, PowerControl};
use sd::host::SDHost;
use sd::interface::Timer;

use crate::console::kprint;

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

    let mut sd = SDHost::new(EMMC_BASE, fs::sd::PiTimer);

    let (version, caps) = sd.info();
    kprintln!("=== Host Version & Capabilities ===");
    kprintln!("{:?}", version);
    kprintln!("{:?}", caps);
    kprintln!("===================================");

    sd.set_debugger(&fs::sd::ConsoleDebugger);

    sd.dump_regs();
    MiniUart::new().wait_for_byte().unwrap();

    sd.reset_host().unwrap();
    sd.detect_card().unwrap();
    sd.supply_clock(400_000).unwrap();
    sd.control_bus_power().unwrap();
    sd.set_timeout_on_dat(Duration::from_secs(1)).unwrap();

    sd.enable_all_interrupts();

    sd.dump_regs();
    MiniUart::new().wait_for_byte().unwrap();

    let r = sd.card_init_and_ident();
    sd.dump_regs();
    kprintln!("Card init result: {:?}", r);

    loop {}
}

fn setup_sd_host_gpio() {
    let pin22 = pi::gpio::Gpio::new(22);
    let pin23 = pi::gpio::Gpio::new(23);
    let pin24 = pi::gpio::Gpio::new(24);
    let pin25 = pi::gpio::Gpio::new(25);
    let pin26 = pi::gpio::Gpio::new(26);
    let pin27 = pi::gpio::Gpio::new(27);

    kprintln!("pin22: {:?}", pin22.function());
    kprintln!("pin23: {:?}", pin23.function());
    kprintln!("pin24: {:?}", pin24.function());
    kprintln!("pin25: {:?}", pin25.function());
    kprintln!("pin26: {:?}", pin26.function());
    kprintln!("pin27: {:?}", pin27.function());

    let pin48 = pi::gpio::Gpio::new(48);
    let pin49 = pi::gpio::Gpio::new(49);
    let pin50 = pi::gpio::Gpio::new(50);
    let pin51 = pi::gpio::Gpio::new(51);
    let pin52 = pi::gpio::Gpio::new(52);
    let pin53 = pi::gpio::Gpio::new(53);

    kprintln!("pin48: {:?}", pin48.function());
    kprintln!("pin49: {:?}", pin49.function());
    kprintln!("pin50: {:?}", pin50.function());
    kprintln!("pin51: {:?}", pin51.function());
    kprintln!("pin52: {:?}", pin52.function());
    kprintln!("pin53: {:?}", pin53.function());

    // let pin48 = pin48.into_alt(pi::gpio::Function::Alt0);
    // let pin49 = pin49.into_alt(pi::gpio::Function::Alt0);
    // let pin50 = pin50.into_alt(pi::gpio::Function::Alt0);
    // let pin51 = pin51.into_alt(pi::gpio::Function::Alt0);
    // let pin52 = pin52.into_alt(pi::gpio::Function::Alt0);
    // let pin53 = pin53.into_alt(pi::gpio::Function::Alt0);

    // kprintln!("pin48: {:?}", pin48.function());
    // kprintln!("pin49: {:?}", pin49.function());
    // kprintln!("pin50: {:?}", pin50.function());
    // kprintln!("pin51: {:?}", pin51.function());
    // kprintln!("pin52: {:?}", pin52.function());
    // kprintln!("pin53: {:?}", pin53.function());
}
