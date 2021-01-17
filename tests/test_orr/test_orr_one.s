.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    movs r0, #0x0
    movs r1, #0x1
    orrs r0, r1
    svc #0x1
1: 
    b 1b @ Halt

