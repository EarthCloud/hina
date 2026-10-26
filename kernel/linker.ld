ENTRY(_start)

SECTIONS
{
    /*
     * In QEMU virt, OpenSBI Firmware start at 0x80000000,
     * the very first 2MiB is set up for the firmware.
     * By the time OpenSBI Start, it jumps to here.
     */
    . = 0x80200000;

    .text : {
        /* 
         * Forced to keep .text.entry, 
         * despite section garbage collection is available.
         */
        KEEP(*(.text.entry))
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        . = ALIGN(8);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    }

    /* RISC-V Global Pointer */
    __global_pointer$ = . + 0x800;
}