.syntax unified
.org 0x10000
.section text
_start:
    .globl _start
    mov r0, #100
1: 
    b 1b @ Halt

