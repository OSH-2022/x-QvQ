use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields,
    registers::InMemoryRegister,
};

// assume that kernel is smaller than 2M, aligned to 4K
pub static mut BOOT_PT: BootPageTable = BootPageTable::new();

pub enum MemoryType {
    Device,
    Normal,
}

#[repr(C)]
#[repr(align(4096))]
pub struct BootPageTable {
    lvl1: [u64; 512],
    lvl2: [u64; 512],
    lvl3: [u64; 512],
}

impl BootPageTable {
    pub const PAGE_SIZE: u64 = 0x1000;

    const fn new() -> Self {
        BootPageTable {
            lvl1: [0; 512],
            lvl2: [0; 512],
            lvl3: [0; 512],
        }
    }

    pub unsafe fn get_lvl1_addr(&self) -> u64 {
        &self.lvl1 as *const _ as u64
    }

    pub unsafe fn map_kernel(&mut self, kernel_va: u64, kernel_pa: u64, kernel_size: u64) {
        self.lvl1[Self::get_lvl1_offset(kernel_va)] =
            Self::table_descriptor(&self.lvl2 as *const _ as u64);
        self.lvl2[Self::get_lvl2_offset(kernel_va)] =
            Self::table_descriptor(&self.lvl3 as *const _ as u64);

        for off in (0..kernel_size).step_by(Self::PAGE_SIZE as usize) {
            self.lvl3[Self::get_lvl3_offset(kernel_va + off)] =
                Self::page_descriptor(kernel_pa + off, MemoryType::Normal);
        }
    }

    pub unsafe fn map_page(&mut self, va: u64, pa: u64, mem_type: MemoryType) {
        self.lvl3[Self::get_lvl3_offset(va)] = Self::page_descriptor(pa, mem_type);
    }

    fn table_descriptor(next_level_pa: u64) -> u64 {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);
        val.write(
            STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_4KiB.val(next_level_pa >> 12)
                + STAGE1_TABLE_DESCRIPTOR::TYPE::Table
                + STAGE1_TABLE_DESCRIPTOR::VALID::True,
        );
        val.get()
    }

    fn page_descriptor(pa: u64, mem_type: MemoryType) -> u64 {
        let val = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(0);
        val.write(
            STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_4KiB.val(pa >> 12)
                + STAGE1_PAGE_DESCRIPTOR::AF::True
                + STAGE1_PAGE_DESCRIPTOR::TYPE::Page
                + STAGE1_PAGE_DESCRIPTOR::VALID::True
                + STAGE1_PAGE_DESCRIPTOR::SH::InnerShareable
                + STAGE1_PAGE_DESCRIPTOR::AP::RW_Elvl1
                + STAGE1_PAGE_DESCRIPTOR::PXN::False
                + STAGE1_PAGE_DESCRIPTOR::UXN::True,
        );
        match mem_type {
            MemoryType::Device => val.modify(STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(0)),
            MemoryType::Normal => val.modify(STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(1)),
        }
        val.get()
    }

    fn get_lvl1_offset(va: u64) -> usize {
        ((va >> (12 + 9 * 2)) & 0x1ff) as usize
    }

    fn get_lvl2_offset(va: u64) -> usize {
        ((va >> (12 + 9 * 1)) & 0x1ff) as usize
    }

    fn get_lvl3_offset(va: u64) -> usize {
        ((va >> (12 + 9 * 0)) & 0x1ff) as usize
    }
}

// A table descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-15.
register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        /// Physical address of the next descriptor.
        NEXT_LEVEL_TABLE_ADDR_4KiB OFFSET(12) NUMBITS(36) [], // [47:12]

        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

// A level 3 page descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-17.
register_bitfields! {u64,
    STAGE1_PAGE_DESCRIPTOR [
        /// Unprivileged execute-never.
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never.
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Physical address of the next table descriptor (lvl2) or the page descriptor (lvl3).
        OUTPUT_ADDR_4KiB OFFSET(12) NUMBITS(36) [], // [47:12]

        /// Access flag.
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareability field.
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions.
        AP       OFFSET(6) NUMBITS(2) [
            RW_Elvl1 = 0b00,
            RW_Elvl1_EL0 = 0b01,
            RO_Elvl1 = 0b10,
            RO_Elvl1_EL0 = 0b11
        ],

        /// Memory attributes index into the MAIR_Elvl1 register.
        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}
