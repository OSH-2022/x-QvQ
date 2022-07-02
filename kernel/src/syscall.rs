use core::fmt::{Arguments, Result};
use core::alloc::Layout;

use crate::mmu::Addr;

struct Syscall {}

unsafe impl interface::Syscall for Syscall {
    unsafe fn create_thread(&self, func: fn()) {
        let mut sche = crate::thread::SCHEDULER.lock();
        sche.insert(crate::thread::Thread::new(crate::mmu::VirtAddr::from_usize(
            func as *const usize as usize,
        )));
    }

    unsafe fn exit(&self) {
        let mut sche = crate::thread::SCHEDULER.lock();
        sche.remove_self();
    }

    unsafe fn print(&self, args: Arguments) {
        crate::print::_print(args).unwrap();
    }

    unsafe fn alloc_page(&self) -> *mut u8 {
        let mut virt = crate::arch::VIRT_PAGE_MANAGE.lock();
        virt.new_page(crate::mmu::MemoryType::Normal).to_usize() as *mut u8
    }

    // shared heap
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}
