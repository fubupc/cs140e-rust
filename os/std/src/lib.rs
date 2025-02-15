// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # The Rust Standard Library
//!
//! The Rust Standard Library is the foundation of portable Rust software, a
//! set of minimal and battle-tested shared abstractions for the [broader Rust
//! ecosystem][crates.io]. It offers core types, like [`Vec<T>`] and
//! [`Option<T>`], library-defined [operations on language
//! primitives](#primitives), [standard macros](#macros), [I/O] and
//! [multithreading], among [many other things][other].
//!
//! `std` is available to all Rust crates by default, just as if each one
//! contained an `extern crate std;` import at the [crate root]. Therefore the
//! standard library can be accessed in [`use`] statements through the path
//! `std`, as in [`use std::env`], or in expressions through the absolute path
//! `::std`, as in [`::std::env::args`].
//!
//! # How to read this documentation
//!
//! If you already know the name of what you are looking for, the fastest way to
//! find it is to use the <a href="#" onclick="focusSearchBar();">search
//! bar</a> at the top of the page.
//!
//! Otherwise, you may want to jump to one of these useful sections:
//!
//! * [`std::*` modules](#modules)
//! * [Primitive types](#primitives)
//! * [Standard macros](#macros)
//! * [The Rust Prelude](prelude/index.html)
//!
//! If this is your first time, the documentation for the standard library is
//! written to be casually perused. Clicking on interesting things should
//! generally lead you to interesting places. Still, there are important bits
//! you don't want to miss, so read on for a tour of the standard library and
//! its documentation!
//!
//! Once you are familiar with the contents of the standard library you may
//! begin to find the verbosity of the prose distracting. At this stage in your
//! development you may want to press the **[-]** button near the top of the
//! page to collapse it into a more skimmable view.
//!
//! While you are looking at that **[-]** button also notice the **[src]**
//! button. Rust's API documentation comes with the source code and you are
//! encouraged to read it. The standard library source is generally high
//! quality and a peek behind the curtains is often enlightening.
//!
//! # What is in the standard library documentation?
//!
//! First of all, The Rust Standard Library is divided into a number of focused
//! modules, [all listed further down this page](#modules). These modules are
//! the bedrock upon which all of Rust is forged, and they have mighty names
//! like [`std::slice`] and [`std::cmp`]. Modules' documentation typically
//! includes an overview of the module along with examples, and are a smart
//! place to start familiarizing yourself with the library.
//!
//! Second, implicit methods on [primitive types] are documented here. This can
//! be a source of confusion for two reasons:
//!
//! 1. While primitives are implemented by the compiler, the standard library
//!    implements methods directly on the primitive types (and it is the only
//!    library that does so), which are [documented in the section on
//!    primitives](#primitives).
//! 2. The standard library exports many modules *with the same name as
//!    primitive types*. These define additional items related to the primitive
//!    type, but not the all-important methods.
//!
//! So for example there is a [page for the primitive type
//! `i32`](primitive.i32.html) that lists all the methods that can be called on
//! 32-bit integers (very useful), and there is a [page for the module
//! `std::i32`](i32/index.html) that documents the constant values [`MIN`] and
//! [`MAX`](i32/constant.MAX.html) (rarely useful).
//!
//! Note the documentation for the primitives [`str`] and [`[T]`][slice] (also
//! called 'slice'). Many method calls on [`String`] and [`Vec<T>`] are actually
//! calls to methods on [`str`] and [`[T]`][slice] respectively, via [deref
//! coercions][deref-coercions].
//!
//! Third, the standard library defines [The Rust Prelude], a small collection
//! of items - mostly traits - that are imported into every module of every
//! crate. The traits in the prelude are pervasive, making the prelude
//! documentation a good entry point to learning about the library.
//!
//! And finally, the standard library exports a number of standard macros, and
//! [lists them on this page](#macros) (technically, not all of the standard
//! macros are defined by the standard library - some are defined by the
//! compiler - but they are documented here the same). Like the prelude, the
//! standard macros are imported by default into all crates.
//!
//! # Contributing changes to the documentation
//!
//! Check out the rust contribution guidelines [here](
//! https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).
//! The source for this documentation can be found on [Github](https://github.com/rust-lang).
//! To contribute changes, make sure you read the guidelines first, then submit
//! pull-requests for your suggested changes.
//!
//! Contributions are appreciated! If you see a part of the docs that can be
//! improved, submit a PR, or chat with us first on irc.mozilla.org #rust-docs.
//!
//! # A Tour of The Rust Standard Library
//!
//! The rest of this crate documentation is dedicated to pointing out notable
//! features of The Rust Standard Library.
//!
//! ## Containers and collections
//!
//! The [`option`] and [`result`] modules define optional and error-handling
//! types, [`Option<T>`] and [`Result<T, E>`]. The [`iter`] module defines
//! Rust's iterator trait, [`Iterator`], which works with the [`for`] loop to
//! access collections.
//!
//! The standard library exposes three common ways to deal with contiguous
//! regions of memory:
//!
//! * [`Vec<T>`] - A heap-allocated *vector* that is resizable at runtime.
//! * [`[T; n]`][array] - An inline *array* with a fixed size at compile time.
//! * [`[T]`][slice] - A dynamically sized *slice* into any other kind of contiguous
//!   storage, whether heap-allocated or not.
//!
//! Slices can only be handled through some kind of *pointer*, and as such come
//! in many flavors such as:
//!
//! * `&[T]` - *shared slice*
//! * `&mut [T]` - *mutable slice*
//! * [`Box<[T]>`][owned slice] - *owned slice*
//!
//! [`str`], a UTF-8 string slice, is a primitive type, and the standard library
//! defines many methods for it. Rust [`str`]s are typically accessed as
//! immutable references: `&str`. Use the owned [`String`] for building and
//! mutating strings.
//!
//! For converting to strings use the [`format!`] macro, and for converting from
//! strings use the [`FromStr`] trait.
//!
//! Data may be shared by placing it in a reference-counted box or the [`Rc`]
//! type, and if further contained in a [`Cell`] or [`RefCell`], may be mutated
//! as well as shared. Likewise, in a concurrent setting it is common to pair an
//! atomically-reference-counted box, [`Arc`], with a [`Mutex`] to get the same
//! effect.
//!
//! The [`collections`] module defines maps, sets, linked lists and other
//! typical collection types, including the common [`HashMap<K, V>`].
//!
//! ## Platform abstractions and I/O
//!
//! Besides basic data types, the standard library is largely concerned with
//! abstracting over differences in common platforms, most notably Windows and
//! Unix derivatives.
//!
//! Common types of I/O, including [files], [TCP], [UDP], are defined in the
//! [`io`], [`fs`], and [`net`] modules.
//!
//! The [`thread`] module contains Rust's threading abstractions. [`sync`]
//! contains further primitive shared memory types, including [`atomic`] and
//! [`mpsc`], which contains the channel types for message passing.
//!
//! [I/O]: io/index.html
//! [`MIN`]: i32/constant.MIN.html
//! [TCP]: net/struct.TcpStream.html
//! [The Rust Prelude]: prelude/index.html
//! [UDP]: net/struct.UdpSocket.html
//! [`::std::env::args`]: env/fn.args.html
//! [`Arc`]: sync/struct.Arc.html
//! [owned slice]: boxed/index.html
//! [`Cell`]: cell/struct.Cell.html
//! [`FromStr`]: str/trait.FromStr.html
//! [`HashMap<K, V>`]: collections/struct.HashMap.html
//! [`Iterator`]: iter/trait.Iterator.html
//! [`Mutex`]: sync/struct.Mutex.html
//! [`Option<T>`]: option/enum.Option.html
//! [`Rc`]: rc/index.html
//! [`RefCell`]: cell/struct.RefCell.html
//! [`Result<T, E>`]: result/enum.Result.html
//! [`String`]: string/struct.String.html
//! [`Vec<T>`]: vec/index.html
//! [array]: primitive.array.html
//! [slice]: primitive.slice.html
//! [`atomic`]: sync/atomic/index.html
//! [`collections`]: collections/index.html
//! [`for`]: ../book/first-edition/loops.html#for
//! [`format!`]: macro.format.html
//! [`fs`]: fs/index.html
//! [`io`]: io/index.html
//! [`iter`]: iter/index.html
//! [`mpsc`]: sync/mpsc/index.html
//! [`net`]: net/index.html
//! [`option`]: option/index.html
//! [`result`]: result/index.html
//! [`std::cmp`]: cmp/index.html
//! [`std::slice`]: slice/index.html
//! [`str`]: primitive.str.html
//! [`sync`]: sync/index.html
//! [`thread`]: thread/index.html
//! [`use std::env`]: env/index.html
//! [`use`]: ../book/first-edition/crates-and-modules.html#importing-modules-with-use
//! [crate root]: ../book/first-edition/crates-and-modules.html#basic-terminology-crates-and-modules
//! [crates.io]: https://crates.io
//! [deref-coercions]: ../book/second-edition/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
//! [files]: fs/struct.File.html
//! [multithreading]: thread/index.html
//! [other]: #what-is-in-the-standard-library-documentation
//! [primitive types]: ../book/first-edition/primitive-types.html

