extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Push/Pop operators

Included varieties:

PUSH <registers> T1     - Save register list to stack memory, can include LR to save a return address set by branch op
POP <registers> T1      - Load register list from stack memory, can include PC to branch to previously pushed return address

Test cases:

- Save/load lo registers
- Save/load pc (causing branch on load)
- Execute subroutines with local scope and automatic return

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "PUSH <registers> T1",
    "POP <registers> T1",
    "PUSH <registers> T1 and POP <registers> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &3;

// Save/load lo registers
#[test]
pub fn test_pushpop_lo_regs() {
    println!("\n>>> Push/Pop ops test case: Save/load lo registers \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1];

    // VM initialization

    // 0: PUSH <registers> T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0));
    vm_states[0].r[13] = Some(stack_mem_address(8 * WORD_SIZE)); // Stack grows "downwards", so add space for the 8 registers to be pushed
    for i in 0..=7 {
        vm_states[0].r[i as usize] = Some(0xB0 + i);
    }

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "push {r0, r1, r2, r3, r4, r5, r6, r7}"
    );

    vm_states[0].r[13] = Some(stack_mem_address(0));
    for i in 0..=7 {
        vm_states[0].memory[i as usize] = Some(0xB0 + i);
    }

    // 1: POP <registers> T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0));
    vm_states[1].r[13] = Some(stack_mem_address(0));
    for i in 0..=7 {
        vm_states[1].memory[i as usize] = Some(0xB0 + i);
    }

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "pop {r0, r1, r2, r3, r4, r5, r6, r7}"
    );

    vm_states[1].r[13] = Some(stack_mem_address(8 * WORD_SIZE));
    for i in 0..=7 {
        vm_states[1].r[i as usize] = Some(0xB0 + i);
    }

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Save/load pc (causing branch on load)
#[test]
pub fn test_pushpop_pc() {
    println!("\n>>> Push/Pop ops test case: Save/load pc (causing branch on load) \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1];

    // VM initialization

    // 0: PUSH <registers> T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0));
    vm_states[0].r[13] = Some(stack_mem_address(WORD_SIZE)); // Stack grows "downwards", so add space for pc
    vm_states[0].r[14] = Some(code_mem_address(OP_SIZE)); // LR (Link Register) will be saved to stack memory

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "push {lr}"
    );

    vm_states[0].r[13] = Some(stack_mem_address(0));
    vm_states[0].memory[0] = Some(code_mem_address(OP_SIZE));

    // 1: POP <registers> T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0));
    vm_states[1].r[13] = Some(stack_mem_address(0));
    vm_states[1].memory[0] = Some(code_mem_address(2 * OP_SIZE)); // This will be loaded from stack memory to PC and cause a branch

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal = "
        pop {pc}
        svc             #0x01 // This op will be skipped by branch
        svc             #0xFF
        "
    );

    vm_states[1].r[13] = Some(stack_mem_address(WORD_SIZE));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Execute subroutines with local scope and automatic return
#[test]
pub fn test_pushpop_subroutine() {
    println!("\n>>> Push/Pop ops test case: Execute subroutines with local scope and automatic return \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2];

    // VM initialization

    vm_states[2].r[13] = Some(stack_mem_address(20 * WORD_SIZE));

    // Test that registers are properly restored after subroutines
    // R0 and R1 are excluded from push/pops and will instead be set by subroutines
    for i in 2..=7 {
        vm_states[2].r[i as usize] = Some(0xB0 + i);
    }

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal = "
        bl subroutine
        svc             #0xFF
        
        svc             #0x01 // Check that dead code isn't reached
        
        nestedsubroutine: 
        push {r2, r3, r4, r5, r6, r7, lr}
        movs r1,        #0xDC
        movs r2,        #0xFF
        pop {r2, r3, r4, r5, r6, r7, pc}
        
        svc             #0x02 // Check that dead code isn't reached
        
        subroutine: 
        push {r2, r3, r4, r5, r6, r7, lr}
        movs r3,        #0xFF
        movs r4,        #0xFF
        movs r5,        #0xFF
        movs r6,        #0xFF
        movs r7,        #0xFF
        
        movs r2,        #0xCD // Check that nested subroutine doesn't overwrite
        bl nestedsubroutine
        movs r0, r2
        
        pop {r2, r3, r4, r5, r6, r7, pc}
        
        svc             #0x03 // Check that dead code isn't reached
        "
    );

    vm_states[2].r[14] = None; // Ignore LR set by BL ops
    vm_states[2].r[0] = Some(0xCD);
    vm_states[2].r[1] = Some(0xDC);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
