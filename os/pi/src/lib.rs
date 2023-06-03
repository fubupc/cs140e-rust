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

#[cfg(feature = "std")]
extern crate std;
extern crate volatile;

pub mod timer;
pub mod uart;
pub mod gpio;
pub mod common;
pub mod atags;
