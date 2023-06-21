use core::convert::TryInto;
use std::cmp::min;
use std::io::{self, Read, Write};
use std::mem::size_of;
use std::path::Path;

use mbr::MasterBootRecord;
use traits::{BlockDevice, FileSystem};
use util::SliceExt;
use vfat::{BiosParameterBlock, CachedDevice, Partition};
use vfat::{Cluster, Dir, Entry, Error, FatEntry, File, Shared, Status};

use super::cluster;

const FAT_ENTRY_SIZE: u64 = size_of::<FatEntry>() as u64;

#[derive(Debug)]
pub struct VFat {
    device: CachedDevice,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    root_dir_cluster: Cluster,
}

impl VFat {
    pub fn from<T>(mut device: T) -> Result<Shared<VFat>, Error>
    where
        T: BlockDevice + 'static,
    {
        let mbr = MasterBootRecord::from(&mut device)?;

        // Locate the first FAT32 partition
        let pe = mbr
            .partitions
            .iter()
            .find(|p| matches!(p.partition_type, 0xB | 0xC))
            .ok_or(Error::NotFound)?;

        let bpb = BiosParameterBlock::from(&mut device, pe.relative_sector as u64)?;

        if pe.total_sectors as u64
            != (bpb.bytes_per_sector as u64 * bpb.total_sectors_32 as u64 / device.sector_size())
        {
            panic!("partition size between MBR and BPB mismatch");
        }

        // Some entries might be empty in the last secotor of FAT
        let max_clusters =
            bpb.sectors_per_fat_32 as u64 * bpb.bytes_per_sector as u64 / FAT_ENTRY_SIZE;
        assert!(
            bpb.total_sectors_32 as u64
                <= bpb.reserved_sectors as u64
                    + bpb.number_of_fats as u64 * bpb.sectors_per_fat_32 as u64
                    + bpb.sectors_per_cluster as u64 * max_clusters
        );

        let partition = Partition {
            start: pe.relative_sector as u64, // physical starting sector of partition
            sector_size: bpb.bytes_per_sector as u64,
        };

        Ok(Shared::new(VFat {
            device: CachedDevice::new(device, partition),
            bytes_per_sector: bpb.bytes_per_sector,
            sectors_per_cluster: bpb.sectors_per_cluster,
            sectors_per_fat: bpb.sectors_per_fat_32,
            fat_start_sector: bpb.reserved_sectors as u64,
            data_start_sector: bpb.reserved_sectors as u64
                + bpb.number_of_fats as u64 * bpb.sectors_per_fat_32 as u64,
            root_dir_cluster: Cluster::from(bpb.root_dir_cluster),
        }))
    }

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
    //    fn read_cluster(
    //        &mut self,
    //        cluster: Cluster,
    //        offset: usize,
    //        buf: &mut [u8]
    //    ) -> io::Result<usize>;
    //
    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
    //    fn read_chain(
    //        &mut self,
    //        start: Cluster,
    //        buf: &mut Vec<u8>
    //    ) -> io::Result<usize>;
    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //
    //    fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry>;
}

impl<'a> FileSystem for &'a Shared<VFat> {
    type File = ::traits::Dummy;
    type Dir = ::traits::Dummy;
    type Entry = ::traits::Dummy;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        unimplemented!("FileSystem::open()")
    }

    fn create_file<P: AsRef<Path>>(self, _path: P) -> io::Result<Self::File> {
        unimplemented!("read only file system")
    }

    fn create_dir<P>(self, _path: P, _parents: bool) -> io::Result<Self::Dir>
    where
        P: AsRef<Path>,
    {
        unimplemented!("read only file system")
    }

    fn rename<P, Q>(self, _from: P, _to: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        unimplemented!("read only file system")
    }

    fn remove<P: AsRef<Path>>(self, _path: P, _children: bool) -> io::Result<()> {
        unimplemented!("read only file system")
    }
}
