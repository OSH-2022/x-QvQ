use crate::mmu::{VirtAddr, Addr};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Context {
    /// General-purpose registers (R0..R30).
    pub r: [u64; 31],
    /// User Stack Pointer (SP_EL0).
    pub usp: u64,
    /// Exception Link Register (ELR_EL1).
    pub elr: u64,
    /// Saved Process Status Register (SPSR_EL1).
    pub spsr: u64,
}

impl Context {
    pub unsafe fn to_sp(&self) -> VirtAddr {
        VirtAddr::from_usize(self as *const Context as _)
    }
}
