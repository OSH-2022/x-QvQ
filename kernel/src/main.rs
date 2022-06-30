#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]

mod arch;
mod bsp;
mod config;
mod gicv2;
mod heap;
mod panic;
mod print;
mod syscall;
mod timer;
mod trap;

extern crate alloc;

use bsp::Driver;
use core::ptr::NonNull;
use palloc::GlobalPalloc;

#[no_mangle]
extern "C" fn _start_kernel(
    aux_va: usize,
    pt_va: usize,
    pa_start: usize,
) {
    bsp::MINI_UART.init(aux_va);
    // let heap_ptr = NonNull::new(heap_va as *mut u8).expect("invalid heap vaddr");
    // unsafe {
    //     heap::ALLOCATOR.init(heap_ptr, heap_size);
    // }
    print!("== Kernel Init ==\n").unwrap();
    trap::init();
    print!("==trap available==\n").unwrap();
    timer::init();
    print!("==timer available==\n").unwrap();
    loop {}
}
