ENTRY(entry)

__init_vaddr = 0xffffffffe0000000;

SECTIONS {
    . = __init_vaddr;
    .text : {
        KEEP(*(.text.entry*))
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(.bss*)
    }
}