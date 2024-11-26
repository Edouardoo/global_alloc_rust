#![no_std]
#![cfg(feature = "global_alloc")]

extern crate my_allocator;
extern crate alloc;

use my_allocator::allocate_example;

#[no_mangle]
pub extern "C" fn main() -> ! {
    allocate_example();

    loop {}
}
