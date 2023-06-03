#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![allow(unused)]

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
    for atag in pi::atags::Atags::get() {
        console::kprint!("{:#?}\n", atag);
    }

    // ALLOCATOR.initialize();
    shell::shell("> ")
}
