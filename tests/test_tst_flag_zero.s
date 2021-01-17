.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    tst r0, r0
    svc #0x1
1: 
    b 1b @ Halt

