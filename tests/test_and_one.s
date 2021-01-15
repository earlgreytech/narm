.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0xF
    movs r1, #0x1
    ands r0, r1
1: 
    b 1b @ Halt

