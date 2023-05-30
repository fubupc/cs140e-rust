#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(restricted_std)]
#![feature(allocator_api)]
#![feature(lang_items)]
#![no_main]

pub mod allocator;
pub mod console;
pub mod fs;
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
    // ALLOCATOR.initialize();
    shell::shell("> ")
}
