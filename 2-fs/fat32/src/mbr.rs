use std::{fmt, io};

use traits::BlockDevice;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct CHS {
    pub head: u8,
    pub sector_and_cylinder: [u8; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct PartitionEntry {
    pub boot_indicator: u8,
    pub starting_chs: CHS,
    pub partition_type: u8,
    pub ending_chs: CHS,
    pub relative_sector: u32, // starting LBA value
    pub total_sectors: u32,
}

/// The master boot record (MBR).
#[repr(C, packed)]
pub struct MasterBootRecord {
    pub bootstrap: [u8; 436],
    pub disk_id: [u8; 10],
    pub partitions: [PartitionEntry; 4],
    pub signature: [u8; 2],
}

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partiion `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

impl MasterBootRecord {
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut buf = [0u8; 512];
        assert!(device.sector_size() >= 512);

        let n = device
            .read_sector(0, &mut buf)
            .map_err(|err| Error::Io(err))?;
        if n < 512 {
            return Err(Error::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "MBR short read",
            )));
        }

        let mbr = unsafe { core::mem::transmute::<[u8; 512], MasterBootRecord>(buf) };

        if mbr.signature != [0x55, 0xAA] {
            return Err(Error::BadSignature);
        }

        for (i, p) in mbr.partitions.iter().enumerate() {
            if p.boot_indicator != 0x00 && p.boot_indicator != 0x80 {
                return Err(Error::UnknownBootIndicator(i as u8));
            }
        }

        Ok(mbr)
    }
}

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MBR")
            .field(
                "signature",
                &format!("{:#X} {:#X}", self.signature[0], self.signature[1]),
            )
            .field(
                "disk_id",
                &std::str::from_utf8(&self.disk_id).map_err(|e| fmt::Error)?,
            )
            .finish()
    }
}
