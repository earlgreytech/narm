.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0xF1
1: 
    b 1b @ Halt

