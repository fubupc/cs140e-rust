#![feature(decl_macro)]
#![feature(prelude_import)]

#![no_std]

extern crate std;
#[prelude_import]
#[allow(unused)]
use std::prelude::v1::*;


#[cfg(not(target_endian="little"))]
compile_error!("only little endian platforms supported");

#[cfg(test)]
mod tests;
mod mbr;
mod util;

pub mod vfat;
pub mod traits;

pub use mbr::*;
