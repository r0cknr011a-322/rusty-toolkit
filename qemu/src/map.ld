ENTRY(reset_trap)

MEMORY {
    code (rw)   : ORIGIN = 0x80000000, LENGTH = 1M
    stack (rw)  : ORIGIN = 0x80200000, LENGTH = 1M
}

SECTIONS {
    .text : {
        *(.text*)
    } > code

    .rodata : {
        *(.rodata*)
    } > code
}
