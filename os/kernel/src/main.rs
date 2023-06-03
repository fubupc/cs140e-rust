#![no_std]
#![no_main]
#![allow(unused)]
#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![feature(prelude_import)]
// `test` crate depends on built-in `std` so not works with #![no_std].
// The feature "custom test frameworks" can help.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// Import our customized std
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::v1::*;

#[macro_use]
extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod lang_items;
pub mod mutex;
pub mod shell;

use core::arch::global_asm;
global_asm!(include_str!("../ext/init.S"));

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;

#[cfg(not(test))]
pub static _ALLOCATOR: Allocator = Allocator::uninitialized();
#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: &Allocator = &_ALLOCATOR;

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    #[cfg(test)]
    test_main();

    for atag in pi::atags::Atags::get() {
        console::kprint!("{:#?}\n", atag);
    }

    // ALLOCATOR.initialize();
    shell::shell("> ")
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    console::kprintln!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
