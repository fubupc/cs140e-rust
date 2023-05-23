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

extern crate pi;
extern crate stack_vec;

pub mod console;
pub mod lang_items;
pub mod mutex;
pub mod shell;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // STEP 1: Set GPIO Pin 16 as output.
    let mut pin16 = pi::gpio::Gpio::new(16).into_output();

    // STEP 2: Continuously set and clear GPIO 16.
    loop {
        pin16.set();
        pi::timer::spin_sleep_ms(1000);
        pin16.clear();
        pi::timer::spin_sleep_ms(1000);
    }
}
