#![no_std]

mod mmu;

use core::arch::asm;

use cortex_a::{
    asm::{barrier, eret},
    registers::*,
};
use mmu::{BootPageTable, MemoryType::*, BOOT_PT};
use tock_registers::interfaces::Writeable;

core::arch::global_asm! {include_str!("boot.s")}

// https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/16_virtual_mem_part4_higher_half_kernel/kernel/src/_arch/aarch64/cpu/boot.rs
unsafe fn prepare_el2_to_el1_transition(stack_va: u64, kernel_start_va: u64) {
    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    ELR_EL2.set(kernel_start_va);
    SP_EL1.set(stack_va);
}

// https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/16_virtual_mem_part4_higher_half_kernel/kernel/src/_arch/aarch64/memory/mmu.rs
fn set_up_mair() {
    // Define the memory types being mapped.
    MAIR_EL1.write(
        // Attribute 1 - Cacheable normal DRAM.
        MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc +
        MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc +

        // Attribute 0 - Device.
        MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck,
    );
}

fn configure_translation_control() {
    let t1sz = 64 - 39;

    TCR_EL1.write(
        TCR_EL1::TBI1::Used
            + TCR_EL1::IPS::Bits_32
            + TCR_EL1::TG1::KiB_4
            + TCR_EL1::SH1::Inner
            + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::EPD1::EnableTTBR1Walks
            + TCR_EL1::A1::TTBR1
            + TCR_EL1::T1SZ.val(t1sz)
            + TCR_EL1::EPD0::DisableTTBR0Walks,
    );
}

unsafe fn enable_mmu_and_caching(boot_pt: &BootPageTable) {
    // Prepare the memory attribute indirection register.
    set_up_mair();

    // Set the "Translation Table Base Register".
    TTBR1_EL1.set_baddr(boot_pt.get_lvl1_addr());

    configure_translation_control();

    // Switch the MMU on.
    //
    // First, force all previous changes to be seen before the MMU is enabled.
    barrier::isb(barrier::SY);

    // Enable the MMU and turn on data and instruction caching.
    SCTLR_EL1.write(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

    // Force MMU init to complete before next instruction.
    barrier::isb(barrier::SY);
}

unsafe fn enable_fp() {
    asm! {r"
        mov x1, #(0x3 << 20)
        msr cpacr_el1, x1
        isb
    "};
}

const AUX_BASE: u64 = 0x3f21_5000;

#[no_mangle]
unsafe extern "C" fn _start_rust(
    kernel_pa: u64,
    kernel_size: u64,
    stack_pa: u64,
    aux_pa: u64,
    heap_pa: u64,
    heap_size: u64,
    va_offset: u64,
) {
    BOOT_PT.map_kernel(kernel_pa + va_offset, kernel_pa, kernel_size);
    BOOT_PT.map_page(
        stack_pa + va_offset - BootPageTable::PAGE_SIZE,
        stack_pa - BootPageTable::PAGE_SIZE,
        Normal,
    );
    BOOT_PT.map_page(aux_pa + va_offset, AUX_BASE, Device);
    let mut pa = heap_pa;
    while pa < heap_pa + heap_size {
        BOOT_PT.map_page(pa + va_offset, pa, Normal);
        pa += BootPageTable::PAGE_SIZE;
    }
    enable_mmu_and_caching(&BOOT_PT);
    prepare_el2_to_el1_transition(stack_pa + va_offset, kernel_pa + va_offset);
    enable_fp();
    let aux_va = aux_pa + va_offset;
    let heap_va = heap_pa + va_offset;
    asm! {
        "mov x0, {aux_va}",
        "mov x1, {heap_va}",
        "mov x2, {heap_size}",
        aux_va = in(reg) aux_va,
        heap_va = in(reg) heap_va,
        heap_size = in(reg) heap_size,
    }
    eret();
}
