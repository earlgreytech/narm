.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0xf
    movs r1, #0x1
    tst r0, r1
    svc #0xFF