#![stable(feature = "rust1", since = "1.0.0")]
//- #![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
//-        html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
//-        html_root_url = "https://doc.rust-lang.org/nightly/",
//-        html_playground_url = "https://play.rust-lang.org/",
//-        issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
//-        test(no_crate_inject, attr(deny(warnings))),
//-        test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]

// Don't link to std. We are std.
#![no_std]

//- #![deny(missing_docs)]
//- #![deny(missing_debug_implementations)]
//-
//- // Tell the compiler to link to either panic_abort or panic_unwind
//- #![needs_panic_runtime]
//-
//- // Turn warnings into errors, but only after stage0, where it can be useful for
//- // code to emit warnings during language transitions
//- #![cfg_attr(not(stage0), deny(warnings))]

// std may use features in a platform-specific way
#![allow(unused_features)]
#![allow(unused_imports)]

// std is implemented with unstable features, many of which are internal
// compiler details that will never be stable
// #![feature(alloc)]
#![feature(allocator_api)]
// #![feature(alloc_system)]
#![feature(allocator_internals)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
// #![feature(align_offset)]
// #![feature(array_error_internals)]
#![feature(ascii_ctype)]
// #![feature(asm)]
// #![feature(attr_literals)]
// #![feature(box_syntax)]
#![feature(cfg_target_has_atomic)]
#![feature(cfg_target_thread_local)]
// #![feature(cfg_target_vendor)]
// #![feature(char_error_internals)]
#![feature(char_internals)]
// #![feature(collections_range)]
#![feature(compiler_builtins)]
// #![feature(const_fn)]
// #![feature(core_float)]
#![feature(core_intrinsics)]
#![feature(dropck_eyepatch)]
#![feature(exact_size_is_empty)]
// #![feature(fs_read_write)]
// #![feature(fixed_size_array)]
// #![feature(float_from_str_radix)]
#![feature(fn_traits)]
// #![feature(fnbox)]
// #![feature(fused)]
// #![feature(generic_param_attrs)]
// #![feature(hashmap_hasher)]
#![feature(hashmap_internals)]
// #![feature(heap_api)]
#![feature(i128)]
// #![feature(i128_type)]
// #![feature(inclusive_range)]
// #![feature(int_error_internals)]
// #![feature(integer_atomics)]
// #![feature(into_cow)]
#![feature(lang_items)]
#![feature(libc)]
// #![feature(link_args)]
#![feature(linkage)]
// #![feature(macro_reexport)]
// #![feature(macro_vis_matcher)]
#![feature(needs_panic_runtime)]
#![feature(negative_impls)]
#![feature(never_type)]
// #![feature(num_bits_bytes)]
// #![feature(old_wrapping)]
// #![feature(on_unimplemented)]
// #![feature(oom)]
#![feature(auto_traits)]
// #![feature(panic_unwind)]
// #![feature(peek)]
// #![feature(placement_in_syntax)]
// #![feature(placement_new_protocol)]
#![feature(prelude_import)]
#![feature(ptr_internals)]
// #![feature(rand)]
// #![feature(raw)]
// #![feature(repr_align)]
#![feature(rustc_attrs)]
// #![feature(sip_hash_13)]
// #![feature(slice_bytes)]
#![feature(slice_concat_ext)]
#![feature(slice_concat_trait)]
#![feature(slice_internals)]
// #![feature(slice_patterns)]
#![feature(slice_ptr_get)]
#![feature(staged_api)]
#![feature(stmt_expr_attributes)]
// #![feature(str_char)]
#![feature(str_internals)]
// #![feature(str_utf16)]
// #![feature(termination_trait)]
#![feature(test, rustc_private)]
#![feature(thread_local)]
// #![feature(toowned_clone_into)]
// #![feature(try_from)]
#![feature(unboxed_closures)]
// #![feature(unicode)]
#![feature(utf8_chunks)]
// #![feature(untagged_unions)]
// #![feature(unwind_attributes)]
// #![feature(vec_push_all)]
#![feature(doc_cfg)]
#![feature(doc_masked)]
#![feature(doc_notable_trait)]
#![feature(custom_inner_attributes)]

