#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(restricted_std)]
#![no_main]

pub mod console;
pub mod mutex;
pub mod shell;

use core::arch::global_asm;
global_asm!(include_str!("../ext/init.S"));

use core::fmt::Write;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut uart = pi::uart::MiniUart::new();
    uart.write_str("hello!\n").unwrap();
    loop {
        uart.write_str("> ").unwrap();
        let b = uart.read_byte();
        uart.write_byte(b);
        uart.write_str("\n").unwrap();
    }
}
