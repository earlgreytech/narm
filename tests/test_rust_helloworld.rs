extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Branch operators

Included varieties:

B <label> T2            - Branch unconditionally by label, jump size restricted to signed 11 bits
B<C> <label> T1         - Branch conditionally by label, jump size restricted to signed 8 bits
BX <Rm> T1              - Branch by register
BL <label> T1           - Branch by label, set link register, jump size restricted to even signed 25 bits (?????)
BLX <Rm> T1             - Branch by register, set link register

General test cases:

- Branch forward
- Branch backward
- Branch far forward (Causing memory error)
- Branch far backward (Causing memory error)

Special test case for BLX <Rm> T1:

- Branch and then branch back using address saved in link register

Special test case for B<C>:

- Test all different conditions

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "B <label> T2",
    "B<C> <label> T1",
    "BX <Rm> T1",
    "BL <label> T1 32bit",
    "BLX <Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &5;

// Branch forward
#[test]
pub fn test_rust_hello_world() {
    println!("\n>>> Hello World Rust Program Test\n");
    let mut vm = create_vm_from_asm(
        "


.syntax unified
.section .text
.thumb_func
.globl _start
_start:



/*
--------------------------------------------------------------------------------

    File: /Users/earlz/neutron-star-test/target/thumbv6m-none-eabi/debug/neutron-star-test
    File created with Hopper 4.7.0
    Analysis version 58
    ELF file
    CPU: arm/v6
    32 bits addresses (Little Endian)

--------------------------------------------------------------------------------
*/



    // Segment Segment 0
    // Range: [0x10000// 0x100a3[ (163 bytes)
    // File offset : [65536// 65699[ (163 bytes)
    // Permissions: readable / executable
    // Flags: 0x5



    // Section .text
    // Range: [0x10000// 0x100a3[ (163 bytes)
    // File offset : [65536// 65699[ (163 bytes)
    // Flags: 0x6
    //   SHT_PROGBITS
    //   SHF_ALLOC
    //   SHF_EXECINSTR



    // ================ B E G I N N I N G   O F   P R O C E D U R E ================

        _start:
         __boot:
     ldr        r0, =0x81000200                                     // dword_10014
     mov        sp, r0
     bl         main                                                // main
     svc        #0xff
         __exit:
     svc        #0xff                                               // CODE XREF=main+46
         __push_costack:
     // svc        #0x10  --note: replaced with nop for testbench    // CODE XREF=main+38
     nop
     mov        pc, lr
                    // endp


    // ================ B E G I N N I N G   O F   P R O C E D U R E ================


         __system_call:
     // svc        #0x20 --note: replaced with nop for testbench
     nop
     mov        pc, lr
                    // endp
         dword_10014:
     .quad         0x81000200                                          // DATA XREF=__boot
         main:
     push       {r7, lr}                                            // CODE XREF=__boot+4
     add        r7, sp, #0x0
     sub        sp, #0x10
     ldr        r0, =sub_10098                                      // 0x1004c
     str        r0, [sp, #0x8]
     movs       r1, #0xb                                            // argument #2 for method _ZN4core3str21_$LT$impl$u20$str$GT$6as_ptr17h00fa268ff4863332E
     str        r1, [sp, #0xc]
     bl         _ZN4core3str21_$LT$impl$u20$str$GT$6as_ptr17h00fa268ff4863332E // core::str::_$LT$impl$u20$str$GT$::as_ptr::h00fa268ff4863332
     str        r0, [sp, #0x4]
     b          main+22
     ldr        r0, =sub_10098                                      // argument #1 for method _ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE, 0x1004c,sub_10098, CODE XREF=main+20
     movs       r1, #0xb                                            // argument #2 for method _ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE
     bl         _ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE // core::str::_$LT$impl$u20$str$GT$::len::h7c07257dd994f6dd
     str        r0, [sp]
     b          main+34
     ldr        r0, [sp, #0x4]                                      // CODE XREF=main+32
     ldr        r1, [sp]
     bl         __push_costack                                      // __push_costack
     b          main+44
     movs       r0, #0x5                                            // argument #1 for method __exit, CODE XREF=main+42
     bl         __exit                                              // __exit


    // ================ B E G I N N I N G   O F   P R O C E D U R E ================


         sub_1004a:
    .inst 0xde00
     lsls       r0, r3, #0x2                                        // DATA XREF=main+6, main+22
     movs       r1, r0
                    // endp


    // ================ B E G I N N I N G   O F   P R O C E D U R E ================

    // Variables:
    //    var_4: int32_t, -4
    //    var_8: int32_t, -8
    //    var_C: int32_t, -12
    //    var_10: int32_t, -16
    //    var_14: int32_t, -20
    //    var_18: int32_t, -24
    //    var_1C: int32_t, -28
    //    var_20: int32_t, -32
    //    var_24: int32_t, -36


         _ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE:        // core::str::_$LT$impl$u20$str$GT$::len::h7c07257dd994f6dd
     push       {r7, lr}                                            // CODE XREF=main+26
     add        r7, sp, #0x0
     sub        sp, #0x28
     str        r0, [sp, #0x28 + -24]
     str        r1, [sp, #0x28 + -20]
     str        r0, [sp, #0x28 + -16]
     str        r1, [sp, #0x28 + -12]
     str        r0, [sp, #0x28 + -8]
     str        r1, [sp, #0x28 + -4]
     ldr        r0, [sp, #0x28 + -8]
     ldr        r1, [sp, #0x28 + -4]
     str        r0, [sp, #0x28 + -28]
     str        r1, [sp, #0x28 + -32]
     b          loc_1006c

         loc_1006c:
     ldr        r0, [sp, #0x28 + -28]                            // argument #1 for method _ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hf4276695aa93f7dfE, CODE XREF=_ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE+26
     ldr        r1, [sp, #0x28 + -32]                            // argument #2 for method _ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hf4276695aa93f7dfE
     bl         _ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hf4276695aa93f7dfE // core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::len::hf4276695aa93f7df
     str        r0, [sp, #0x28 + -36]
     b          loc_10078

         loc_10078:
     ldr        r0, [sp, #0x28 + -36]                            // CODE XREF=_ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE+38
     add        sp, #0x28
     pop        {r7, pc}
                    // endp


    // ================ B E G I N N I N G   O F   P R O C E D U R E ================

    // Variables:
    //    var_4: int32_t, -4
    //    var_8: int32_t, -8


         _ZN4core3str21_$LT$impl$u20$str$GT$6as_ptr17h00fa268ff4863332E:        // core::str::_$LT$impl$u20$str$GT$::as_ptr::h00fa268ff4863332
     sub        sp, #0x8                                            // CODE XREF=main+14
     str        r0, [sp, #0x8 + -8]
     str        r1, [sp, #0x8 + -4]
     add        sp, #0x8
     bx         lr
                    // endp


    // ================ B E G I N N I N G   O F   P R O C E D U R E ================

    // Variables:
    //    var_4: int32_t, -4
    //    var_8: int32_t, -8
    //    var_C: int32_t, -12
    //    var_10: int32_t, -16


         _ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hf4276695aa93f7dfE:        // core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::len::hf4276695aa93f7df
     sub        sp, #0x10                                           // CODE XREF=_ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE+32
     str        r0, [sp, #0x10 + -8]
     str        r1, [sp, #0x10 + -4]
     str        r0, [sp, #0x10 + -16]
     str        r1, [sp, #0x10 + -12]
     ldr        r0, [sp, #0x10 + -12]
     add        sp, #0x10
     bx         lr
                    // endp


    // ================ B E G I N N I N G   O F   P R O C E D U R E ================


         sub_10098:
     ldr        r6, [r4, #0x74]                                     // DATA XREF=main+22
     str        r7, [r5, #0x24]
     strb       r1, [r4, #0x9]
     adds       r1, #0x20
     adds       r3, #0x32
                    // endp
     .byte  0x21 // '!'



    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0xF1);

    assert_vm_eq!(vm_expected, vm);
}

