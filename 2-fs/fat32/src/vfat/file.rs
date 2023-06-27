use std::cmp::{max, min};
use std::io::{self, SeekFrom};

use traits;
use vfat::{Cluster, Metadata, Shared, VFat};

use super::Status;

#[derive(Debug)]
pub struct File {
    // FIXME: Fill me in.
    pub(super) long_name: Option<String>,
    pub(super) short_name: String,
    pub metadata: Metadata,
    pub(super) file_size: u32, // in bytes

    pub(super) vfat: Shared<VFat>,
    pub(super) absolute_offset: u32, // current absolute offset in file, in bytes
    pub(super) start_cluster: Cluster,
    pub(super) curr_cluster: Cluster,
}

impl File {
    pub fn name(&self) -> &str {
        self.long_name.as_ref().unwrap_or(&self.short_name)
    }
}

// FIXME: Implement `traits::File` (and its supertraits) for `File`.
impl traits::File for File {
    fn sync(&mut self) -> io::Result<()> {
        todo!()
    }

    fn size(&self) -> u64 {
        self.file_size as u64
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.absolute_offset == self.file_size {
            return Ok(0);
        }

        let mut vfat = self.vfat.borrow_mut();
        let mut curr_buf = buf;
        let mut total = 0;
        while !curr_buf.is_empty() {
            let offset_in_cluster = self.absolute_offset as usize % vfat.cluster_size();
            let mut n = vfat.read_cluster(self.curr_cluster, offset_in_cluster, curr_buf)?;
            if self.absolute_offset as usize + n > self.file_size as usize {
                n = (self.file_size - self.absolute_offset) as usize
            }

            self.absolute_offset += n as u32;
            total += n;
            curr_buf = &mut curr_buf[n..];

            match vfat.fat_entry(self.curr_cluster)?.status() {
                Status::Eoc(_) => {
                    if curr_buf.len() > 0 && self.absolute_offset < self.file_size {
                        // File size and the actual end mismatch
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                    } else {
                        // Reached to the end of file
                        return Ok(total);
                    }
                }
                Status::Data(next) => {
                    // Only change `curr_cluster` when `absolute_offset` moved across cluster boundary.
                    if self.absolute_offset / vfat.cluster_size() as u32
                        > (self.absolute_offset - n as u32) / vfat.cluster_size() as u32
                    {
                        self.curr_cluster = next;
                    }
                }
                _ => return Err(io::ErrorKind::InvalidData.into()),
            }
        }

        Ok(total)
    }
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl io::Seek for File {
    /// Seek to offset `pos` in the file.
    ///
    /// A seek to the end of the file is allowed. A seek _beyond_ the end of the
    /// file returns an `InvalidInput` error.
    ///
    /// If the seek operation completes successfully, this method returns the
    /// new position from the start of the stream. That position can be used
    /// later with SeekFrom::Start.
    ///
    /// # Errors
    ///
    /// Seeking before the start of a file or beyond the end of the file results
    /// in an `InvalidInput` error.
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let absolute_offset = match pos {
            SeekFrom::Start(start) => {
                if start > self.file_size as u64 {
                    return Err(io::ErrorKind::InvalidInput.into());
                } else {
                    start as u32
                }
            }
            SeekFrom::End(end) => {
                if end > 0 || -end as u64 > self.file_size as u64 {
                    return Err(io::ErrorKind::InvalidInput.into());
                } else {
                    (self.file_size as i64 + end) as u32
                }
            }
            SeekFrom::Current(curr) => {
                let absolute_offset = self.absolute_offset as i64 + curr;
                if absolute_offset < 0 || absolute_offset as u64 > self.file_size as u64 {
                    return Err(io::ErrorKind::InvalidInput.into());
                } else {
                    absolute_offset as u32
                }
            }
        };

        // Below: absolute_offset <= file size
        let cluster_size = self.vfat.borrow().cluster_size() as u32;
        let mut curr_cluster = self.start_cluster;
        let mut curr_offset = 0;
        while curr_offset + cluster_size <= absolute_offset {
            match self.vfat.borrow_mut().fat_entry(curr_cluster)?.status() {
                Status::Eoc(_) => {
                    if curr_offset + cluster_size == absolute_offset
                        && absolute_offset == self.file_size
                    {
                        break;
                    }
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }
                Status::Data(next) => {
                    curr_cluster = next;
                    curr_offset += cluster_size
                }
                _ => return Err(io::ErrorKind::InvalidData.into()),
            }
        }

        self.absolute_offset = absolute_offset;
        self.curr_cluster = curr_cluster;

        Ok(absolute_offset as u64)
    }
}
