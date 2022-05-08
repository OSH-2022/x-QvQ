#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod arch;
mod bsp;
mod panic;
mod print;

#[no_mangle]
fn _start_rust() {
    bsp::driver_init();
    print!("== Kernel Init ==\n").unwrap();
    loop {}
}
