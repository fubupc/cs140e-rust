// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits for working with Errors.
//!
//! # The `Error` trait
//!
//! `Error` is a trait representing the basic expectations for error values,
//! i.e. values of type `E` in [`Result<T, E>`]. At a minimum, errors must provide
//! a description, but they may optionally provide additional detail (via
//! [`Display`]) and cause chain information:
//!
//! ```
//! use std::fmt::Display;
//!
//! trait Error: Display {
//!     fn description(&self) -> &str;
//!
//!     fn cause(&self) -> Option<&Error> { None }
//! }
//! ```
//!
//! The [`cause`] method is generally used when errors cross "abstraction
//! boundaries", i.e.  when a one module must report an error that is "caused"
//! by an error from a lower-level module. This setup makes it possible for the
//! high-level module to provide its own errors that do not commit to any
//! particular implementation, but also reveal some of its implementation for
//! debugging via [`cause`] chains.
//!
//! [`Result<T, E>`]: ../result/enum.Result.html
//! [`Display`]: ../fmt/trait.Display.html
//! [`cause`]: trait.Error.html#method.cause

#![stable(feature = "rust1", since = "1.0.0")]

// A note about crates and the facade:
//
// Originally, the `Error` trait was defined in libcore, and the impls
// were scattered about. However, coherence objected to this
// arrangement, because to create the blanket impls for `Box` required
// knowing that `&str: !Error`, and we have no means to deal with that
// sort of conflict just now. Therefore, for the time being, we have
// moved the `Error` trait into libstd. As we evolve a sol'n to the
// coherence challenge (e.g., specialization, neg impls, etc) we can
// reconsider what crate these items belong in.

use alloc::alloc as allocator;
use any::TypeId;
use borrow::Cow;
use cell;
use core::char;
use convert;
use core::array;
use fmt::{self, Debug, Display};
use mem::transmute;
use num;
use str;
use string;

/// Base functionality for all errors in Rust.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Error: Debug + Display {
    /// A short description of the error.
    ///
    /// The description should only be used for a simple message.
    /// It should not contain newlines or sentence-ending punctuation,
    /// to facilitate embedding in larger user-facing strings.
    /// For showing formatted error messages with more information see
    /// [`Display`].
    ///
    /// [`Display`]: ../fmt/trait.Display.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::error::Error;
    ///
    /// match "xc".parse::<u32>() {
    ///     Err(e) => {
    ///         println!("Error: {}", e.description());
    ///     }
    ///     _ => println!("No error"),
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn description(&self) -> &str;

    /// The lower-level cause of this error, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::error::Error;
    /// use std::fmt;
    ///
    /// #[derive(Debug)]
    /// struct SuperError {
    ///     side: SuperErrorSideKick,
    /// }
    ///
    /// impl fmt::Display for SuperError {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         write!(f, "SuperError is here!")
    ///     }
    /// }
    ///
    /// impl Error for SuperError {
    ///     fn description(&self) -> &str {
    ///         "I'm the superhero of errors"
    ///     }
    ///
    ///     fn cause(&self) -> Option<&Error> {
    ///         Some(&self.side)
    ///     }
    /// }
    ///
    /// #[derive(Debug)]
    /// struct SuperErrorSideKick;
    ///
    /// impl fmt::Display for SuperErrorSideKick {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         write!(f, "SuperErrorSideKick is here!")
    ///     }
    /// }
    ///
    /// impl Error for SuperErrorSideKick {
    ///     fn description(&self) -> &str {
    ///         "I'm SuperError side kick"
    ///     }
    /// }
    ///
    /// fn get_super_error() -> Result<(), SuperError> {
    ///     Err(SuperError { side: SuperErrorSideKick })
    /// }
    ///
    /// fn main() {
    ///     match get_super_error() {
    ///         Err(e) => {
    ///             println!("Error: {}", e.description());
    ///             println!("Caused by: {}", e.cause().unwrap());
    ///         }
    ///         _ => println!("No error"),
    ///     }
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn cause(&self) -> Option<&dyn Error> { None }

    /// Get the `TypeId` of `self`
    #[doc(hidden)]
    #[unstable(feature = "error_type_id",
               reason = "unclear whether to commit to this public implementation detail",
               issue = "27745")]
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
    fn from(err: E) -> Box<dyn Error + 'a> {
        Box::new(err)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a> {
    fn from(err: E) -> Box<dyn Error + Send + Sync + 'a> {
        Box::new(err)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl From<String> for Box<dyn Error + Send + Sync> {
    fn from(err: String) -> Box<dyn Error + Send + Sync> {
        #[derive(Debug)]
        struct StringError(String);

        impl Error for StringError {
            fn description(&self) -> &str { &self.0 }
        }

        impl Display for StringError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        Box::new(StringError(err))
    }
}

#[stable(feature = "string_box_error", since = "1.6.0")]
impl From<String> for Box<dyn Error> {
    fn from(str_err: String) -> Box<dyn Error> {
        let err1: Box<dyn Error + Send + Sync> = From::from(str_err);
        let err2: Box<dyn Error> = err1;
        err2
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, 'b> From<&'b str> for Box<dyn Error + Send + Sync + 'a> {
    fn from(err: &'b str) -> Box<dyn Error + Send + Sync + 'a> {
        From::from(String::from(err))
    }
}

#[stable(feature = "string_box_error", since = "1.6.0")]
impl<'a> From<&'a str> for Box<dyn Error> {
    fn from(err: &'a str) -> Box<dyn Error> {
        From::from(String::from(err))
    }
}

#[stable(feature = "cow_box_error", since = "1.22.0")]
impl<'a, 'b> From<Cow<'b, str>> for Box<dyn Error + Send + Sync + 'a> {
    fn from(err: Cow<'b, str>) -> Box<dyn Error + Send + Sync + 'a> {
        From::from(String::from(err))
    }
}

#[stable(feature = "cow_box_error", since = "1.22.0")]
impl<'a> From<Cow<'a, str>> for Box<dyn Error> {
    fn from(err: Cow<'a, str>) -> Box<dyn Error> {
        From::from(String::from(err))
    }
}

#[unstable(feature = "never_type", issue = "35121")]
impl Error for ! {
    fn description(&self) -> &str { *self }
}

// #[unstable(feature = "allocator_api",
//            reason = "the precise API and guarantees it provides may be tweaked.",
//            issue = "32838")]
// impl Error for allocator::AllocError {
//     fn description(&self) -> &str {
//         allocator::AllocError::description(self)
//     }
// }

// #[unstable(feature = "allocator_api",
//            reason = "the precise API and guarantees it provides may be tweaked.",
//            issue = "32838")]
// impl Error for allocator::CannotReallocInPlace {
//     fn description(&self) -> &str {
//         allocator::CannotReallocInPlace::description(self)
//     }
// }

#[stable(feature = "rust1", since = "1.0.0")]
impl Error for str::ParseBoolError {
    fn description(&self) -> &str { "failed to parse bool" }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Error for str::Utf8Error {
    fn description(&self) -> &str {
        "invalid utf-8: corrupt contents"
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
// impl Error for num::ParseIntError {
//     fn description(&self) -> &str {
//         self.__description()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl Error for num::TryFromIntError {
//     fn description(&self) -> &str {
//         self.__description()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl Error for array::TryFromSliceError {
//     fn description(&self) -> &str {
//         self.__description()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl Error for num::ParseFloatError {
//     fn description(&self) -> &str {
//         self.__description()
//     }
// }

#[stable(feature = "rust1", since = "1.0.0")]
impl Error for string::FromUtf8Error {
    fn description(&self) -> &str {
        "invalid utf-8"
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Error for string::FromUtf16Error {
    fn description(&self) -> &str {
        "invalid utf-16"
    }
}

#[stable(feature = "str_parse_error2", since = "1.8.0")]
impl Error for string::ParseError {
    fn description(&self) -> &str {
        match *self {}
    }
}

#[stable(feature = "decode_utf16", since = "1.9.0")]
impl Error for char::DecodeUtf16Error {
    fn description(&self) -> &str {
        "unpaired surrogate found"
    }
}

#[stable(feature = "box_error", since = "1.8.0")]
impl<T: Error> Error for Box<T> {
    fn description(&self) -> &str {
        Error::description(&**self)
    }

    fn cause(&self) -> Option<&dyn Error> {
        Error::cause(&**self)
    }
}

#[stable(feature = "fmt_error", since = "1.11.0")]
impl Error for fmt::Error {
    fn description(&self) -> &str {
        "an error occurred when formatting an argument"
    }
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Error for cell::BorrowError {
    fn description(&self) -> &str {
        "already mutably borrowed"
    }
}

#[stable(feature = "try_borrow", since = "1.13.0")]
impl Error for cell::BorrowMutError {
    fn description(&self) -> &str {
        "already borrowed"
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Error for char::CharTryFromError {
    fn description(&self) -> &str {
        "converted integer out of range for `char`"
    }
}

// #[stable(feature = "char_from_str", since = "1.20.0")]
// impl Error for char::ParseCharError {
//     fn description(&self) -> &str {
//         self.__description()
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl Error for convert::Infallible {
//     fn description(&self) -> &str {
//         match *self {
//         }
//     }
// }

// copied from any.rs
impl dyn Error + 'static {
    /// Returns true if the boxed type is the same as `T`
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let boxed = self.type_id();

        // Compare both TypeIds on equality
        t == boxed
    }

    /// Returns some reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                Some(&*(self as *const dyn Error as *const T))
            }
        } else {
            None
        }
    }

    /// Returns some mutable reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe {
                Some(&mut *(self as *mut dyn Error as *mut T))
            }
        } else {
            None
        }
    }
}

impl dyn Error + 'static + Send {
    /// Forwards to the method defined on the type `Any`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        <dyn Error + 'static>::is::<T>(self)
    }

    /// Forwards to the method defined on the type `Any`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        <dyn Error + 'static>::downcast_ref::<T>(self)
    }

    /// Forwards to the method defined on the type `Any`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        <dyn Error + 'static>::downcast_mut::<T>(self)
    }
}

impl dyn Error + 'static + Send + Sync {
    /// Forwards to the method defined on the type `Any`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        <dyn Error + 'static>::is::<T>(self)
    }

    /// Forwards to the method defined on the type `Any`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        <dyn Error + 'static>::downcast_ref::<T>(self)
    }

    /// Forwards to the method defined on the type `Any`.
    #[stable(feature = "error_downcast", since = "1.3.0")]
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        <dyn Error + 'static>::downcast_mut::<T>(self)
    }
}

impl dyn Error {
    #[inline]
    #[stable(feature = "error_downcast", since = "1.3.0")]
    /// Attempt to downcast the box to a concrete type.
    pub fn downcast<T: Error + 'static>(self: Box<Self>) -> Result<Box<T>, Box<dyn Error>> {
        if self.is::<T>() {
            unsafe {
                let raw: *mut dyn Error = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }
}

impl dyn Error + Send {
    #[inline]
    #[stable(feature = "error_downcast", since = "1.3.0")]
    /// Attempt to downcast the box to a concrete type.
    pub fn downcast<T: Error + 'static>(self: Box<Self>)
                                        -> Result<Box<T>, Box<dyn Error + Send>> {
        let err: Box<dyn Error> = self;
        <dyn Error>::downcast(err).map_err(|s| unsafe {
            // reapply the Send marker
            transmute::<Box<dyn Error>, Box<dyn Error + Send>>(s)
        })
    }
}

impl dyn Error + Send + Sync {
    #[inline]
    #[stable(feature = "error_downcast", since = "1.3.0")]
    /// Attempt to downcast the box to a concrete type.
    pub fn downcast<T: Error + 'static>(self: Box<Self>)
                                        -> Result<Box<T>, Box<Self>> {
        let err: Box<dyn Error> = self;
        <dyn Error>::downcast(err).map_err(|s| unsafe {
            // reapply the Send+Sync marker
            transmute::<Box<dyn Error>, Box<dyn Error + Send + Sync>>(s)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use fmt;

    #[derive(Debug, PartialEq)]
    struct A;
    #[derive(Debug, PartialEq)]
    struct B;

    impl fmt::Display for A {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "A")
        }
    }
    impl fmt::Display for B {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "B")
        }
    }

    impl Error for A {
        fn description(&self) -> &str { "A-desc" }
    }
    impl Error for B {
        fn description(&self) -> &str { "A-desc" }
    }

    #[test]
    fn downcasting() {
        let mut a = A;
        let a = &mut a as &mut (Error + 'static);
        assert_eq!(a.downcast_ref::<A>(), Some(&A));
        assert_eq!(a.downcast_ref::<B>(), None);
        assert_eq!(a.downcast_mut::<A>(), Some(&mut A));
        assert_eq!(a.downcast_mut::<B>(), None);

        let a: Box<Error> = Box::new(A);
        match a.downcast::<B>() {
            Ok(..) => panic!("expected error"),
            Err(e) => assert_eq!(*e.downcast::<A>().unwrap(), A),
        }
    }
}
