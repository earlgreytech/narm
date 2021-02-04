extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration tests for ADD/SUB operators using the SP

Note that since the SP is aligned by 4 the following applies:
- When adding/diffing with register the lowest 2 bits are ignored/zeroed
- Immediate values have to be multiples of 4, but can be 2 bits "bigger" than listed because of built-in left shift

Included varieties:

ADD <Rd>, SP, #<imm8> T1        - Rd  <- SP  + imm
ADD SP, SP, #<imm7> T2          - SP  <- SP  + imm
ADD <Rdm>, SP, <Rdm> T1         - Rdn <- Rdn + SP
ADD SP, <Rm> T2                 - SP  <- SP  + Rm
SUB SP, SP, #<imm7> T1          - SP  <- SP  - imm

General test cases:

- SP artihmetic with SP as destination + Preserve flags
- SP artihmetic with a register as destination + Preserve flags

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "ADD <Rd>, SP, #<imm8> T1",
    "ADD SP, SP, #<imm7> T2",
    "ADD <Rdm>, SP, <Rdm> T1",
    "ADD SP, <Rm> T2",
    "SUB SP, SP, #<imm7> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &5;

// SP artihmetic with SP as destination + Preserve flags
#[test]
pub fn test_addsub_sp_to_sp() {
    println!(
        "\n>>> Add sp ops test case: SP artihmetic with SP as destination + Preserve flags \n"
    );

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![1, 3, 4];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0x1100_1110));
    common_state!(ops_to_test, vm_states.r[13] = Some(0x0011_CCCC));

    common_state!(ops_to_test, vm_states.n = Some(true));
    common_state!(ops_to_test, vm_states.z = Some(true));
    common_state!(ops_to_test, vm_states.c = Some(true));
    common_state!(ops_to_test, vm_states.v = Some(true));

    // VM initialization

    // 0: ADD <Rd>, SP, #<imm8> T1 - Not applicable

    // 1: ADD SP, SP, #<imm7> T2
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "ADD SP, SP, #0x01FC"
    );
    vm_states[1].r[13] = Some(0x0011_CEC8);

    // 2: ADD <Rdm>, SP, <Rdm> T1 - Not applicable

    // 3: ADD SP, <Rm> T2
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "ADD SP, r0"
    );
    vm_states[3].r[13] = Some(0x1111_DDDC);

    // 4: SUB SP, SP, #<imm7> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "SUB SP, SP, #0x01FC"
    );
    vm_states[4].r[13] = Some(0x0011_CAD0);

    run_test!(vms, vm_states, ops_to_test);
}

// SP artihmetic with register as destination + Preserve flags
#[test]
pub fn test_addsub_sp_to_reg() {
    println!("\n>>> Add sp ops test case: SP artihmetic with register as destination + Preserve flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 2];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0x1100_1110));
    common_state!(ops_to_test, vm_states.r[13] = Some(0x0011_CCCC));

    common_state!(ops_to_test, vm_states.n = Some(true));
    common_state!(ops_to_test, vm_states.z = Some(true));
    common_state!(ops_to_test, vm_states.c = Some(true));
    common_state!(ops_to_test, vm_states.v = Some(true));

    // VM initialization

    // 0: ADD <Rd>, SP, #<imm8> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ADD r0, SP, #0x03FC"
    );
    vm_states[0].r[0] = Some(0x0011_D0C8);

    // 1: ADD SP, SP, #<imm7> T2 - Not applicable

    // 2: ADD <Rdm>, SP, <Rdm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "ADD r0, SP, r0"
    );
    vm_states[2].r[0] = Some(0x1111_DDDC);

    // 3: ADD SP, <Rm> T2 - Not applicable

    // 4: SUB SP, SP, #<imm7> T1 - Not applicable

    run_test!(vms, vm_states, ops_to_test);
}
