extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Sign Extension operators

Included varieties:

SXTB <Rd>,<Rm> T1           - [byte3, byte2, byte1, byte0] -> [sigext(byte0), byte0]
SXTH <Rd>,<Rm> T1           - [byte3, byte2, byte1, byte0] -> [sigext(byte1), byte1, byte0]
UXTB <Rd>,<Rm> T1           - [byte3, byte2, byte1, byte0] -> [0000, 0000, 0000, byte0]
UXTH <Rd>,<Rm> T1           - [byte3, byte2, byte1, byte0] -> [0000, 0000, byte1, byte0]

Test cases:

- Sign extend positive number
- Sign extend negative number

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "SXTB <Rd>,<Rm> T1",
    "SXTH <Rd>,<Rm> T1",
    "UXTB <Rd>,<Rm> T1",
    "UXTH <Rd>,<Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &4;

// Sign extend positive number
#[test]
pub fn test_signextend_positive() {
    println!("\n>>> Sign Extension ops test case: Sign extend positive number \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0xBEEF_7654));

    // VM initialization

    // 0: SXTB <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "sxtb r0, r1"
    );
    vm_states[0].r[0] = Some(0x54);

    // 1: SXTH <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "sxth r0, r1"
    );
    vm_states[1].r[0] = Some(0x7654);

    // 2: UXTB <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "uxtb r0, r1"
    );
    vm_states[2].r[0] = Some(0x54);

    // 3: UXTH <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "uxth r0, r1"
    );
    vm_states[3].r[0] = Some(0x7654);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Sign extend negative number
#[test]
pub fn test_signextend_negative() {
    println!("\n>>> Sign Extension ops test case: Sign extend negative number \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0xBEEF_9786));

    // VM initialization

    // 0: SXTB <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "sxtb r0, r1"
    );
    vm_states[0].r[0] = Some(0xFFFF_FF86);

    // 1: SXTH <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "sxth r0, r1"
    );
    vm_states[1].r[0] = Some(0xFFFF_9786);

    // 2: UXTB <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "uxtb r0, r1"
    );
    vm_states[2].r[0] = Some(0x86);

    // 3: UXTH <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "uxth r0, r1"
    );
    vm_states[3].r[0] = Some(0x9786);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
