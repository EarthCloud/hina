    .section .text
    .globl   trap_entry
    .align   2

trap_entry:
    csrr     a0, scause
    csrr     a1, sepc
    csrr     a2, stval
    mv       a3,tp
    call     trap_panic
1:
    wfi
    j        1b