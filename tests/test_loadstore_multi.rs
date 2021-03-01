extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Load/Store Multiple operators

NOTE: Unlike stack operators (PUSH/POP) these operators always read/write a rising sequence of words relative to the base address
NOTE: The variants with ! after Rn must not include Rn in the register list, while the one without must include it

Included varieties:

LDM <Rn>!,<registers> T1            - [reglist] <- [mem(Rn) ... mem(Rn + 4*(size(reglist)-1))], Rn <- Rn + 4*size(reglist)
LDM <Rn>,<registers> T1             - Same as above, but since Rn is included in reglist the "incremented address" isn't written back to it (it gets a memory value loaded into it instead)
STM <Rn>!,<registers> T1            - [mem(Rn) ... mem(Rn + 4*(size(reglist)-1))] <- [reglist], Rn <- Rn + 4*size(reglist)

Test cases:

- Load/store single register
- Load/store multiple registers
- Store and then load back register values

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "LDM <Rn>!,<registers> T1",
    "LDM <Rn>,<registers> T1",
    "STM <Rn>!,<registers> T1",
    "STM <Rn>!,<registers> T1 AND LDM <Rn>!,<registers> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &4;

// Load/store single register
#[test]
pub fn test_loadstore_multi_single() {
    println!("\n>>> Load/Store multiple ops test case: Load/store single register \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2];

    // VM initialization

    // 0: LDM <Rn>!,<registers> T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0));
    vm_states[0].r[3] = Some(stack_mem_address(0));
    vm_states[0].memory[0] = Some(0xDEAD_BEEF);

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ldm r3!, {r5}"
    );

    vm_states[0].r[5] = Some(0xDEAD_BEEF);
    vm_states[0].r[3] = Some(stack_mem_address(WORD_SIZE));

    // 1: LDM <Rn>,<registers> T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0));
    vm_states[1].r[3] = Some(stack_mem_address(0));
    vm_states[1].memory[0] = Some(0xDEAD_BEEF);

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ldm r3, {r3}"
    );

    vm_states[1].r[3] = Some(0xDEAD_BEEF);

    // 2: STM <Rn>!,<registers> T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0));
    vm_states[2].r[3] = Some(stack_mem_address(0));
    vm_states[2].r[5] = Some(0xDEAD_BEEF);

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "stm r3!, {r5}"
    );

    vm_states[2].memory[0] = Some(0xDEAD_BEEF);
    vm_states[2].r[3] = Some(stack_mem_address(WORD_SIZE));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Load/store multiple registers
#[test]
pub fn test_loadstore_multi_many() {
    println!("\n>>> Load/Store multiple ops test case: Load/store multiple registers \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2];

    // VM initialization

    // 0: LDM <Rn>!,<registers> T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0));
    vm_states[0].r[3] = Some(stack_mem_address(0));

    // Split range to account for r3 not being in reg list
    for i in 0..=2 {
        vm_states[0].memory[i as usize] = Some(0xB0 + i);
    }
    for i in 4..=7 {
        vm_states[0].memory[(i - 1) as usize] = Some(0xB0 + i);
    }

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ldm r3!, {r0, r1, r2, r4, r5, r6, r7}"
    );

    // Not need to split range here, but r3 will get a memory address instead of this value
    for i in 0..=7 {
        vm_states[0].r[i as usize] = Some(0xB0 + i);
    }
    vm_states[0].r[3] = Some(stack_mem_address(7 * WORD_SIZE));

    // 1: LDM <Rn>,<registers> T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0));
    vm_states[1].r[3] = Some(stack_mem_address(0));
    for i in 0..=7 {
        vm_states[1].memory[i as usize] = Some(0xB0 + i);
    }

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ldm r3, {r0, r1, r2, r3, r4, r5, r6, r7}"
    );

    for i in 0..=7 {
        vm_states[1].r[i as usize] = Some(0xB0 + i);
    }

    // 2: STM <Rn>!,<registers> T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0));

    // Not need to split range here, but r3 will contain the base memory address instead of this value
    for i in 0..=7 {
        vm_states[2].r[i as usize] = Some(0xB0 + i);
    }

    vm_states[2].r[3] = Some(stack_mem_address(0));

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "stm r3!, {r0, r1, r2, r4, r5, r6, r7}"
    );

    // Split range to account for r3 not being in reg list
    for i in 0..=2 {
        vm_states[2].memory[i as usize] = Some(0xB0 + i);
    }
    for i in 4..=7 {
        vm_states[2].memory[(i - 1) as usize] = Some(0xB0 + i);
    }
    vm_states[2].r[3] = Some(stack_mem_address(7 * WORD_SIZE));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Store and then load back register values
#[test]
pub fn test_loadstore_multi_reload() {
    println!(
        "\n>>> Load/Store multiple ops test case: Store and then load back register values \n"
    );

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![3];

    // VM initialization

    vm_states[3].check_memory_start = Some(stack_mem_address(0));

    // r3 and r4 will be overwritten with memory addresses
    for i in 0..=7 {
        vm_states[3].r[i as usize] = Some(0xB0 + i);
    }
    // No need to check memory in this test, if it's stored incorrectly the registers will be wrong anyway
    for i in 0..=5 {
        vm_states[3].memory[i as usize] = None;
    }

    vm_states[3].r[3] = Some(stack_mem_address(0));

    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal = "
        movs r4, r3             // Save memory address (STM will increment it)
        stm r3!, {r0, r1, r2, r5, r6, r7}
        movs r3, r4             // Restore memory address
        movs r0, #0xFF
        movs r1, #0xFF
        movs r2, #0xFF
        // r3 contains the address to load from, so we can't write over it
        movs r4, #0xFF
        movs r5, #0xFF
        movs r6, #0xFF
        movs r7, #0xFF
        ldm r3!, {r0, r1, r2, r5, r6, r7}
        svc      #0xFF
        "
    );

    // Registers except for r3 and r4 should be unchanged

    vm_states[3].r[3] = Some(stack_mem_address(6 * WORD_SIZE));
    vm_states[3].r[4] = Some(0xFF);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
