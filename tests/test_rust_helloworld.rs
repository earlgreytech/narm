extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

#[test]
pub fn test_rust_hello_world() {
    println!("\n>>> Hello World Rust Program Test\n");
    let mut vm = create_vm_from_asm(
        "
         _start:
         __boot:
     ldr        r0, =0x81000200                                     // dword_10014
     mov        sp, r0
     bl         main  
     //note if a nop is removed here, it'll turn into an infinite loop   
     nop    //replaces svc exit call   
     //and conversely, if a nop is added here it'll also turn into an infinite loop

         __exit:
     svc        #0xff                                               // CODE XREF=main+46
         __push_costack:
     // svc        #0x10  --not: replaced with nop for testbench    // CODE XREF=main+38
     nop
     mov        pc, lr


         __system_call:
     // svc        #0x20 --note: replaced with nop for testbench
     nop
     mov        pc, lr

         dword_10014:
     .quad         0x81000200                                          // DATA XREF=__boot

         main:
     push       {r7, lr}                                            // CODE XREF=__boot+4
     add        r7, sp, #0x0
     sub        sp, #0x10
     ldr        r0, =test_string                                      // 0x1004c
     str        r0, [sp, #0x8]
     movs       r1, #0xb                                            // argument #2 for method _ZN4core3str21_$LT$impl$u20$str$GT$6as_ptr17h00fa268ff4863332E
     str        r1, [sp, #0xc]
     bl         _str_as_ptr    // core::str::_$LT$impl$u20$str$GT$::as_ptr::h00fa268ff4863332
     str        r0, [sp, #0x4]
     ldr        r0, =test_string                                      // argument #1 for method _ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE, 0x1004c,sub_10098, CODE XREF=main+20
     movs       r1, #0xb                                            // argument #2 for method _ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE
     bl         _str_len      // core::str::_$LT$impl$u20$str$GT$::len::h7c07257dd994f6dd
     str        r0, [sp]
     ldr        r0, [sp, #0x4]                                      // CODE XREF=main+32
     ldr        r1, [sp]
     bl         __push_costack                                      // __push_costack
     movs       r0, #0x5                                            // argument #1 for method __exit, CODE XREF=main+42
     bl         __exit                                              // __exit


         sub_1004a:
     .inst 0xde00 //purposefully invalid instruction
     lsls       r0, r3, #0x2                                        // DATA XREF=main+6, main+22
     movs       r1, r0


         _str_len:        // core::str::_$LT$impl$u20$str$GT$::len::h7c07257dd994f6dd
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
         loc_1006c:
     ldr        r0, [sp, #0x28 + -28]                            // argument #1 for method _ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hf4276695aa93f7dfE, CODE XREF=_ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE+26
     ldr        r1, [sp, #0x28 + -32]                            // argument #2 for method _ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hf4276695aa93f7dfE
     bl         _slice_len // core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::len::hf4276695aa93f7df
     str        r0, [sp, #0x28 + -36]
         loc_10078:
     ldr        r0, [sp, #0x28 + -36]                            // CODE XREF=_ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE+38
     add        sp, #0x28
     pop        {r7, pc}
                    // endp


         _str_as_ptr:        // core::str::_$LT$impl$u20$str$GT$::as_ptr::h00fa268ff4863332
     sub        sp, #0x8                                            // CODE XREF=main+14
     str        r0, [sp, #0x8 + -8]
     str        r1, [sp, #0x8 + -4]
     add        sp, #0x8
     bx         lr
                    // endp



         _slice_len:        // core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::len::hf4276695aa93f7df
     sub        sp, #0x10                                           // CODE XREF=_ZN4core3str21_$LT$impl$u20$str$GT$3len17h7c07257dd994f6ddE+32
     str        r0, [sp, #0x10 + -8]
     str        r1, [sp, #0x10 + -4]
     str        r0, [sp, #0x10 + -16]
     str        r1, [sp, #0x10 + -12]
     ldr        r0, [sp, #0x10 + -12]
     add        sp, #0x10
     bx         lr


         test_string: 
     //these instructions encode the string 'foobar 123!' 
     ldr        r6, [r4, #0x74]                                     // DATA XREF=main+22
     str        r7, [r5, #0x24]
     strb       r1, [r4, #0x9]
     adds       r1, #0x20
     adds       r3, #0x32
     .byte  0x21 // '!'



    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
}

