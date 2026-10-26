    .section .text.entry, "ax", @progbits
    .globl   _start
    .type    _start, @function

_start:
    /*       a0 = hartid, a1 = DTB physical address */

    /*
    *        Clear BSS first. The boot stack lives in BSS, so we must not
    *        use the stack before this loop finishes.
    */
    la       t0, __bss_start
    la       t1, __bss_end
1:
    bgeu     t0, t1, 2f
    sd       zero, 0(t0)
    addi     t0, t0, 8
    j        1b

2:
    /*
    *        Set gp for the RISC-V ABI. norelax prevents the linker from
    *        rewriting this address calculation before gp is usable.
    */
    .option  push
    .option  norelax
    la       gp, __global_pointer$
    .option  pop

    /*       Install the boot stack. The stack grows downward. */
    la       sp, boot_stack_top

    mv       tp, a0

    /*       keep S-mode interrupts disabled. */
    csrci    sstatus, 2
    csrw     sie, zero

    call     rust_main

3:
    wfi
    j        3b


    .size    _start, .-_start

    /*
    *        Boot stack. It lives in BSS so the BSS clear covers it.
    *        .align 4 means 2^4 = 16-byte alignment on RISC-V.
    */
    .section .bss.stack, "aw", @nobits
    .align   4

boot_stack:
    .space   16384                                                        # 16 KiB for stack

boot_stack_top:
