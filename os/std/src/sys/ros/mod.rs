use os::raw::c_char;

pub fn decode_error_kind(_errno: i32) -> ::io::ErrorKind {
    ::io::ErrorKind::Other
}

pub fn strlen(string: *const c_char) -> usize {
    let mut size = 0;
    while unsafe { *(string.offset(size as isize)) } != 0 {
        size += 1;
    }

    size
}

pub mod os {
    /// Gets a detailed string description for the given error number.
    pub fn error_string(_errno: i32) -> String {
        "unknown error".to_string()
    }

    /// Returns the platform-specific value of errno
    pub fn errno() -> i32 {
        -1
    }
}

pub mod os_str {
    use borrow::Cow;
    use fmt;
    use str;
    use mem;
    use rc::Rc;
    use sync::Arc;
    use sys_common::{AsInner, IntoInner};
    use sys_common::bytestring::debug_fmt_bytestring;
    // use std_unicode::lossy::Utf8Lossy;

    #[derive(Clone, Hash)]
    pub struct Buf {
        pub inner: Vec<u8>
    }

    pub struct Slice {
        pub inner: [u8]
    }

    impl fmt::Debug for Slice {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            debug_fmt_bytestring(&self.inner, formatter)
        }
    }

    impl fmt::Display for Slice {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            debug_fmt_bytestring(&self.inner, formatter)
        }
    }

    impl fmt::Debug for Buf {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self.as_slice(), formatter)
        }
    }

    impl fmt::Display for Buf {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self.as_slice(), formatter)
        }
    }

    impl IntoInner<Vec<u8>> for Buf {
        fn into_inner(self) -> Vec<u8> {
            self.inner
        }
    }

    impl AsInner<[u8]> for Buf {
        fn as_inner(&self) -> &[u8] {
            &self.inner
        }
    }

    impl Buf {
        pub fn from_string(s: String) -> Buf {
            Buf { inner: s.into_bytes() }
        }

        #[inline]
        pub fn with_capacity(capacity: usize) -> Buf {
            Buf {
                inner: Vec::with_capacity(capacity)
            }
        }

        #[inline]
        pub fn clear(&mut self) {
            self.inner.clear()
        }

        #[inline]
        pub fn capacity(&self) -> usize {
            self.inner.capacity()
        }

        #[inline]
        pub fn reserve(&mut self, additional: usize) {
            self.inner.reserve(additional)
        }

        #[inline]
        pub fn reserve_exact(&mut self, additional: usize) {
            self.inner.reserve_exact(additional)
        }

        #[inline]
        pub fn shrink_to_fit(&mut self) {
            self.inner.shrink_to_fit()
        }

        pub fn as_slice(&self) -> &Slice {
            unsafe { mem::transmute(&*self.inner) }
        }

        pub fn into_string(self) -> Result<String, Buf> {
            String::from_utf8(self.inner).map_err(|p| Buf { inner: p.into_bytes() } )
        }

        pub fn push_slice(&mut self, s: &Slice) {
            self.inner.extend_from_slice(&s.inner)
        }

        #[inline]
        pub fn into_box(self) -> Box<Slice> {
            unsafe { mem::transmute(self.inner.into_boxed_slice()) }
        }

        #[inline]
        pub fn from_box(boxed: Box<Slice>) -> Buf {
            let inner: Box<[u8]> = unsafe { mem::transmute(boxed) };
            Buf { inner: inner.into_vec() }
        }

        #[inline]
        pub fn into_arc(&self) -> Arc<Slice> {
            self.as_slice().into_arc()
        }

        #[inline]
        pub fn into_rc(&self) -> Rc<Slice> {
            self.as_slice().into_rc()
        }
    }

    impl Slice {
        fn from_u8_slice(s: &[u8]) -> &Slice {
            unsafe { mem::transmute(s) }
        }

        pub fn from_str(s: &str) -> &Slice {
            Slice::from_u8_slice(s.as_bytes())
        }

        pub fn to_str(&self) -> Option<&str> {
            str::from_utf8(&self.inner).ok()
        }

        pub fn to_string_lossy(&self) -> Cow<str> {
            String::from_utf8_lossy(&self.inner)
        }

        pub fn to_owned(&self) -> Buf {
            Buf { inner: self.inner.to_vec() }
        }

        #[inline]
        pub fn into_box(&self) -> Box<Slice> {
            let boxed: Box<[u8]> = self.inner.into();
            unsafe { mem::transmute(boxed) }
        }

        pub fn empty_box() -> Box<Slice> {
            let boxed: Box<[u8]> = Default::default();
            unsafe { mem::transmute(boxed) }
        }

        #[inline]
        pub fn into_arc(&self) -> Arc<Slice> {
            let arc: Arc<[u8]> = Arc::from(&self.inner);
            unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Slice) }
        }

        #[inline]
        pub fn into_rc(&self) -> Rc<Slice> {
            let rc: Rc<[u8]> = Rc::from(&self.inner);
            unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Slice) }
        }
    }
}

pub mod path {
    use ffi::OsStr;
    use path::Prefix;

    #[inline] pub fn is_sep_byte(b: u8) -> bool { b == b'/' }
    #[inline] pub fn is_verbatim_sep(b: u8) -> bool { b == b'/' }
    pub fn parse_prefix(_: &OsStr) -> Option<Prefix> { None }

    pub const MAIN_SEP_STR: &'static str = "/";
    pub const MAIN_SEP: char = '/';
}
