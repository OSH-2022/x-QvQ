#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod arch;
mod bsp;
mod panic;
mod print;
mod trap;
mod syscall;
mod timer;
mod config;
mod gicv2;

#[no_mangle]
fn _start_kernel() {
    bsp::driver_init();
    print!("== Kernel Init ==\n").unwrap();
    trap::init();
    print!("==trap available==\n").unwrap();
    timer::init();
    print!("==timer available==\n").unwrap();
    loop {}
}
