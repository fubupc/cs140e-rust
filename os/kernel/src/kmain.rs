#![feature(lang_items)]
#![feature(core_intrinsics)]
// #![feature(const_fn)]
// #![feature(asm)]
#![feature(auto_traits)]
#![feature(decl_macro)]
// #![feature(repr_align)]
// #![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(negative_impls)]
#![feature(restricted_std)]

use std::fmt::Write;

extern crate pi;
extern crate stack_vec;

pub mod console;
pub mod lang_items;
pub mod mutex;
pub mod shell;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut uart = pi::uart::MiniUart::new();
    loop {
        let b = uart.read_byte();
        uart.write_byte(b);
        uart.write_str("<-").unwrap();
    }
}
