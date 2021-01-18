.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0x05
    movs r1, #0xAA
    orrs r0, r1
    svc #0xFF