// #![feature(shared)] //- added due to no NonNull changes
// #![feature(unique)] //- added due to no NonNull changes


//- #![cfg_attr(test, feature(update_panic_count))]
//- #![cfg_attr(windows, feature(used))]

//- #![default_lib_allocator]

//- // Always use alloc_system during stage0 since we don't know if the alloc_*
//- // crate the stage0 compiler will pick by default is enabled (e.g.
//- // if the user has disabled jemalloc in `./configure`).
//- // `force_alloc_system` is *only* intended as a workaround for local rebuilds
//- // with a rustc without jemalloc.
//- // FIXME(#44236) shouldn't need MSVC logic
//- #![cfg_attr(all(not(target_env = "msvc"),
//-                 any(stage0, feature = "force_alloc_system")),
//-             feature(global_allocator))]
//- #[cfg(all(not(target_env = "msvc"),
//-           any(stage0, feature = "force_alloc_system")))]
//- #[global_allocator]
//- static ALLOC: alloc_system::System = alloc_system::System;

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

//- // Access to Bencher, etc.
//- #[cfg(test)] extern crate test;
//- #[cfg(test)] extern crate rand;

// We want to re-export a few macros from core but libcore has already been
// imported by the compiler (via our #[no_std] attribute) In this case we just
// add a new crate name so we can attach the re-exports to it.
//- #[macro_reexport(assert, assert_eq, assert_ne, debug_assert, debug_assert_eq,
// #[macro_reexport(panic, assert, assert_eq, assert_ne, debug_assert, debug_assert_eq,
//                  debug_assert_ne, unreachable, unimplemented, write, writeln, try)]
// extern crate core as __core;

