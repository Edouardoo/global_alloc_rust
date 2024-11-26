#[cfg(feature = "global_alloc")]
extern crate alloc;

#[cfg(feature = "global_alloc")]
use core::alloc::{GlobalAlloc, Layout};





#[cfg(feature = "global_alloc")]

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

impl BumpAllocator {
    pub const unsafe fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: heap_start,
        }
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.next, layout.align());
        let alloc_end = alloc_start + layout.size();


        if alloc_end > self.heap_end {
            core::ptr::null_mut()
        } else {
            self.next = alloc_end;
            alloc_start as *mut u8
        }
    }
}


const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

