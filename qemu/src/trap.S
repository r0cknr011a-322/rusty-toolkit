.section .text
.global reset_trap

reset_trap:
    li sp, 0x84000000
    call main
    ret
