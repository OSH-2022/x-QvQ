#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]
#![feature(const_mut_refs)]

mod arch;
mod bsp;
mod heap;
mod mmu;
mod panic;
mod print;
mod thread;

extern crate alloc;

use alloc::string::String;
use bsp::Driver;
use core::ptr::NonNull;
use mmu::{Addr, MemoryType, PhyAddr, VirtAddr};
use palloc::GlobalPalloc;

const HEAP_SIZE: usize = 10;

#[no_mangle]
extern "C" fn _start_kernel(aux_va: usize, pte_va: usize, va_start: usize, pa_start: usize) {
    bsp::MINI_UART.init(aux_va);

    print!("== kernel init ==\n").unwrap();

    /* init mapping */
    {
        let mut phy = arch::PHY_PAGE_ALLOC.lock();
        phy.init(mmu::PhyAddr::from_usize(pa_start));

        let mut virt_alloc = arch::VIRT_PAGE_ALLOC.lock();
        virt_alloc.init(mmu::VirtAddr::from_usize(va_start));

        let mut virt = arch::VIRT_PAGE_MANAGE.lock();
        virt.init(
            mmu::VirtAddr::from_usize(pte_va),
            mmu::VirtAddr::from_usize(va_start),
        );

        print!("pa_start: {:#x}\nva_start: {:#x}\n", pa_start, va_start).unwrap();

        /* heap */
        let heap_start = virt_alloc.alloc();
        let heap_ptr = NonNull::new(heap_start.to_usize() as *mut u8).expect("invalid heap vaddr");
        virt.map(heap_start, phy.alloc(), MemoryType::Normal);
        for i in 0..HEAP_SIZE - 1 {
            virt.map(virt_alloc.alloc(), phy.alloc(), MemoryType::Normal)
        }
        print!("heap_size: {}\n", HEAP_SIZE).unwrap();

        unsafe {
            heap::ALLOCATOR.init(heap_ptr, HEAP_SIZE * arch::PAGE_SIZE);
        }
        print!("{}", String::from("heap init\n")).unwrap();

        /* core timer */
        let core_timer_start = virt_alloc.alloc();
        virt.map(
            core_timer_start,
            PhyAddr::from_usize(bsp::memory_map::perip::core_timer::BASE),
            MemoryType::Device,
        );
        {
            let mut core_timer = bsp::CORE_TIMER.lock();
            core_timer.init(core_timer_start);
        }
        bsp::CoreTimer::set_interval(10);
        print!("core_timer init at {:#x}\n", core_timer_start.to_usize()).unwrap();
    }

    /* exception */
    arch::Exception::mask_irq();
    arch::Exception::setup_vbar();
    print!("exception init\n").unwrap();

    /* thread */
    {
        let mut sche = thread::SCHEDULER.lock();
        sche.init();

        sche.insert(thread::Thread::new(VirtAddr::from_usize(
            kernel_thread as *const usize as usize,
        )));
        sche.insert(thread::Thread::new(VirtAddr::from_usize(
            hello as *const usize as usize,
        )));
    }

    /* start schedule */
    arch::Exception::unmask_irq();

    loop {}
}

fn kernel_thread() {
    print!("kernel thread init\n").unwrap();
    loop {}
}

fn hello() {
    print!("hello thread\n").unwrap();
    {
        let mut sche = thread::SCHEDULER.lock();
        sche.remove_self();
    }
    loop {}
}
