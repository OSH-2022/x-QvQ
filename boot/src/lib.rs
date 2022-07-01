#![no_std]

mod mmu;

use core::arch::asm;

use cortex_a::{
    asm::{barrier, eret},
    registers::*,
};
use mmu::{BootPageTable, MemoryType};
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
    /* 2-level page table, mapping 1GB in total */
    let t1sz = 64 - 30;

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

unsafe fn enable_mmu_and_caching(base: u64) {
    // Prepare the memory attribute indirection register.
    set_up_mair();

    // Set the "Translation Table Base Register".
    TTBR1_EL1.set_baddr(base);

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
    off: u64,
    pt_pa: u64,

    aux_va: u64,
    va_start: u64,
    pa_start: u64,
) {
    let boot_pt = &mut *(pt_pa as *mut BootPageTable);
    boot_pt.init();

    /* text bss data */
    boot_pt.map_pages(kernel_pa + off, kernel_pa, kernel_size, MemoryType::Normal);

    /* stack */
    let stack_pa_start = stack_pa - BootPageTable::PAGE_SIZE;
    boot_pt.map_page(stack_pa_start + off, stack_pa_start, MemoryType::Normal);

    /* mmio */
    boot_pt.map_page(aux_va, AUX_BASE, MemoryType::Device);

    /* pte */
    let pte_pa = pt_pa + BootPageTable::PAGE_SIZE;
    boot_pt.map_pages(
        pte_pa + off,
        pte_pa,
        BootPageTable::PAGE_SIZE * 512,
        MemoryType::Normal,
    );

    enable_mmu_and_caching(boot_pt.get_base_addr());
    prepare_el2_to_el1_transition(stack_pa + off, kernel_pa + off);
    enable_fp();
    asm! {
        "mov x0, {aux_va}",
        "mov x1, {pte_va}",
        "mov x2, {va_start}",
        "mov x3, {pa_start}",
        aux_va = in(reg) aux_va,
        pte_va = in(reg) pte_pa + off,
        va_start = in(reg) va_start,
        pa_start = in(reg) pa_start,
    }
    eret();
}
