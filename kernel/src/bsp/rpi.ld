ENTRY(_start)

PAGE_SIZE = 4K;

__boot_start_paddr = 0x80000;
__kernel_start_vaddr = 0xff00000000000000;

SECTIONS {
    . = __boot_start_paddr;
    .text.boot : AT(__boot_start_paddr) {
        *(.text.boot)
    }

    __kernel_start_paddr = ALIGN(PAGE_SIZE);
    __kernel_vaddr_offset = __kernel_start_vaddr - __kernel_start_paddr;

    . = __kernel_start_vaddr;
    
    .text : AT(__kernel_start_paddr) {
        *(.text._start_rust)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    . = ALIGN(16);
    __bss_start_paddr = . - __kernel_vaddr_offset;

    .bss (NOLOAD) : {
        *(.bss*);
        . = ALIGN(16);
        __bss_end_paddr = . - __kernel_vaddr_offset;
    }

     __kernel_end_vaddr = .;

     /* guard page */
     . = ALIGN(PAGE_SIZE);
     . += PAGE_SIZE;

    . = ALIGN(PAGE_SIZE);
    .stack (NOLOAD) : {
        . += PAGE_SIZE;
    }
    __stack_end_paddr = . - __kernel_vaddr_offset;
}