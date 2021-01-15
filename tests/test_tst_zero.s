.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    tst r1, r2
1: 
    b 1b @ Halt

