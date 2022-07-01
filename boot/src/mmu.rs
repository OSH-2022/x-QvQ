#[derive(Copy, Clone)]
pub enum MemoryType {
    Device,
    Normal,
}

#[repr(C)]
#[repr(align(4096))]
pub struct BootPageTable {
    lvl2: [u64; 512],
    lvl3: [[u64; 512]; 512],
}

impl BootPageTable {
    pub const PAGE_SIZE: u64 = 0x1000;

    pub unsafe fn get_base_addr(&self) -> u64 {
        &self.lvl2 as *const _ as u64
    }

    pub fn init(&mut self) {
        let mut i: usize = 0;
        while i < 512 {
            self.lvl2[i] = Self::table_descriptor(&self.lvl3[i] as *const _ as u64);
            let mut j: usize = 0;
            while j < 512 {
                self.lvl3[i][j] = 0;
                j += 1;
            }
            i += 1;
        }
    }

    pub unsafe fn map_pages(&mut self, va: u64, pa: u64, size: u64, mem_type: MemoryType) {
        let mut off: u64 = 0;
        while off < size {
            self.lvl3[Self::get_lvl2_offset(va + off)][Self::get_lvl3_offset(va + off)] = Self::page_descriptor(pa + off, mem_type);
            off += Self::PAGE_SIZE ;
        }
    }

    pub unsafe fn map_page(&mut self, va: u64, pa: u64, mem_type: MemoryType) {
        self.lvl3[Self::get_lvl2_offset(va)][Self::get_lvl3_offset(va)] = Self::page_descriptor(pa, mem_type);
    }

    fn table_descriptor(next_level_pa: u64) -> u64 {
        0x3 | (next_level_pa & 0x0000_ffff_ffff_f000)
    }

    fn page_descriptor(pa: u64, mem_type: MemoryType) -> u64 {
        match mem_type {
            MemoryType::Device => 0x0040_0000_0000_0000 | 0b1_11_00_0_000_1_1 | (pa & 0x0000_ffff_ffff_f000),
            MemoryType::Normal => 0x0040_0000_0000_0000 | 0b1_11_00_0_001_1_1 | (pa & 0x0000_ffff_ffff_f000)
        }
    }

    pub fn get_lvl2_offset(va: u64) -> usize {
        ((va >> (12 + 9 * 1)) & 0x1ff) as usize
    }

    pub fn get_lvl3_offset(va: u64) -> usize {
        ((va >> (12 + 9 * 0)) & 0x1ff) as usize
    }
}
