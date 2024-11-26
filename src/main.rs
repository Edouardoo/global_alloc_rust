#[cfg(feature = "global_alloc")]
extern crate alloc;

#[cfg(feature = "global_alloc")]
use core::alloc::{GlobalAlloc, Layout};

#[link_section = ".heap"] 



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


unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocator = &mut *(self as *const _ as *mut BumpAllocator);
        allocator.alloc(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}

const HEAP_SIZE: usize = 1024 * 1024; 

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];


#[global_allocator]
static GLOBAL_ALLOCATOR: BumpAllocator = unsafe {
    BumpAllocator::new(HEAP.as_ptr() as usize, HEAP_SIZE)
};



pub fn allocate_example() {
    // Import necessary types from the `alloc` crate.
    use alloc::vec::Vec;
    use alloc::boxed::Box;
    use alloc::string::String;


    let mut numbers = Vec::new(); // Creates a new empty Vec

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    let boxed_value = Box::new(42);
    let greeting = String::from("Hello, world!");
    let boxed_array = Box::new([10, 20, 30, 40, 50]);

    #[derive(Debug)]
    struct Node {
        value: u32,
        next: Option<Box<Node>>,
    }

    let node3 = Box::new(Node { value: 3, next: None });
    let node2 = Box::new(Node { value: 2, next: Some(node3) });
    let node1 = Box::new(Node { value: 1, next: Some(node2) });

}