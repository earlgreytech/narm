extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Load/Store byte operators

Note:
To access the lowest trough the highest byte in a word you use the word address plus 0 though 3

Included varieties:

LDRB <Rt>, [<Rn>{,#<imm5>}] T1      - Rt <- membyte(Rn + imm)
LDRB <Rt>,[<Rn>,<Rm>] T1            - Rt <- membyte(Rn + Rm)
LDRSB <Rt>,[<Rn>,<Rm>] T1           - Rt <- sigextend(membyte(Rn + Rm))
STRB <Rt>, [<Rn>{,#<imm5>}] T1      - membyte(Rn + imm) <- Rt
STRB <Rt>,[<Rn>,<Rm>] T1            - membyte(Rn + Rm)  <- Rt

Test cases:

- Load word-aligned memory byte to register
- Load word-misaligned memory byte to register
- Store register to word-aligned memory byte
- Store register to word-misaligned memory byte

Special test cases for LDRSB <Rt>,[<Rn>,<Rm>] T1:

- Load word-aligned memory byte with sign extension
- Load word-misaligned memory byte with sign extension

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "LDRB <Rt>, [<Rn>{,#<imm5>}] T1",
    "LDRB <Rt>,[<Rn>,<Rm>] T1",
    "LDRSB <Rt>,[<Rn>,<Rm>] T1",
    "STRB <Rt>, [<Rn>{,#<imm5>}] T1",
    "STRB <Rt>,[<Rn>,<Rm>] T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &5;

// Load word-aligned memory byte to register
#[test]
pub fn test_ldrstr_byte_load() {
    println!("\n>>> Ldr/Str (byte) ops test case: Load word-aligned memory byte to register \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_001D));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[13] = Some(stack_mem_address(0))); // SP
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEC));

    // VM initialization

    // 0: LDRB <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0x1C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ldrb r0, [r1, #0x1C]"
    );

    // 1: LDRB <Rt>,[<Rn>,<Rm>] T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ldrb r0, [r1, r2]"
    );

    // 2: LDRSB <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsb r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_001D));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Load word-misaligned memory byte to register
#[test]
pub fn test_ldrstr_byte_load_misalign() {
    println!("\n>>> Ldr/Str (byte) ops test case: Load word-misaligned memory byte to register \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_1D00));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[13] = Some(stack_mem_address(0))); // SP
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEED)); // Add 1 to get second-lowest byte

    // VM initialization

    // 0: LDRB <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0x1C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ldrb r0, [r1, #0x1D]"
    );

    // 1: LDRB <Rt>,[<Rn>,<Rm>] T1
    vm_states[1].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ldrb r0, [r1, r2]"
    );

    // 2: LDRSB <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsb r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_001D));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Store register to word-aligned memory byte
#[test]
pub fn test_ldrstr_byte_store() {
    println!("\n>>> Ldr/Str (byte) ops test case: Store register to word-aligned memory byte \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_001D));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEC));

    // VM initialization

    // 3: STRB <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[3].check_memory_start = Some(stack_mem_address(0x1C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "strb r0, [r1, #0x1C]"
    );

    // 4: STRB <Rt>,[<Rn>,<Rm>] T1
    vm_states[4].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "strb r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_001D));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Store register to word-misaligned memory byte
#[test]
pub fn test_ldrstr_byte_store_misaligned() {
    println!(
        "\n>>> Ldr/Str (byte) ops test case: Store register to word-misaligned memory byte \n"
    );

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_001D));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEED)); // Add 1 to get second-lowest byte

    // VM initialization

    // 3: STRB <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[3].check_memory_start = Some(stack_mem_address(0x1C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "strb r0, [r1, #0x1D]"
    );

    // 4: STRB <Rt>,[<Rn>,<Rm>] T1
    vm_states[4].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "strb r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_1D00));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Load word-aligned memory byte with sign extension
#[test]
pub fn test_ldrstr_byte_load_sign() {
    println!("\n>>> LDRSB <Rt>,[<Rn>,<Rm>] T1 special test case: Load word-aligned memory byte with sign extension \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_009D));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEC));

    // VM initialization

    // 2: LDRSB <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsb r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFFFF_FF9D));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Load word-misaligned memory byte with sign extension
#[test]
pub fn test_ldrstr_byte_load_misaligned_sign() {
    println!("\n>>> LDRSB <Rt>,[<Rn>,<Rm>] T1 special test case: Load word-misaligned memory byte with sign extension \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0000_9D00));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEED)); // Add 1 to get second-lowest byte

    // VM initialization

    // 2: LDRSB <Rt>,[<Rn>,<Rm>] T1
    vm_states[2].check_memory_start = Some(stack_mem_address(0xBEEC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ldrsb r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFFFF_FF9D));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
