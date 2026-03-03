.section .text
.global reset_trap

reset_trap:
    li sp, 0x80200000
    call main

sleep:
    wfi
    j sleep
