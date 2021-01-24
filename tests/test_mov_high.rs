extern crate narm;
mod common;

use common::*;

/*

Special unit test for MOV on at least one high register
TODO: Merge with tests for other MOV/S varieties when those are written

Included cases:

- Cause a predictable branch when value is moved to PC
- Correctly move values between two high registers

TODO: Check alignment for value moved to SP?
TODO: Check if using with SP/PC works to spec?

All tests also check for unexpected changes in untargeted lo registers and condition flags

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if operation causes a predictable branch when value is moved to PC
#[test]
pub fn test_mov_pc() {
    /*
        mov  r0  <- r15     // Move PC into r0, we will jump with this op as reference
        adds r0  <- #0x0A   // Since instructions are 2 byte, this represent a 5 op forward jump
        mov  r15 <- r0      // Move r0 back to PC. Since we're already 2 op ahead of reference we'll jump 3 ops forward
        movs r1  <- #0xFF   // Skipped
        movs r2  <- #0xFF   // Skipped
        movs r3  <- #0xFF   // This is where the jump takes us!
        ...
    */
    let mut vm = create_vm_from_asm(
        "
        mov  r0, r15
        adds r0,            #0x0A
        mov  r15, r0
        movs r1,            #0xFF
        movs r2,            #0xFF
        movs r3,            #0xFF
        movs r4,            #0xFF
        movs r5,            #0xFF
        svc                 #0xFF
    ",
    );
    //vm.cycle().unwrap(); // Step 1 instruction to set last_pc
    //vm.set_pc(vm.get_last_pc)
    assert_eq!(vm.execute().unwrap(), 0x0000_00FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = None;
    //vm_expected.r[1] = Some(0xFF); //Skipped, will still be 0
    //vm_expected.r[2] = Some(0xFF); //Skipped, will still be 0
    vm_expected.r[3] = Some(0xFF);
    vm_expected.r[4] = Some(0xFF);
    vm_expected.r[5] = Some(0xFF);

    assert_vm_eq!(vm_expected, vm);
}

// Test if operation correctly moves values between two high registers
#[test]
pub fn test_mov_between_high() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0x0A
        mov  r8, r0
        mov  r9, r8
        mov  r1, r9
        svc                 #0xFF
    ",
    );

    assert_eq!(vm.execute().unwrap(), 0x0000_00FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0x0A);
    vm_expected.r[1] = Some(0x0A);
    vm_expected.r[8] = Some(0x0A);
    vm_expected.r[9] = Some(0x0A);

    assert_vm_eq!(vm_expected, vm);
}
