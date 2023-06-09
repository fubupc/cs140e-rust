use core::convert::TryInto;
use std::cmp::min;
use std::io::{self, Read, Write};
use std::mem::size_of;
use std::path::{Component, Path};

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
            fat_start_sector: pe.relative_sector as u64 + bpb.reserved_sectors as u64,
            data_start_sector: pe.relative_sector as u64
                + bpb.reserved_sectors as u64
                + bpb.number_of_fats as u64 * bpb.sectors_per_fat_32 as u64,
            root_dir_cluster: Cluster::from(bpb.root_dir_cluster),
        }))
    }

    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
    pub fn read_cluster(
        &mut self,
        cluster: Cluster,
        offset: usize,
        buf: &mut [u8],
    ) -> io::Result<usize> {
        match self.fat_entry(cluster)?.status() {
            Status::Data(_) | Status::Eoc(_) => {
                let start_sector = self.cluster_start_sector(cluster.inner());

                let sector_offset = offset / (self.bytes_per_sector as usize);
                let offset_in_sector = offset % (self.bytes_per_sector as usize); // in bytes

                assert!(sector_offset < self.sectors_per_cluster as usize);

                let mut total = 0;
                let mut buf = buf;
                for i in sector_offset as u64..self.sectors_per_cluster as u64 {
                    let mut sector = self.device.get(start_sector + i)?;
                    let n = if i == sector_offset as u64 {
                        (&sector[offset_in_sector..]).read(buf)?
                    } else {
                        sector.read(buf)?
                    };
                    total += n;
                    buf = &mut buf[n..];
                    if buf.len() == 0 {
                        return Ok(total);
                    }
                }
                Ok(total)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("cluster {} is not data cluster", cluster.inner()),
            )),
        }
    }

    //
    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
    pub fn read_chain(&mut self, start: Cluster, buf: &mut Vec<u8>) -> io::Result<usize> {
        let init_len = buf.len();
        let mut curr = start;
        let mut total = 0;

        loop {
            buf.resize(buf.len() + self.cluster_size(), 0x00);
            assert!(buf.len() >= init_len + total + self.cluster_size());

            let n = self.read_cluster(curr, 0, &mut buf[init_len + total..])?;
            assert_eq!(n, self.cluster_size());
            total += n;

            match self.fat_entry(curr)?.status() {
                Status::Eoc(_) => return Ok(total),
                Status::Data(next) => curr = next,
                _ => return Err(io::ErrorKind::InvalidData.into()),
            }
        }
    }

    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //
    pub fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {
        let cluster = cluster.inner();

        let sector_offset = (cluster as u64 * FAT_ENTRY_SIZE) / (self.bytes_per_sector as u64);
        let byte_offset = (cluster as u64 * FAT_ENTRY_SIZE) % (self.bytes_per_sector as u64);

        let sector = self.device.get(self.fat_start_sector + sector_offset)?;

        let entry = unsafe {
            &*(&sector[byte_offset as usize] as *const u8 as *const u32 as *const FatEntry)
        };

        Ok(entry)
    }

    pub fn cluster_size(&self) -> usize {
        self.sectors_per_cluster as usize * self.bytes_per_sector as usize
    }

    fn cluster_start_sector(&self, cluster: u32) -> u64 {
        self.data_start_sector + self.sectors_per_cluster as u64 * (cluster as u64 - 2)
    }
}

impl<'a> FileSystem for &'a Shared<VFat> {
    type File = File;
    type Dir = Dir;
    type Entry = Entry;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        let root = Entry::Dir(Dir::root(self.borrow().root_dir_cluster, self.clone()));

        match path.as_ref().components().collect::<Vec<_>>().as_slice() {
            [] => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid path: empty",
            )),
            [first, lasts @ ..] => {
                if first != &Component::RootDir {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("invalid path: not an absolute path: {:?}", path.as_ref()),
                    ));
                }

                let mut parents = vec![root];
                // Loop invariant: `parents` contains at least one element (the root dir)
                for c in lasts {
                    match c {
                        Component::Prefix(_) => return Err(io::ErrorKind::InvalidInput.into()),
                        Component::RootDir => {
                            panic!("[bug] root dir should not be in the middle of path")
                        }
                        Component::CurDir => continue, // Dir '.'
                        Component::ParentDir => {
                            // Dir '..'
                            if parents.len() == 1 {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidInput,
                                    "invalid path: beyond root directory",
                                ));
                            }
                            parents.pop().unwrap();
                        }
                        Component::Normal(c) => {
                            let entry = match parents.last().unwrap() {
                                Entry::Dir(dir) => dir.find(c)?,
                                Entry::File(_) => {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidInput,
                                        "invalid path: not directory",
                                    ))
                                }
                            };
                            parents.push(entry);
                        }
                    }
                }

                Ok(parents.pop().unwrap())
            }
        }
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
