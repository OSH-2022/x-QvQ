.section .text.boot

.global _start

// https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
    adrp \register, \symbol
    add \register, \register, #:lo12:\symbol
.endm

_start:
    mrs	x1, mpidr_el1
    and	x1, x1, #3
    mov	x2, #0
    cmp	x1, x2
    bne	.L_parking_loop

    ADR_REL x1, __bss_start_paddr
    ADR_REL x2, __bss_end_paddr
.L_zero_bss:
    cmp x1, x2
    beq .L_setup_stack
    stp xzr, xzr, [x1], #16
    b .L_zero_bss

.L_setup_stack:
    ADR_REL x0, __stack_end_paddr
    mov	sp, x0
    ADR_REL x0, __kernel_start_paddr
    br x0

.L_parking_loop:
    wfe
    b .L_parking_loop