mod pagetable;

use crate::mmu::{Addr, MemoryType, PhyAddr, VirtAddr};
use pagetable::PageTable;
use spin::Mutex;

pub const PAGE_SIZE: usize = 0x1000;

pub static PHY_PAGE_ALLOC: Mutex<PhyPageAlloc> = Mutex::new(PhyPageAlloc::empty());
pub static VIRT_PAGE_MANAGE: Mutex<VirtPageManage> = Mutex::new(VirtPageManage::empty());

/* alloc but never free (for simplicity) */
pub struct PhyPageAlloc {
    current: Option<PhyAddr>,
}

impl PhyPageAlloc {
    const fn empty() -> Self {
        Self { current: None }
    }

    pub fn init(&mut self, pa_start: PhyAddr) {
        self.current = Some(pa_start);
    }

    pub fn alloc(&mut self) -> PhyAddr {
        let old = self.current.expect("phy page alloc uninitialized");
        self.current = Some(old.add_off(PAGE_SIZE));
        old
    }
}

pub struct VirtPageManage {
    pt: Option<&'static mut PageTable>,
    va_start: Option<VirtAddr>,
}

impl VirtPageManage {
    const fn empty() -> Self {
        Self {
            pt: None,
            va_start: None,
        }
    }

    pub fn init(&mut self, pte_va: VirtAddr, va_start: VirtAddr) {
        unsafe {
            self.pt = Some(&mut *(pte_va.to_usize() as *mut PageTable));
        }
        self.va_start = Some(va_start);
    }

    pub fn map(&mut self, va: VirtAddr, pa: PhyAddr, mem_type: MemoryType) {
        assert!(
            va >= self.va_start.expect("virt page manage uninitialized"),
            "trying to remap kernel text"
        );
        let pt = self.pt.take().expect("virt page manage uninitialized");
        pt.map(va, pa, mem_type);
        self.pt = Some(pt);
    }

    pub fn unmap(&mut self, va: VirtAddr) -> PhyAddr {
        let pt = self.pt.take().expect("virt page manage uninitialized");
        let pa = pt.unmap(va);
        self.pt = Some(pt);
        pa
    }
}
