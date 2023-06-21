use std::{fmt, io};

use traits::BlockDevice;
use vfat::Error;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    pub jump_boot: [u8; 3], // Must be 0xEB, 0x76, 0x90.
    pub oem_identifier: [u8; 8],
    pub bytes_per_sector: u16, // logical sector size
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16, // The number of logical sectors before the first FAT in the file system image. At least 1 for this sector, usually 32 for FAT32
    pub number_of_fats: u8,
    pub max_root_entries: u16, // 0 for FAT32, which stores directories in data region
    pub total_sectors_16: u16, // if zero, use 4 byte value at offset 32
    pub media: u8,
    pub sectors_per_fat_16: u16, //  0 for FAT32; use 32-bit value at 36 instead
    pub sectors_per_track: u16,
    pub number_of_heads: u16,
    pub hidden_sectors: u32,   // the LBA of the beginning of the partition
    pub total_sectors_32: u32, // if greater than 65535; otherwise, see offset 19

    // Below is Extended BPB
    pub sectors_per_fat_32: u32,
    pub ext_flags: u16, // only for FAT32
    pub fat_version: [u8; 2],
    pub root_dir_cluster: u32, // the cluster number of the root directory. Often this field is set to 2.
    pub fsinfo_sector: u16,
    pub backup_boot_sector: u16,
    pub reserved: [u8; 12],
    pub drive_number: u8,
    pub reserved1: u8,
    pub ext_boot_signature: u8, // should be 0x28 or 0x29
    pub volume_id: u32,
    pub volume_label: [u8; 11],
    pub fs_type: [u8; 8], // always "FAT32   "
    pub boot_code: [u8; 420],
    pub boot_sector_signature: [u8; 2], // 0x55 0xAA
}

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64) -> Result<BiosParameterBlock, Error> {
        let mut buf = [0u8; 512];
        assert!(device.sector_size() >= 512);

        let n = device
            .read_sector(sector, &mut buf)
            .map_err(|err| Error::Io(err))?;
        if n < 512 {
            return Err(Error::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "BPB short read",
            )));
        }

        let bpb = unsafe { core::mem::transmute::<[u8; 512], BiosParameterBlock>(buf) };

        if bpb.boot_sector_signature != [0x55, 0xAA] {
            return Err(Error::BadSignature);
        }

        Ok(bpb)
    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sectors_per_fat = if self.sectors_per_fat_16 == 0 {
            self.sectors_per_fat_32
        } else {
            self.sectors_per_fat_16 as u32
        };

        f.debug_struct("BPB")
            .field(
                "Logical Sector Size",
                &{ self.bytes_per_sector }.to_string(),
            )
            .field(
                "Sectors Per Cluster",
                &{ self.sectors_per_cluster }.to_string(),
            )
            .field("Number of FATs", &self.number_of_fats.to_string())
            .field("Sectors per FAT", &sectors_per_fat.to_string())
            .finish()
    }
}
