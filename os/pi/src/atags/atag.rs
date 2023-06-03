use atags::raw;

pub use atags::raw::{Core, Mem};

/// An ATAG.
#[derive(Debug, Copy, Clone)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        match self {
            Atag::Core(core) => Some(core),
            _ => None,
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        match self {
            Atag::Mem(mem) => Some(mem),
            _ => None,
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        match self {
            Atag::Cmd(cmd) => Some(cmd),
            _ => None,
        }
    }
}

// FIXME: Implement `From<raw::Core>`, `From<raw::Mem>`, and `From<&raw::Cmd>`
// for `Atag`. These implementations should be used by the `From<&raw::Atag> for
// Atag` implementation below.

impl From<raw::Core> for Atag {
    fn from(value: raw::Core) -> Self {
        Atag::Core(value)
    }
}

impl From<raw::Mem> for Atag {
    fn from(value: raw::Mem) -> Self {
        Atag::Mem(value)
    }
}

impl From<&raw::Cmd> for Atag {
    fn from(value: &raw::Cmd) -> Self {
        let mut len = 0;
        let ptr = value as *const raw::Cmd as *const u8;
        while unsafe { *ptr.add(len) } != 0x00 {
            len += 1;
        }
        let s = unsafe { std::slice::from_raw_parts(ptr, len) };
        let s = std::str::from_utf8(s).expect("Atag::Cmd contains invalid string");

        Atag::Cmd(s)
    }
}

impl<'a> From<&'a raw::Atag> for Atag {
    fn from(atag: &raw::Atag) -> Atag {
        // FIXME: Complete the implementation below.

        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => core.into(),
                (raw::Atag::MEM, &raw::Kind { mem }) => mem.into(),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => cmd.into(),
                (raw::Atag::NONE, _) => Atag::None,
                (id, _) => Atag::Unknown(id),
            }
        }
    }
}
