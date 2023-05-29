#![feature(restricted_std)]
#![no_main]

use pi;
use xmodem;

use core::arch::{asm, global_asm};
use std::fmt::Write;
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

    loop {
        uart.write_str("Bootloader is waiting to receive kernel...\n")
            .unwrap();
        pi::timer::spin_sleep_ms(5000);

        let buf = unsafe { std::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };

        match xmodem::Xmodem::receive(&mut uart, buf) {
            Ok(_) => {
                uart.write_str("Kernel received OK, about to start in 5 seconds...\n")
                    .unwrap();
                pi::timer::spin_sleep_ms(5000);
                jump_to(BINARY_START)
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
