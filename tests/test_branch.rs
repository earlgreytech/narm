extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Unit test for Branch operators

Included varieties:

B <label> T2            - Branch unconditionally by label, jump size restricted to signed 11 bits
B<cond> <label> T1      - Branch conditionally by label, jump size restricted to signed 8 bits
BX <Rm> T1              - Branch by register
BL <label T1            - Branch by label, set link register, jump size restricted to even signed 25 bits (?????)
BLX <Rm> T1             - Branch by register, set link register 

General test cases:

- Branch forward
- Branch far forward (Causing memory error)
- Branch backward
- Branch far backward (Causing memory error)
- Call, and return from, subroutine using link register

Special test case for B<cond>

- Test all different conditions

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "B <label> T2",
    "B<cond> <label> T1",
    "BX <Rm> T1",
    "BL <label T1 32bit",
    "BLX <Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &5;

// Branch forward
#[test]
pub fn test_branch_forward() {
    println!("\n>>> Branch ops test case: Branch forward \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    //let applicable_op_ids = vec![0, 1, 2, 3, 4];
    let applicable_op_ids = vec![0, 1, 2, 3, 4];

    // Common pre-execution state
    common_state!(applicable_op_ids, vm_states.r[1] = Some(ASM_ENTRY + 2*OP_SIZE + THUMBS_MODE));

    // VM initialization
    let post_ops = 
        "
        svc             #0x01
        test1:
        movs r0,        #0xCD
        svc             #0xFF
        ";

    // 0: B <label> T2" 
    let ops0 = format!("b test1 {}", post_ops);
    create_vm!(vms, vm_states, 0, code_var = true, ops0);
    
    // 1: B<cond> <label> T1 (BNE -> if flag Z = 0)
    let ops1 = format!("bne test1 {}", post_ops);
    create_vm!(vms, vm_states, 1, code_var = true, ops1);

    // 2: BX <Rm> T1
    let ops2 = format!("bx r1 {}", post_ops);
    create_vm!(vms, vm_states, 2, code_var = true, ops2);

    // 3: BL <label T1 32bit
    let ops3 = format!("bl test1 {}", post_ops);
    create_vm!(vms, vm_states, 3, code_var = true, ops3);
    vm_states[3].r[14] = Some(ASM_ENTRY + OP_SIZE_32BIT + THUMBS_MODE);

    // 4: BLX <Rm> T1
    let ops4 = format!("blx r1 {}", post_ops);
    create_vm!(vms, vm_states, 4, code_var = true, ops4);
    vm_states[4].r[14] = Some(ASM_ENTRY + OP_SIZE + THUMBS_MODE);
    
    // Common expected post-execution state
    common_state!(applicable_op_ids, vm_states.r[0] = Some(0xCD));

    run_test!(vms, vm_states, applicable_op_ids);
}

//this will test small nearby jumps using bl
#[test]
pub fn test_bl_small_jump() {
    println!("\n>>> TODOOOOOT \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let applicable_op_ids = vec![3];
    create_vm!(vms, vm_states, 3, multiline = true, 
        "
        bl test1
        svc             #0x01
        test1:
        movs r0,        #0x01
        svc             #0xFF
    "
    );

    vm_states[3].r[0] = Some(1);
    vm_states[3].r[14] = Some(ASM_ENTRY + OP_SIZE_32BIT + THUMBS_MODE);

    run_test!(vms, vm_states, applicable_op_ids);
}

#[test]
pub fn test_bl_big_jump() {
    //note this test will access unavailable memory
    println!("\n>>> TODOOOOOT \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let applicable_op_ids = vec![3];
    create_vm!(vms, vm_states, 3, multiline = true, 
        "
        bl              #0xF0020
        svc             #0x01
    "
    );

    vm_states[3].expect_exec_error = true;
    vm_states[3].r[14] = Some(ASM_ENTRY + OP_SIZE_32BIT + THUMBS_MODE);
    vm_states[3].pc_address = Some(0xF_0020);

    run_test!(vms, vm_states, applicable_op_ids);
}

#[test]
pub fn test_bl_backward_jump() {
    //note this test will access unavailable memory
    println!("\n>>> TODOOOOOT \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let applicable_op_ids = vec![3];
    create_vm!(vms, vm_states, 3, multiline = true, 
        "
        bl              #0x50
        svc             #0x01
    "
    );

    vm_states[3].expect_exec_error = true;
    vm_states[3].r[14] = Some(ASM_ENTRY + OP_SIZE_32BIT + THUMBS_MODE);
    vm_states[3].pc_address = Some(0x50);

    run_test!(vms, vm_states, applicable_op_ids);
}
