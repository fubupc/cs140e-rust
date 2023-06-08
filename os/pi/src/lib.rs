#![feature(core_intrinsics)]
// #![feature(const_fn)]
// #![feature(asm)]
#![feature(decl_macro)]
// #![feature(repr_align)]
// #![feature(attr_literals)]
#![feature(never_type)]
// #![feature(pointer_methods)]

#![no_std]
#![allow(unused)]

#[cfg(feature = "custom-std")]
extern crate custom_std as std;
#[cfg(not(feature = "custom-std"))]
extern crate std;

extern crate volatile;

pub mod atags;
pub mod common;
pub mod gpio;
pub mod timer;
pub mod uart;
