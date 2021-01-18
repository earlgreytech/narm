.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    orrs r0, r1
    svc #0xFF

