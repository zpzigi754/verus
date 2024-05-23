#![no_std]
#![no_main]
use core::alloc::*;

extern crate alib;
use alib::add;

fn main() {
    let res = add(2,5);
}

/// The global allocator type.
#[derive(Default)]
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
         unimplemented!();
     }
     unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
         unimplemented!();
     }
}

/// The static global allocator.
#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
