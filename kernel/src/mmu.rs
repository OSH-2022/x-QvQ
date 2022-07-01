pub enum MemoryType {
    Device,
    Normal,
}

pub trait Addr {
    fn to_usize(&self) -> usize;
    fn from_usize(u: usize) -> Self;

    fn add_off(&self, off: usize) -> Self
    where
        Self: Sized,
    {
        Self::from_usize(self.to_usize() + off)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct PhyAddr {
    pa: usize,
}

impl Addr for PhyAddr {
    fn to_usize(&self) -> usize {
        self.pa
    }

    fn from_usize(u: usize) -> Self {
        Self { pa: u }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct VirtAddr {
    va: usize,
}

impl Addr for VirtAddr {
    fn to_usize(&self) -> usize {
        self.va
    }

    fn from_usize(u: usize) -> Self {
        Self { va: u }
    }
}
