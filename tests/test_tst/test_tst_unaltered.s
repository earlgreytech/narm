.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0xFF
    movs r1, #0x0A
    tst r0, r1
    svc #0xFF

