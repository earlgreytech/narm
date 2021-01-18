.syntax unified
.section .text
.thumb_func
_start:
    .globl _start
    orrs r0, r0
    svc #0xFF

