use std::fmt;
use vfat::*;

use self::Status::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    /// The FAT entry corresponds to an unused (free) cluster.
    Free,
    /// The FAT entry/cluster is reserved.
    Reserved,
    /// The FAT entry corresponds to a valid data cluster. The next cluster in
    /// the chain is `Cluster`.
    Data(Cluster),
    /// The FAT entry corresponds to a bad (disk failed) cluster.
    Bad,
    /// The FAT entry corresponds to a valid data cluster. The corresponding
    /// cluster is the last in its chain.
    Eoc(u32),
}

#[repr(C, packed)]
pub struct FatEntry(pub u32);

impl FatEntry {
    /// Returns the `Status` of the FAT entry `self`.
    pub fn status(&self) -> Status {
        match self.0 & 0x0FFFFFFF {
            0x00000000 => Status::Free,
            0x00000001 => Status::Reserved,
            c @ 0x00000002..=0x0FFFFFEF => Status::Data(Cluster::from(c)),
            0x0FFFFFF0..=0x0FFFFFF5 => Status::Reserved,
            0x0FFFFFF6 => Status::Reserved,
            0x0FFFFFF7 => Status::Bad,
            e @ 0x0FFFFFF8..=0x0FFFFFFF => Status::Eoc(e),
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for FatEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FatEntry")
            .field("value", &{ self.0 })
            .field("status", &self.status())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::vfat::{Cluster, FatEntry, Status};

    #[test]
    fn test_status() {
        assert_eq!(FatEntry(0x00000000).status(), Status::Free);
        assert_eq!(FatEntry(0x10000000).status(), Status::Free);

        assert_eq!(
            FatEntry(0x000001F6).status(),
            Status::Data(Cluster::from(0x000001F6))
        );
        assert_eq!(
            FatEntry(0x200001E2).status(),
            Status::Data(Cluster::from(0x000001E2))
        );

        assert_eq!(FatEntry(0x0FFFFFF8).status(), Status::Eoc(0x0FFFFFF8));
        assert_eq!(FatEntry(0x3FFFFFF9).status(), Status::Eoc(0x0FFFFFF9));
    }
}
