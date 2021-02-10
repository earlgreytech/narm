extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Load/Store word operators

Note:
Addresses used here have to be aligned by 4 / word-size
Immediate values are actually 2 bits "larger", with the 2 lowest bits forced to 0

Included varieties:

LDR <Rt>, [<Rn>{,#<imm5>}] T1       - Rt <- memword(Rn + imm)
LDR <Rt>,[SP{,#<imm8>}] T2          - Rt <- memword(SP + imm)
LDR <Rt>,<label> T1                 - Rt <- memword(label) // Label is turned into pc + offset by compiler
LDR <Rt>,[<Rn>,<Rm>] T1             - Rt <- memword(Rn + Rm)
STR <Rt>, [<Rn>{,#<imm5>}] T1       - memword(Rn + imm) <- Rt
STR <Rt>,[SP{,#<imm8>}] T2          - memword(SP + imm) <- Rt
STR <Rt>,[<Rn>,<Rm>] T1             - memword(Rn + Rm)  <- Rt

Test cases:

- Load memory word to register
- Store register to memory word

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "LDR <Rt>, [<Rn>{,#<imm5>}] T1",
    "LDR <Rt>,[SP{,#<imm8>}] T2",
    "LDR <Rt>,<label> T1",
    "LDR <Rt>,[<Rn>,<Rm>] T1",
    "STR <Rt>, [<Rn>{,#<imm5>}] T1",
    "STR <Rt>,[SP{,#<imm8>}] T2",
    "STR <Rt>,[<Rn>,<Rm>] T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &7;

// Load memory word to register
#[test]
pub fn test_ldrstr_load() {
    println!("\n>>> Ldr/Str (word) ops test case: Load memory word to register \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0DED_BEEF));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[13] = Some(stack_mem_address(0))); // SP
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEF));

    // VM initialization

    // 0: LDR <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[0].check_memory_start = Some(stack_mem_address(0x7C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ldr r0, [r1, #0x7C]"
    );

    // 1: LDR <Rt>,[SP{,#<imm8>}] T2
    vm_states[1].check_memory_start = Some(stack_mem_address(0x03FC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ldr r0, [SP, #0x03FC]"
    );

    // 2: LDR <Rt>,<label> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal = "
        ldr r0, loadinstruction
        nop                         // Needed so offset from PC doesn't turn negative -> invalid
        nop
        nop
        loadinstruction:
        svc                 #0xFF
        "
    );

    // 3: LDR <Rt>,[<Rn>,<Rm>] T1
    vm_states[3].check_memory_start = Some(stack_mem_address(0xBEEF));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "ldr r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0DED_BEEF));
    vm_states[2].r[0] = Some(0xDFFF); // Opcode for SVC #0xFF

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Store register to memory word
#[test]
pub fn test_ldrstr_store() {
    println!("\n>>> Ldr/Str (word) ops test case: Store register to memory word \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0DED_BEEF));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(stack_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].r[13] = Some(stack_mem_address(0))); // SP
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0xBEEF));

    // VM initialization

    // 4: STR <Rt>, [<Rn>{,#<imm5>}] T1
    vm_states[4].check_memory_start = Some(stack_mem_address(0x7C));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "str r0, [r1, #0x7C]"
    );

    // 5: STR <Rt>,[SP{,#<imm8>}] T2
    vm_states[5].check_memory_start = Some(stack_mem_address(0x03FC));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "str r0, [SP, #0x03FC]"
    );

    // 6: STR <Rt>,[<Rn>,<Rm>] T1
    vm_states[6].check_memory_start = Some(stack_mem_address(0xBEEF));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "str r0, [r1, r2]"
    );

    set_for_all!(vm_states[ops_to_test].memory[0] = Some(0x0DED_BEEF));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
