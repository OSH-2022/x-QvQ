#![no_std]
#![no_main]
#![feature(lang_items)]

mod arch;

use core::panic::PanicInfo;

#[no_mangle]
fn _start_rust() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
