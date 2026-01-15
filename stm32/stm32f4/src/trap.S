.section .text
.global reset_trap

reset_trap:
    ldr r0, =0x20010000
    msr msp, r0
    msr psp, r0
    bl main
