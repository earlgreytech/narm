.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r1, #0x55
    movs r2, #0xAA
    tst r1, r2
1: 
    b 1b @ Halt

