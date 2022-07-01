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
use mmu::{Addr, MemoryType, VirtAddr, PhyAddr};
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

    print!("== kernel init ==\n").unwrap();

    /* init mapping */
    {
        let mut va = VirtAddr::from_usize(va_start);

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

        let heap_ptr = NonNull::new(va.to_usize() as *mut u8).expect("invalid heap vaddr");
        unsafe {
            heap::ALLOCATOR.init(heap_ptr, HEAP_SIZE * arch::PAGE_SIZE);
        }
        print!("{}", String::from("heap init\n")).unwrap();

        va = va.add_off(arch::PAGE_SIZE * HEAP_SIZE);

        /* core timer */
        virt.map(va, PhyAddr::from_usize(bsp::memory_map::perip::core_timer::BASE), MemoryType::Device);
        {
            let mut core_timer = bsp::CORE_TIMER.lock();
            core_timer.init(va);
        }
        print!("core_timer init at {:#x}\n", va.to_usize()).unwrap();

        va = va.add_off(arch::PAGE_SIZE);
    }
    trap::init();
    print!("==trap available==\n").unwrap();
    loop {}
}
