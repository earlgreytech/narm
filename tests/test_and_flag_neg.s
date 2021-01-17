.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0x2
    lsls r0, #0xf
    lsls r0, #0xf
    ands r0, r0
1: 
    b 1b @ Halt

