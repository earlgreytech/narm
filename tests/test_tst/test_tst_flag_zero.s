.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0x5
    movs r1, #0xa
    tst r0, r1
    svc #0xFF

