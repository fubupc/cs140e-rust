#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

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
