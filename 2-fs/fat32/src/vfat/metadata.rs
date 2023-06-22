use std::fmt;

use traits;

/// A date as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Date(u16);

// The date on which the file was created.
// Bits 15 - 9: Year (0 = 1980). Bits 8 - 5: Month. Bits 4 - 0: Day.
impl Date {
    pub fn year(&self) -> usize {
        1980 + (self.0 >> 9 & 0x7F) as usize
    }

    pub fn month(&self) -> u8 {
        (self.0 >> 5 & 0xF) as u8
    }

    pub fn day(&self) -> u8 {
        (self.0 & 0x1F) as u8
    }
}

/// Time as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Time(u16);

// The time that the file was created. Multiply Seconds by 2.
// Bits 15 - 11: hours. Bits 10 -5: minutes. Bits 4 - 0: seconds/2.
impl Time {
    pub fn zero() -> Time {
        Time(0)
    }

    pub fn hour(&self) -> u8 {
        (self.0 >> 11 & 0x1F) as u8
    }

    pub fn minute(&self) -> u8 {
        (self.0 >> 5 & 0x3F) as u8
    }

    // NOTE: The granularity is 2 seconds.
    pub fn second(&self) -> u8 {
        (self.0 & 0x1F) as u8 * 2
    }
}

/// File attributes as represented in FAT32 on-disk structures.
/// Attributes of the file. The possible attributes are:
/// READ_ONLY=0x01 HIDDEN=0x02 SYSTEM=0x04 VOLUME_ID=0x08
/// DIRECTORY=0x10 ARCHIVE=0x20
/// LFN=READ_ONLY|HIDDEN|SYSTEM|VOLUME_ID
/// (LFN means that this entry is a long file name entry)
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Attributes(u8);

impl Attributes {
    pub fn read_only(&self) -> bool {
        self.0 & 0x01 != 0
    }

    pub fn hidden(&self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn system(&self) -> bool {
        self.0 & 0x04 != 0
    }

    pub fn volume_id(&self) -> bool {
        self.0 & 0x08 != 0
    }

    pub fn directory(&self) -> bool {
        self.0 & 0x10 != 0
    }

    pub fn archive(&self) -> bool {
        self.0 & 0x20 != 0
    }

    pub fn lfn(&self) -> bool {
        self.read_only() && self.hidden() && self.system() && self.volume_id()
    }
}

/// A structure containing a date and time.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timestamp {
    pub date: Date,
    pub time: Time,
    pub addtional_in_10ms: u8, // addtional time with 10ms granularity, range: [0, 199)
}

/// Metadata for a directory entry.
#[derive(Default, Debug, Clone)]
pub struct Metadata {
    // FIXME: Fill me in.
    pub attributes: Attributes,
    pub created: Timestamp,
    pub accessed: Timestamp,
    pub modified: Timestamp,
}

// FIXME: Implement `traits::Timestamp` for `Timestamp`.
impl traits::Timestamp for Timestamp {
    fn year(&self) -> usize {
        self.date.year()
    }

    fn month(&self) -> u8 {
        self.date.month()
    }

    fn day(&self) -> u8 {
        self.date.day()
    }

    fn hour(&self) -> u8 {
        self.time.hour()
    }

    fn minute(&self) -> u8 {
        self.time.minute()
    }

    fn second(&self) -> u8 {
        // additional time, range: [0, 2] seconds
        let add = (self.addtional_in_10ms as f64 / 100.0).round() as u8;
        self.time.second() + add
    }
}

// FIXME: Implement `traits::Metadata` for `Metadata`.
impl traits::Metadata for Metadata {
    type Timestamp = Timestamp;

    fn read_only(&self) -> bool {
        self.attributes.read_only()
    }

    fn hidden(&self) -> bool {
        self.attributes.hidden()
    }

    fn created(&self) -> Self::Timestamp {
        self.created
    }

    fn accessed(&self) -> Self::Timestamp {
        self.accessed
    }

    fn modified(&self) -> Self::Timestamp {
        self.modified
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use traits::Timestamp;

        f.write_fmt(format_args!(
            "{:>4}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second()
        ))
    }
}

// FIXME: Implement `fmt::Display` (to your liking) for `Metadata`.
impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use traits::Metadata;

        f.write_fmt(format_args!(
            "metadata: read-only={} hidden={} created={} accessed={} modified={}",
            self.read_only(),
            self.hidden(),
            self.created(),
            self.accessed(),
            self.modified()
        ))
    }
}
