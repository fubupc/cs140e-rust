#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(restricted_std)]
#![no_main]

pub mod console;
pub mod mutex;
pub mod shell;

use core::arch::global_asm;
global_asm!(include_str!("../ext/init.S"));

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    shell::shell("> ")
}
