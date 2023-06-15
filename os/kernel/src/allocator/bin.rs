use alloc::alloc::{AllocError, Layout};
use core::cmp::max;
use core::fmt::Debug;
use core::panic;

use crate::allocator::linked_list::LinkedList;
use crate::allocator::{pool, util::*};

const K: usize = 16;

/// A simple allocator that allocates based on size classes.
pub struct Allocator {
    // block size of bins[k] = 2^(k+3)
    // alignment = block size
    bins: [LinkedList; K - 2],

    pool: pool::Allocator,
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        Allocator {
            bins: [LinkedList::new(); K - 2],
            pool: pool::Allocator::new(start, end),
        }
    }

    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
        let block_size = self.block_size_fit(layout);
        match self.first_bin_fit(block_size) {
            Some(first_bin_idx) => {
                // allocate a block if possible from the first bin fit
                if let Some(addr) = self.bins[first_bin_idx].pop() {
                    return Ok(addr as *mut u8);
                }
                // allocate by splitting bigger bin
                for i in (first_bin_idx + 1)..self.bins.len() {
                    // split bigger bin and returns a request-sized block
                    if let Some(addr) = unsafe { self.split_bin(i, first_bin_idx) } {
                        return Ok(addr);
                    }
                }
                // allocate from pool if no bin fit
                self.pool
                    .alloc(Layout::from_size_align(block_size, block_size).unwrap())
            }
            None => self.pool.alloc(layout), // too-big block cannot fit in any bin
        }
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let block_size = self.block_size_fit(layout);
        match self.first_bin_fit(block_size) {
            Some(fit_bin) => unsafe { self.bins[fit_bin].push(ptr as *mut usize) },
            None => self.pool.dealloc(ptr, layout),
        }
    }

    unsafe fn split_bin(&mut self, big_bin: usize, small_bin: usize) -> Option<*mut u8> {
        let addr = self.bins[big_bin].pop()? as *mut u8;

        for i in small_bin..big_bin {
            let bin_addr = addr.add(self.bin_block_size(i));
            self.bins[i].push(bin_addr as *mut usize)
        }

        Some(addr)
    }

    fn bin_block_size(&self, bin_index: usize) -> usize {
        1 << (bin_index + 3)
    }

    fn first_bin_fit(&self, block_size: usize) -> Option<usize> {
        let exp = block_size.ilog2() as usize;
        match exp {
            0..=3 => Some(0),
            4..=K => Some(exp - 3),
            _ => None,
        }
    }

    fn block_size_fit(&self, layout: Layout) -> usize {
        max(layout.size().next_power_of_two(), layout.align())
    }
}

// FIXME: Implement `Debug` for `Allocator`.
impl Debug for Allocator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Bins:\n")?;
        for (bin_index, bin) in self.bins.iter().enumerate() {
            let block_size = self.bin_block_size(bin_index);
            f.write_fmt(format_args!("  Bin={:<6}:", block_size))?;
            if bin.is_empty() {
                f.write_str(" <empty>")?;
            } else {
                for ptr in bin.iter() {
                    f.write_fmt(format_args!(" {:#X}", ptr as usize))?;
                }
            }
            f.write_str("\n")?;
        }
        f.write_fmt(format_args!("Pool: {:?}\n", &self.pool))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Allocator;
    use core::alloc::Layout;

    #[test]
    fn test_allocator() {
        let mut v = vec![0u8; 64];
        let ptr_range = v.as_mut_ptr_range();
        println!(
            "Memory Range: [{:#X}, {:#X})",
            ptr_range.start as usize, ptr_range.end as usize
        );

        let mut allocator = Allocator::new(ptr_range.start as usize, ptr_range.end as usize);
        println!("Init:\n{:?}", allocator);

        let l1 = Layout::from_size_align(1, 32).unwrap();
        let a1 = allocator.alloc(l1).unwrap();
        println!("Alloc 1:\n{:?}", allocator);

        allocator.dealloc(a1, l1);
        println!("Dealloc 1:\n{:?}", allocator);

        let l2 = Layout::from_size_align(1, 8).unwrap();
        let a2 = allocator.alloc(l2).unwrap();
        println!("Alloc 2:\n{:?}", allocator);

        allocator.dealloc(a2, l2);
        println!("Dealloc 1:\n{:?}", allocator);
    }
}
