.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0x02
    lsls r0, #0x0F
    lsls r0, #0x0F
    ands r0, r0
    svc #0xFF

