#![no_std]
#![feature(auto_traits)]
#![feature(negative_impls)]

mod rref;

use core::fmt::Arguments;
use core::alloc::Layout;

pub unsafe trait Syscall {
    unsafe fn create_thread(&self, func: fn());
    unsafe fn exit(&self);
    unsafe fn print(&self, args: Arguments);
    unsafe fn alloc_page(&self) -> *mut u8;

    // shared heap
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
}
