extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Load/Store halfword operators

Note:
Addresses used here have to be aligned by 2 / halfword-size
To access the upper memory halfword you add 2 to the word address
Immediate values can actually be 1 bit "larger" than listed, with the lowest bit forced to 0

Included varieties:

LDRH <Rt>, [<Rn>{,#<imm5>}] T1      - Rt <- memhalf(Rn + imm)
LDRH <Rt>,[<Rn>,<Rm>] T1            - Rt <- memhalf(Rn + Rm)
LDRSH <Rt>,[<Rn>,<Rm>] T1           - Rt <- sigextend(memhalf(Rn + Rm))
STRH <Rt>, [<Rn>{,#<imm5>}] T1      - memhalf(Rn + imm) <- Rt
STRH <Rt>,[<Rn>,<Rm>] T1            - memhalf(Rn + Rm)  <- Rt

Test cases:

- Load word-aligned memory halfword to register
- Load word-misaligned memory halfword to register
- Store register to word-aligned memory halfword
- Store register to word-misaligned memory halfword

Special test cases for LDRSH <Rt>,[<Rn>,<Rm>] T1:

- Load word-aligned memory halfword with sign extension
- Load word-misaligned memory halfword with sign extension

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "LDRH <Rt>, [<Rn>{,#<imm5>}] T1",
    "LDRH <Rt>,[<Rn>,<Rm>] T1",
    "LDRSH <Rt>,[<Rn>,<Rm>] T1",
    "STRH <Rt>, [<Rn>{,#<imm5>}] T1",
    "STRH <Rt>,[<Rn>,<Rm>] T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &5;

// Load word-aligned memory halfword to register
#[test]
pub fn test_ldrstr_half_load() {
    println!(
        "\n>>> Ldr/Str (halfword) ops test case: Load word-aligned memory halfword to register \n"
    );

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_0DED));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[13] = Some(stack_mem_address(0))); // SP
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEC));

    // VM initialization

    // 0: LDRH <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0x3C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ldrh r0, [r1, #0x3C]"
    );

    // 1: LDRH <Rt>,[<Rn>,<Rm>] T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ldrh r0, [r1, r2]"
    );

    // 2: LDRSH <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsh r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_0DED));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Load word-misaligned memory halfword to register
#[test]
pub fn test_ldrstr_half_load_misalign() {
    println!("\n>>> Ldr/Str (halfword) ops test case: Load word-misaligned memory halfword to register \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0DED_0000));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[13] = Some(stack_mem_address(0))); // SP
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEE)); // Add 2 to get high halfword

    // VM initialization

    // 0: LDRH <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0x3C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ldrh r0, [r1, #0x3E]"
    );

    // 1: LDRH <Rt>,[<Rn>,<Rm>] T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ldrh r0, [r1, r2]"
    );

    // 2: LDRSH <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsh r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_0DED));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Store register to word-aligned memory halfword
#[test]
pub fn test_ldrstr_half_store() {
    println!(
        "\n>>> Ldr/Str (halfword) ops test case: Store register to word-aligned memory halfword \n"
    );

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_0DED));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEC));

    // VM initialization

    // 3: STRH <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[3].check_memory_start = Some(stack_mem_address(0x3C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "strh r0, [r1, #0x3C]"
    );

    // 4: STRH <Rt>,[<Rn>,<Rm>] T1
    vm_states[4].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "strh r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_0DED));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Store register to word-misaligned memory halfword
#[test]
pub fn test_ldrstr_half_store_misaligned() {
    println!("\n>>> Ldr/Str (halfword) ops test case: Store register to word-misaligned memory halfword \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_0DED));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEE)); // Add 2 to get high halfword

    // VM initialization

    // 3: STRH <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[3].check_memory_start = Some(stack_mem_address(0x3C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "strh r0, [r1, #0x3E]"
    );

    // 4: STRH <Rt>,[<Rn>,<Rm>] T1
    vm_states[4].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "strh r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0DED_0000));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Load word-aligned memory halfword with sign extension
#[test]
pub fn test_ldrstr_half_load_sign() {
    println!("\n>>> LDRSH <Rt>,[<Rn>,<Rm>] T1 special test case: Load word-aligned memory halfword with sign extension \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_8DED));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEC));

    // VM initialization

    // 2: LDRSH <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsh r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFFFF_8DED));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Load word-misaligned memory halfword with sign extension
#[test]
pub fn test_ldrstr_half_load_misaligned_sign() {
    println!("\n>>> LDRSH <Rt>,[<Rn>,<Rm>] T1 special test case: Load word-misaligned memory halfword with sign extension \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x8DED_0000));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEE)); // Add 2 to get high halfword

    // VM initialization

    // 2: LDRSH <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsh r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFFFF_8DED));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
