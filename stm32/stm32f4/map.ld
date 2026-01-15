ENTRY(reset_trap)

MEMORY {
    ram (rw) : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS {
    .text : {
        *(.text*)
    } > ram

    .rodata : {
        *(.rodata*)
    } > ram
}
