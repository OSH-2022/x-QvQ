ENTRY(_start)

PAGE_SIZE = 4K;

__boot_start_paddr = 0x80000;
__kernel_start_vaddr = 0xffffffffc0000000;

SECTIONS {
    . = __boot_start_paddr;
    .text.boot : AT(__boot_start_paddr){
        KEEP(*boot*(.text.boot*))
        *boot*(.text*)
    }

    .rodata.boot : AT(.) {
        *boot*(.rodata*)
    }

    .data.boot : AT(.) {
        *boot*(.data*)
    }

    .bss.boot : AT(.) {
        *boot*(.bss*)
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
        *(.bss*)
        . = ALIGN(16);
        __bss_end_paddr = . - __vaddr_offset;
    }

    __kernel_end_paddr = . - __vaddr_offset;

    /* guard page */
    . = ALIGN(PAGE_SIZE);
    . += PAGE_SIZE;

    .stack (NOLOAD) : {
        . += PAGE_SIZE;
    }
    __stack_end_paddr = . - __vaddr_offset;

    .pt (NOLOAD) : {
        __pt_start_paddr = . - __vaddr_offset;
        . += PAGE_SIZE * (1 + 512);
    }

    __paddr_start = . - __vaddr_offset;

    .mmio (NOLOAD) : {
        __aux_start_vaddr = .;
        . += PAGE_SIZE;
    }
    __vaddr_start = .;
}