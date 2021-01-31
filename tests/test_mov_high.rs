extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Special integration tests for MOV on at least one high register
TODO: Merge with tests for other MOV/S varieties when those are written

Included varieties:

MOV <Rd>, <Rm> T1       - Rd  <- <Rm>

Included cases:

- Cause a predictable branch when value is moved to PC

TODO: Check alignment for value moved to SP?
TODO: Check if using with SP/PC works to spec?

All tests also check for unexpected changes in untargeted lo registers and condition flags

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &["MOV <Rd>, <Rm> T1"];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &6;

// Cause a predictable branch when value is moved to PC
#[test]
pub fn test_mov_pc() {
    println!("\n>>> Mov ops test case: Cause a predictable branch when value is moved to PC \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let applicable_op_ids = vec![0];

    // VM initialization

    // 0: MOV <Rd>, <Rm> T1
    create_vm!(
        vms,
        vm_states,
        0,
        multiline = true,
        "
        mov  r0, r15
        adds r0,            #0x0A
        mov  r15, r0
        movs r1,            #0xFF
        movs r2,            #0xFF
        movs r3,            #0xFF
        movs r4,            #0xFF
        svc                 #0xFF
        "
    );

    vm_states[0].r[0] = Some(0x0001_000B);
    vm_states[0].r[1] = Some(0x00); // Skipped by branching
    vm_states[0].r[2] = Some(0x00); // Skipped by branching
    vm_states[0].r[3] = Some(0xFF);
    vm_states[0].r[4] = Some(0xFF);

    run_test!(vms, vm_states, applicable_op_ids);
}
