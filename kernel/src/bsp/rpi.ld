ENTRY(_start)

PAGE_SIZE = 4K;

__boot_start_paddr = 0x80000;
__kernel_start_vaddr = 0xffffff8000000000;

SECTIONS {
    . = __boot_start_paddr;
    .boot : AT(__boot_start_paddr) {
        *boot*(.text._start)
        *boot*(.text* .rodata* .data* .bss*)
    }

    __kernel_start_paddr = ALIGN(PAGE_SIZE);
    __vaddr_offset = __kernel_start_vaddr - __kernel_start_paddr;

    . = __kernel_start_vaddr;
    
    .text : AT(__kernel_start_paddr) {
        KEEP(*(.text._start_kernel))
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    . = ALIGN(16);
    __bss_start_paddr = . - __vaddr_offset;

    .bss (NOLOAD) : {
        *(.bss*);
        . = ALIGN(16);
        __bss_end_paddr = . - __vaddr_offset;
    }

    __kernel_end_paddr = . - __vaddr_offset;

    /* guard page */
    . = ALIGN(PAGE_SIZE);
    . += PAGE_SIZE;

    . = ALIGN(PAGE_SIZE);
    .stack (NOLOAD) : {
        . += PAGE_SIZE;
    }
    __stack_end_paddr = . - __vaddr_offset;

    . = ALIGN(PAGE_SIZE);
    .mmio (NOLOAD) : {
        __aux_start_paddr = . - __vaddr_offset;
        . += PAGE_SIZE;
    }
}