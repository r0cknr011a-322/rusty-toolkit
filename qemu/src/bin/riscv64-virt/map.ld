ENTRY(reset_trap)

MEMORY {
    ram (rw) : ORIGIN = 0x80000000, LENGTH = 8M
}

SECTIONS {
    .text : {
        *(.text*)
    } > ram

    .rodata : {
        *(.rodata*)
    } > ram
}
