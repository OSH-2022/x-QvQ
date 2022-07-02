#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::boxed::Box;
use interface::Syscall;

#[no_mangle]
fn entry(sys: Box<dyn Syscall + Send + Sync>) {
    libsyscall::SYS.call_once(|| sys);
    libsyscall::pheap_init();
    main();
}

/* only safe rust in main */
fn main() {}
