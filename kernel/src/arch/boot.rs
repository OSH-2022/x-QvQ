core::arch::global_asm! {r##"

.section .text._start

.global _start

_start:
    mrs	x1, mpidr_el1
    and	x1, x1, #3
    mov	x2, #0
    cmp	x1, x2
    bne	parking_loop

    adr x0, __boot_stack_end
    mov	sp, x0
    b _start_rust

parking_loop:
    wfe
    b parking_loop

"##}
