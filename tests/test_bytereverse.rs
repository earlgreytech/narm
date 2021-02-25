extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for byte reverse operators

Included varieties:

REV <Rd>,<Rm> T1            - [b4,b3,b2,b1] -> [b1,b2,b3,b4]
REV16 <Rd>,<Rm> T1          - [b4,b3,b2,b1] -> [b3,b4,b1,b2]
REVSH <Rd>,<Rm> T1          - [_,_,b2,b1]   -> [signextend(b1),b1,b2]

General test cases:

- Reverse bytes

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "REV <Rd>,<Rm> T1",
    "REV16 <Rd>,<Rm> T1",
    "REVSH <Rd>,<Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &3;

// Reverse bytes
#[test]
pub fn test_bytereverse_reverse() {
    println!("\n>>> Byte reverse ops test case: Reverse bytes \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2];

    // Common pre-execution state

    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x1234_6789));

    // VM initialization

    // 0: REV <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "rev r0, r1"
    );
    vm_states[0].r[0] = Some(0x8967_3412);

    // 1: REV16 <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "rev16 r0, r1"
    );
    vm_states[1].r[0] = Some(0x3412_8967);

    // 2: REVSH <Rd>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "revsh r0, r1"
    );
    vm_states[2].r[0] = Some(0xFFFF_8967);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
