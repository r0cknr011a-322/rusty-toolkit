.section .text
.global reset_trap

reset_trap:
    li sp, 0x80080000
    call main
    ret
