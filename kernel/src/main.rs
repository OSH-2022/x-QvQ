#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]
#![feature(const_mut_refs)]

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
mod mmu;

extern crate alloc;

use bsp::Driver;
use mmu::{Addr, MemoryType, VirtAddr};
use core::ptr::NonNull;
use palloc::GlobalPalloc;
use alloc::string::String;

const HEAP_SIZE: usize = 10;

#[no_mangle]
extern "C" fn _start_kernel(
    aux_va: usize,
    pte_va: usize,
    va_start: usize,
    pa_start: usize,
) {
    bsp::MINI_UART.init(aux_va);
    let va = VirtAddr::from_usize(va_start);
    {
        let mut phy = arch::PHY_PAGE_ALLOC.lock();
        phy.init(mmu::PhyAddr::from_usize(pa_start));

        let mut virt = arch::VIRT_PAGE_MANAGE.lock();
        virt.init(mmu::VirtAddr::from_usize(pte_va), mmu::VirtAddr::from_usize(va_start));

        print!("pa_start: {:#x}\nva_start: {:#x}\n", pa_start, va_start).unwrap();
        /* heap */
        for i in 0..HEAP_SIZE {
            virt.map(va.add_off(i * arch::PAGE_SIZE), phy.alloc(), MemoryType::Normal)
        }
        print!("heap_size: {}\n", HEAP_SIZE).unwrap();
    }
    let heap_ptr = NonNull::new(va.to_usize() as *mut u8).expect("invalid heap vaddr");
    unsafe {
        heap::ALLOCATOR.init(heap_ptr, HEAP_SIZE * arch::PAGE_SIZE);
    }
    print!("{}", String::from("heap init\n")).unwrap();
    
    trap::init();
    print!("==trap available==\n").unwrap();
    timer::init();
    print!("==timer available==\n").unwrap();
    loop {}
}
