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

use core::arch::global_asm;
#[cfg(not(test))]
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
    #[cfg(not(test))]
    ALLOCATOR.initialize();

    let mut v = vec![];
    for i in 0..1000 {
        v.push(i);
        console::kprintln!("{:?}", v);
    }

    loop {}
}
