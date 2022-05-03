ENTRY(_start)

__rpi_phys_dram_start_addr = 0;

__rpi_phys_binary_load_addr = 0x80000;

SECTIONS {
    . = __rpi_phys_dram_start_addr;

    .boot_stack (NOLOAD) : {
        . += __rpi_phys_binary_load_addr;
    }
    
    __boot_stack_end = .;
    
    __kernel_start = .;

    .text : {
        *(.text._start)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss*);
        __bss_end = .;
    }

    __kernel_end = .;
}