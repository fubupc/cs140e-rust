#![feature(decl_macro)]
#![feature(lang_items)]
#![feature(negative_impls)]
#![feature(panic_info_message)]
#![feature(prelude_import)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
#[macro_use]
extern crate custom_std as std;

#[allow(unused_imports)]
#[prelude_import]
use std::prelude::v1::*;

// #[macro_use]
// extern crate alloc;

mod console;
mod lang_items;
mod mutex;

use pi;
use xmodem;

use core::{
    arch::{asm, global_asm},
    fmt::Write,
};

use crate::console::{kprint, kprintln};
global_asm!(include_str!("../ext/init.S"));

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br {}", in(reg) addr as usize);
        loop {
            asm!("nop")
        }
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    use std::io;

    let mut uart = pi::uart::MiniUart::new();
    uart.set_read_timeout(750);

    kprintln!("\nReady to receive kernel");

    loop {
        let buf = unsafe { core::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };

        match xmodem::Xmodem::receive(&mut uart, buf) {
            Ok(_) => {
                // Repeatedly print until receive any user input
                loop {
                    uart.write_byte(b'\r'); // Carriage Return without Line Feed
                    kprint!("Kernel received, press any key to continue");
                    match uart.wait_for_byte() {
                        Ok(_) => break,
                        Err(_) => {}
                    }
                }
                kprint!("\n");
                jump_to(BINARY_START);
            }
            Err(err) => match err.kind() {
                io::ErrorKind::TimedOut => {}
                _ => uart
                    .write_fmt(format_args!("Failed to receive kernel, retry: {:?}\n", err))
                    .unwrap(),
            },
        }
    }
}
