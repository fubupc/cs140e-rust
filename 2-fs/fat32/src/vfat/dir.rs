use std::char::{decode_utf16, DecodeUtf16Error};
use std::ffi::OsStr;
use std::io;
use std::mem::size_of;

use traits;
use util::VecExt;
use vfat::{Attributes, Date, Metadata, Time, Timestamp};
use vfat::{Cluster, Entry, File, Shared, VFat};

#[derive(Debug)]
pub struct Dir {
    pub long_name: Option<String>,
    pub short_name: String,
    pub metadata: Metadata,

    pub(super) start_cluster: Cluster,
    pub(super) vfat: Shared<VFat>,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatRegularDirEntry {
    // File name: 8 ASCII characters.
    // A file name may be terminated early using 0x00 or 0x20 characters.
    // If the file name starts with 0x00, the previous entry was the last entry.
    // If the file name starts with 0xE5, this is a deleted/unused entry.
    file_name: [u8; 8],

    // File extension: 3 ASCII characters.
    // A file extension may be terminated early using 0x00 or 0x20 characters.
    file_ext: [u8; 3],

    attributes: Attributes,

    reserved: u8, // Reserved for use by Windows NT.

    // FIXME: There are 2 conflict description of this field:
    // 1. Creation time in tenths of a second. Range 0-199 inclusive. Ubuntu uses 0-100.
    // 2. Create time, fine resolution: 10 ms units, values from 0 to 199 (since DOS 7.0 with VFAT).
    // Seems 2 makes more sense.
    created_in_10ms: u8,

    created_time: Time,
    created_date: Date,
    accessed_date: Date,
    first_cluster_high: u16, // High 16 bits of first cluster number
    modified_time: Time,
    modified_date: Date,
    first_cluster_low: u16, // Low 16 bits of first cluster number
    file_size: u32,         // In bytes
}

impl VFatRegularDirEntry {
    fn name(&self) -> io::Result<String> {
        let name = Self::parse_str(&self.file_name)?;
        if name.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "empty filename"));
        };

        let ext = Self::parse_str(&self.file_ext)?;
        if ext.len() == 0 {
            Ok(name.to_string())
        } else {
            Ok(format!("{}.{}", name, ext))
        }
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            attributes: self.attributes,
            created: Timestamp {
                date: self.created_date,
                time: self.created_time,
                addtional_in_10ms: self.created_in_10ms,
            },
            accessed: Timestamp {
                date: self.accessed_date,
                time: Time::zero(),
                addtional_in_10ms: 0,
            },
            modified: Timestamp {
                date: self.modified_date,
                time: self.modified_time,
                addtional_in_10ms: 0,
            },
        }
    }

    fn first_cluster(&self) -> Cluster {
        Cluster::from((self.first_cluster_high as u32) << 16 | self.first_cluster_low as u32)
    }

    fn parse_str(s: &[u8]) -> io::Result<&str> {
        std::str::from_utf8(&s[..Self::str_len(s)])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid string"))
    }

    fn str_len(s: &[u8]) -> usize {
        match s.iter().position(Self::is_terminate_char) {
            Some(pos) => pos,
            None => s.len(),
        }
    }

    fn is_terminate_char(&b: &u8) -> bool {
        b == 0x00 || b == 0x20
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatLfnDirEntry {
    // Sequence Number:
    // Bit 6 set: last logical LFN entry, first physical LFN entry
    // Bit 5 clear
    // Bits 4-0: from 0x01..0x14 (0x1F): position of entry
    // If the sequence number is 0x00, the previous entry was the last entry.
    // If the sequence number is 0xE5, this is a deleted/unused entry.
    sequence_number: u8,

    // Name characters (five UCS-2 (subset of UTF-16) characters)
    // A file name may be terminated early using 0x00 or 0xFF characters.
    name_part1: [u16; 5],

    attributes: Attributes,
    r#type: u8,           // Always 0x00 for VFAT LFN, other values reserved for future use
    checksum: u8,         // Checksum of DOS file name
    name_part2: [u16; 6], // 6 UCS-2 characters
    first_cluster: u16,   // First cluster (always 0x0000)
    name_part3: [u16; 2], // 2 UCS-2 characters
}

impl VFatLfnDirEntry {
    fn last_logical(&self) -> bool {
        self.sequence_number & 0x40 != 0
    }

    fn position(&self) -> usize {
        (self.sequence_number & 0x1F) as usize
    }

    fn name(&self) -> io::Result<String> {
        let p1 = Self::parse_str(&{ self.name_part1 })?;
        let p2 = Self::parse_str(&{ self.name_part2 })?;
        let p3 = Self::parse_str(&{ self.name_part3 })?;
        Ok(p1 + &p2 + &p3)
    }

    fn parse_str(s: &[u16]) -> io::Result<String> {
        decode_utf16(s[..Self::str_len(s)].iter().copied())
            .collect::<Result<String, DecodeUtf16Error>>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid long file name"))
    }

    fn str_len(s: &[u16]) -> usize {
        match s.iter().position(Self::is_terminate_char) {
            Some(pos) => pos,
            None => s.len(),
        }
    }

    fn is_terminate_char(&b: &u16) -> bool {
        b == 0x0000 || b == 0xFFFF
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatUnknownDirEntry {
    // The first byte of an entry (whether regular or LFN) is also known as the ID.
    // ID of 0x00. Indicates the end of the directory.
    // ID of 0xE5: Marks an unused/deleted entry.
    // All other IDs make up part of the file’s name or LFN sequence number
    id: u8,

    _ignore: [u8; 10],
    attributes: Attributes,
    _ignore2: [u8; 20],
}

pub union VFatDirEntry {
    unknown: VFatUnknownDirEntry,
    regular: VFatRegularDirEntry,
    long_filename: VFatLfnDirEntry,
}

impl Dir {
    /// Finds the entry named `name` in `self` and returns it. Comparison is
    /// case-insensitive.
    ///
    /// # Errors
    ///
    /// If no entry with name `name` exists in `self`, an error of `NotFound` is
    /// returned.
    ///
    /// If `name` contains invalid UTF-8 characters, an error of `InvalidInput`
    /// is returned.
    pub fn find<P: AsRef<OsStr>>(&self, name: P) -> io::Result<Entry> {
        use traits::{Dir, Entry};

        let name = name
            .as_ref()
            .to_str()
            .ok_or(io::Error::new(io::ErrorKind::InvalidInput, ""))?;

        self.entries()?
            .find(|e| e.name().eq_ignore_ascii_case(name))
            .ok_or(io::ErrorKind::NotFound.into())
    }
}

impl traits::Dir for Dir {
    type Entry = Entry;

    type Iter = EntryIter;

    fn entries(&self) -> io::Result<Self::Iter> {
        let mut buf: Vec<u8> = Vec::new();

        let size = self
            .vfat
            .borrow_mut()
            .read_chain(self.start_cluster, &mut buf)?;

        assert!(size % size_of::<VFatDirEntry>() == 0);

        Ok(EntryIter {
            entries: unsafe { buf.cast() },
            next: 0,
            vfat: self.vfat.clone(),
        })
    }
}

pub struct EntryIter {
    entries: Vec<VFatDirEntry>,
    next: usize,
    vfat: Shared<VFat>,
}

impl Iterator for EntryIter {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.entries.len() {
            let entry = &self.entries[self.next];
            match unsafe { entry.unknown.id } {
                0x00 => return None,    // 0x00: end of directory
                0xE5 => self.next += 1, // 0xE5: unused/deleted entry
                _ => {
                    let mut long_name: Option<String> = None;
                    if unsafe { entry.unknown.attributes.lfn() } {
                        let (name, lfn_entry_num) = self.parse_lfn(self.next).unwrap();
                        self.next += lfn_entry_num; // make self.next point to the regular entry after lfn entries
                        long_name = Some(name)
                    }

                    let regular = unsafe { self.entries[self.next].regular };
                    self.next += 1;

                    if regular.attributes.directory() {
                        return Some(Entry::Dir(Dir {
                            long_name,
                            short_name: regular.name().unwrap(),
                            metadata: regular.metadata(),
                            start_cluster: regular.first_cluster(),
                            vfat: self.vfat.clone(),
                        }));
                    } else {
                        return Some(Entry::File(File {}));
                    }
                }
            }
        }

        None
    }
}

impl EntryIter {
    // Returns the parsed long filename and number of lfn entries
    fn parse_lfn(&self, start_entry: usize) -> io::Result<(String, usize)> {
        let start = unsafe { self.entries[start_entry].long_filename };
        assert!(start.attributes.lfn());
        assert!(start.last_logical());
        assert!(start.position() >= 1);
        // Needs an additional entry space for regular entry
        assert!(start.position() + 1 <= self.entries.len() - start_entry);

        let name = self.entries[start_entry..start_entry + start.position()]
            .iter()
            .rev() // LFN entries are ordered reversely
            .enumerate()
            .map(|(i, e)| {
                let e = unsafe { &e.long_filename };
                assert!(e.attributes.lfn());
                assert!(e.position() == i + 1);
                e.name()
            })
            .collect::<io::Result<String>>()?;

        Ok((name, start.position()))
    }
}
