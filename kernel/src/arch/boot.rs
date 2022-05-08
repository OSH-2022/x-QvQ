core::arch::global_asm! {r##"

.section .text._start

.global _start

_start:
    mrs	x1, mpidr_el1
    and	x1, x1, #3
    mov	x2, #0
    cmp	x1, x2
    bne	.L_parking_loop

    adr x1, __bss_start
    adr x2, __bss_end
.L_zero_bss:
    cmp x1, x2
    beq .L_setup_stack
    stp xzr, xzr, [x1], #16
    b .L_zero_bss

.L_setup_stack:
    adr x0, __boot_stack_end
    mov	sp, x0
    b _start_rust

.L_parking_loop:
    wfe
    b .L_parking_loop

"##}