#[macro_use]
// #[macro_reexport(vec, format)]
extern crate alloc;
//- extern crate alloc_system;
// extern crate std_unicode;
//- #[doc(masked)]
//- extern crate libc;

//- // We always need an unwinder currently for backtraces
//- #[doc(masked)]
//- #[allow(unused_extern_crates)]
//- extern crate unwind;

// compiler-rt intrinsics
// #[doc(masked)]
// extern crate compiler_builtins;

//- // During testing, this crate is not actually the "real" std library, but rather
//- // it links to the real std library, which was compiled from this same source
//- // code. So any lang items std defines are conditionally excluded (or else they
//- // wolud generate duplicate lang item errors), and any globals it defines are
//- // _not_ the globals used by "real" std. So this import, defined only during
//- // testing gives test-std access to real-std lang items and globals. See #2912
//- #[cfg(test)] extern crate std as realstd;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and re-exports
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::any;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cell;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::clone;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cmp;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::convert;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::default;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::hash;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::intrinsics;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::iter;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::marker;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::mem;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ops;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ptr;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use core::raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::result;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::option;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::isize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i64;
#[unstable(feature = "i128", issue = "35118")]
pub use core::i128;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::usize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::boxed;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::rc;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::borrow;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::fmt;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::slice;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::str;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::string;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::vec;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::char;
#[unstable(feature = "i128", issue = "35118")]
pub use core::u128;

//- pub mod f32;
//- pub mod f64;

//- #[macro_use]
//- pub mod thread;
pub mod ascii;
pub mod collections;
//- pub mod env;
pub mod error;
pub mod ffi;
//- pub mod fs;
pub mod io;
//- pub mod net;
pub mod num;
pub mod os;
//- pub mod panic;
pub mod path;
//- pub mod process;
pub mod sync;
//- pub mod time;
//- pub mod heap;

//- // Platform-abstraction modules
#[macro_use]
mod sys_common;
mod sys;

// Private support modules
//- mod panicking;
mod memchr;

// The runtime entry point and a few unstable public functions used by the
// compiler
//- pub mod rt;
// The trait to support returning arbitrary types in the main function
//- mod termination;
//-
//- #[unstable(feature = "termination_trait", issue = "43301")]
//- pub use self::termination::Termination;

// Include a number of private modules that exist solely to provide
// the rustdoc documentation for primitive types. Using `include!`
// because rustdoc only looks for these modules at the crate level.
include!("primitive_docs.rs");
