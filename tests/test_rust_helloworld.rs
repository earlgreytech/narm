extern crate narm;
mod common;

use common::*;

#[test]
pub fn test_rust_hello_world() {
    println!("\n>>> Hello World Rust Program Test\n");
    let mut vm = create_vm_from_asm(
        "
        ldr        r0, =0x81000200 
        mov        sp, r0
        bl         main  
        //note if a nop is removed here, it'll turn into an infinite loop   
        //and conversely, if a nop is added here it'll also turn into an infinite loop
        //There is some alignment specific weirdness happening here
        //nop    //replaces svc exit call   
        //nop
        //nop

    __exit:
        svc        #0xff 
    
    __push_costack:
        // svc        #0x10  --not: replaced with nop for testbench 
        nop
        mov        pc, lr


    __system_call:
        // svc        #0x20 --note: replaced with nop for testbench
        nop
        mov        pc, lr

    main:
        push       {r7, lr} 
        add        r7, sp, #0x0
        sub        sp, #0x10
        ldr        r0, =test_string     //argument #1 for method _str_as_ptr
        str        r0, [sp, #0x8]
        movs       r1, #0xb             //argument #2 for method _str_as_ptr
        str        r1, [sp, #0xc]
        bkpt
        bl         _str_as_ptr   
        str        r0, [sp, #0x4]
        ldr        r0, =test_string     // argument #1 for method _str_len
        movs       r1, #0xb             // argument #2 for method _str_len
        bl         _str_len   
        str        r0, [sp]
        ldr        r0, [sp, #0x4]
        ldr        r1, [sp]
        bl         __push_costack   
        movs       r0, #0x5             // argument #1 for method __exit
        bl         __exit               // __exit


    _str_len: 
        push       {r7, lr}
        add        r7, sp, #0x0
        sub        sp, #0x28
        str        r0, [sp, #0x28 + -24]
        str        r1, [sp, #0x28 + -20]
        str        r0, [sp, #0x28 + -16]
        str        r1, [sp, #0x28 + -12]
        str        r0, [sp, #0x28 + -8]
        str        r1, [sp, #0x28 + -4]
        str        r0, [sp, #0x28 + -28]
        str        r1, [sp, #0x28 + -32]
        ldr        r0, [sp, #0x28 + -28]   // argument #1 for method _slice_len
        ldr        r1, [sp, #0x28 + -32]   // argument #2 for method _slice_len
        bl         _slice_len 
        str        r0, [sp, #0x28 + -36]
        add        sp, #0x28
        pop        {r7, pc}


    _str_as_ptr:  
        sub        sp, #0x8  
        str        r0, [sp, #0x8 + -8]
        str        r1, [sp, #0x8 + -4]
        add        sp, #0x8
        bx         lr



    _slice_len: 
        sub        sp, #0x10   
        str        r0, [sp, #0x10 + -8]
        str        r1, [sp, #0x10 + -4]
        str        r0, [sp, #0x10 + -16]
        str        r1, [sp, #0x10 + -12]
        ldr        r0, [sp, #0x10 + -12]
        add        sp, #0x10
        bx         lr


    test_string: 
        .string \"foo bar 123!\" 



    ",
    );
    let result = vm.execute();
    vm.print_diagnostics();
    assert_eq!(result.unwrap(), 0xFF);
}

