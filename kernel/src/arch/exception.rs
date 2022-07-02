mod context;

use core::arch::asm;
use core::ops::{Deref, DerefMut};

use crate::mmu::{Addr, VirtAddr};
use crate::thread::Scheduler;
pub use context::Context;
use cortex_a::asm::barrier;
use cortex_a::registers::VBAR_EL1;
use tock_registers::interfaces::Writeable;

core::arch::global_asm!(include_str!("exception/exception.s"));

extern "C" {
    static exception_handlers: u8;
    fn exit_exception();
}

pub struct Exception {}

impl Exception {
    pub fn setup_vbar() {
        unsafe {
            VBAR_EL1.set(&exception_handlers as *const _ as _);
            barrier::isb(barrier::SY);
        }
    }

    pub fn mask_irq() {
        unsafe {
            core::arch::asm!("msr daifset, #2");
        }
    }

    pub fn unmask_irq() {
        unsafe {
            core::arch::asm!("msr daifclr, #2");
        }
    }

    pub fn set_sp_and_exit(sp: VirtAddr) {
        unsafe {
            asm!(
                "mov sp, {sp}",
                sp = in(reg) sp.to_usize(),
            );
            exit_exception();
        }
    }
}

#[no_mangle]
extern "C" fn handle_exception(context: &mut Context) {
    panic!("exception\n{:?}", context);
}

#[no_mangle]
extern "C" fn handle_exception_serror(context: &mut Context) {
    panic!("exception serror\n{:?}", context);
}

#[no_mangle]
extern "C" fn handle_interrupt(context: &'static Context) {
    // irq masked automatically
    crate::bsp::CoreTimer::set_interval(10);
    if let Some(mut sche) = crate::thread::SCHEDULER.try_lock() {
        let sche_ptr = sche.deref_mut() as *mut Scheduler;
        /* have to drop the lock here in case of dead lock, should be safe for single core cpu (hope so) */
        drop(sche);
        unsafe {
            (*sche_ptr).schedule(context);
        }
    }
    unsafe {
        Exception::set_sp_and_exit(context.to_sp());
    }
}
