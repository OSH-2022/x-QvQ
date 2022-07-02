#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::fmt::Arguments;
use core::panic::PanicInfo;
use core::ptr::NonNull;
use interface::Syscall;
use palloc::{GlobalPalloc, SpinPalloc};
use spin::Once;

pub static SYS: Once<Box<dyn Syscall + Send + Sync>> = Once::new();

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[global_allocator]
static mut PHEAP: SpinPalloc = SpinPalloc::empty();

pub fn pheap_init() {
    let heap_ptr = NonNull::new(sys_alloc_page()).unwrap();
    unsafe {
        PHEAP.init(heap_ptr, 4096);
    }
}

/* should never panic */
pub fn sys_create_thread(func: fn()) {
    unsafe {
        SYS.get().unwrap().create_thread(func);
    }
}

pub fn sys_exit() {
    unsafe {
        SYS.get().unwrap().exit();
    }
}

pub fn sys_print(args: Arguments) {
    unsafe {
        SYS.get().unwrap().print(args);
    }
}

pub fn sys_alloc_page() -> *mut u8 {
    unsafe { SYS.get().unwrap().alloc_page() }
}
