use crate::mmu::{Addr, PhyAddr, VirtAddr, MemoryType};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields,
    registers::InMemoryRegister,
};

#[repr(C)]
#[repr(align(4096))]
#[allow(dead_code)]
pub struct PageTable {
    pte: [[u64; 512]; 512],
}

impl PageTable {
    pub fn map(&mut self, va: VirtAddr, pa: PhyAddr, mem_type: MemoryType) {
        self.pte[Self::get_pmd_offset(va)][Self::get_pte_offset(va)] = Self::page_descriptor(pa, mem_type);
    }

    pub fn unmap(&mut self, va: VirtAddr) -> PhyAddr {
        let pte = self.pte[Self::get_pmd_offset(va)][Self::get_pte_offset(va)];
        let reg = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(pte);
        reg.modify(STAGE1_PAGE_DESCRIPTOR::VALID::False);
        let pa = PhyAddr::from_usize(
            (reg.read(STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_4KiB) as usize) << 12,
        );
        self.pte[Self::get_pmd_offset(va)][Self::get_pte_offset(va)] = reg.get();
        pa
    }

    fn page_descriptor(pa: PhyAddr, mem_type: MemoryType) -> u64 {
        let val = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(0);
        val.write(
            STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_4KiB.val(pa.to_usize() as u64 >> 12)
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

    fn get_pmd_offset(va: VirtAddr) -> usize {
        (va.to_usize() >> (12 + 9 * 1)) & 0x1ff
    }

    fn get_pte_offset(va: VirtAddr) -> usize {
        (va.to_usize() >> (12 + 9 * 0)) & 0x1ff
    }
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
