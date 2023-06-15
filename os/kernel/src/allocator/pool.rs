use core::{
    alloc::{AllocError, Layout},
    fmt::{Alignment, Debug},
    mem,
};

use super::util::{align_down, align_up};

const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}
const MIN_BLOCK_SIZE: usize = max(mem::align_of::<Node>(), mem::size_of::<Node>());

struct Node {
    size: usize,
    next: *mut Node,
}

pub struct Allocator {
    // Head node is a sentinel, not part of heap memory, while all other nodes are on the heap.
    head: Node,

    start: usize,
    end: usize,
}

impl Allocator {
    pub fn new(start: usize, end: usize) -> Allocator {
        let aligned_start = align_up(start, MIN_BLOCK_SIZE);
        let aligned_end = align_down(end, MIN_BLOCK_SIZE);
        if aligned_end - aligned_start <= 0 {
            panic!("no enough memory to initialize pool::Allocator")
        }
        let node = aligned_start as *mut Node;
        unsafe {
            (*node).size = aligned_end - aligned_start;
            (*node).next = core::ptr::null_mut();
        }
        Allocator {
            head: Node {
                size: 0,
                next: node,
            },
            start,
            end,
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
        let mut prev = (&mut self.head) as *mut Node;
        let mut curr = self.head.next;

        while !curr.is_null() {
            let curr_start = curr as usize;
            let curr_end = curr as usize + unsafe { (*curr).size };

            let next = unsafe { (*curr).next };

            let aligned_start = align_up(curr_start, layout.align());
            let aligned_end = align_up(aligned_start + layout.size(), MIN_BLOCK_SIZE);

            if curr_end >= aligned_end {
                if aligned_start > curr_start {
                    assert!((aligned_start - curr_start) % MIN_BLOCK_SIZE == 0);
                    unsafe {
                        (*curr).size = aligned_start - curr_start;
                        (*curr).next = core::ptr::null_mut();
                    }
                    prev = curr;
                }

                if curr_end == aligned_end {
                    unsafe { (*prev).next = next }
                    return Ok(aligned_start as *mut u8);
                } else {
                    let new_node = aligned_end as *mut Node;
                    unsafe {
                        (*new_node).size = curr_end - aligned_end;
                        (*new_node).next = next;
                        (*prev).next = new_node;
                    };
                    return Ok(aligned_start as *mut u8);
                }
            }

            prev = curr;
            curr = next;
        }

        Err(AllocError)
    }

    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let start = ptr as usize;
        let request_end = start + layout.size();
        let actual_end = align_up(request_end, MIN_BLOCK_SIZE);

        let mut prev = (&mut self.head) as *mut Node;
        let mut curr = self.head.next;

        // Node is sorted by address
        while !curr.is_null() && start > (curr as usize) {
            prev = curr;
            curr = unsafe { (*curr).next };
        }

        let new_node = start as *mut Node;
        unsafe {
            (*new_node).size = actual_end - start;
            (*new_node).next = curr;
            (*prev).next = new_node;

            self.merge_adjacent_regions();
        }
    }

    unsafe fn merge_adjacent_regions(&mut self) {
        let mut curr = self.head.next;
        while !curr.is_null() {
            let next = (*curr).next;
            if next.is_null() {
                break;
            }

            let curr_end = curr as usize + (*curr).size;
            let next_start = next as usize;

            if curr_end == next_start {
                // curr and next are adjacent, merge
                (*curr).size = (*curr).size + (*next).size;
                (*curr).next = (*next).next;
            } else {
                // not adjacent, skip
                curr = next;
            }
        }
    }
}

impl Debug for Allocator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut prev_addr = self.start;
        let mut curr = self.head.next;
        while !curr.is_null() {
            let curr_addr = curr as usize;
            let (size, next) = unsafe { ((*curr).size, (*curr).next) };

            if curr_addr > prev_addr {
                f.write_fmt(format_args!(
                    "({:#X},{}) ",
                    prev_addr,
                    curr_addr - prev_addr
                ))?;
            }

            f.write_fmt(format_args!("[{:#X},{}] ", curr_addr, size))?;

            prev_addr = curr_addr + size;
            curr = next;
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Allocator, MIN_BLOCK_SIZE};
    use core::{alloc::Layout, cell::RefCell};

    #[test]
    fn test_pool() {
        println!("MIN_BLOCK_SIZE: {}", MIN_BLOCK_SIZE);
        let mut v = vec![0u8; 64];
        let ptr_range = v.as_mut_ptr_range();
        println!(
            "Memory Range: [{:#X}, {:#X})",
            ptr_range.start as usize, ptr_range.end as usize
        );

        let mut allocator = Allocator::new(ptr_range.start as usize, ptr_range.end as usize);
        println!("Init: {:?}", allocator);

        let l1 = Layout::from_size_align(1, 8).unwrap();
        let a1 = allocator.alloc(l1).unwrap();
        println!("Alloc 1: {:?}", allocator);

        let l2 = Layout::from_size_align(1, 32).unwrap();
        let a2 = allocator.alloc(l2).unwrap();
        println!("Alloc 2: {:?}", allocator);

        allocator.dealloc(a1, l1);
        println!("Dealloc 1: {:?}", allocator);

        allocator.dealloc(a2, l2);
        println!("Dealloc 2: {:?}", allocator);
    }
}
