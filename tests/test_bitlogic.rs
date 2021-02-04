extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration tests for bitwise logic operators

Note that these operatiors shouldn't affect the V or C flag

Included varieties:

ANDS <Rdn>, <Rm> T1         - <Rdn> <- <Rdn> & <Rm>
TST <Rn>, <Rm> T1           - _     <- <Rn>  & <Rm>
BICS <Rdn>, <Rm> T1         - <Rdn> <- <Rdn> & !(<Rm>)
ORRS <Rdn>, <Rm> T1         - <Rdn> <- <Rdn> | <Rm>
EORS <Rdn>, <Rm> T1         - <Rdn> <- <Rdn> ^ <Rm>

Included cases:

- Calculate result for two register values
- Set Negative flag when result is negative
- Set Zero flag when result is zero

All tests also check for unexpected changes in registers and condition flags

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "ANDS <Rdn>, <Rm> T1",
    "TST <Rn>, <Rm> T1",
    "BICS <Rdn>, <Rm> T1",
    "ORRS <Rdn>, <Rm> T1",
    "EORS <Rdn>, <Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &5;

// Calculate result for two register values
#[test]
pub fn test_bitlogic_result() {
    println!("\n>>> Bitwise logic ops test case: Calculate result for two register values \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0000_1111));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x0101_0101));

    // VM initialization

    // 0: ANDS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ands r0, r1"
    );
    vm_states[0].r[0] = Some(0x0000_0101);

    // 1: TST <Rn>, <Rm> T1 - Not applicable

    // 2: BICS <Rdn>, <Rm> T1"
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "bics r0, r1"
    );
    vm_states[2].r[0] = Some(0x0000_1010);

    // 3: ORRS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "orrs r0, r1"
    );
    vm_states[3].r[0] = Some(0x0101_1111);

    // 4: EORS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "eors r0, r1"
    );
    vm_states[4].r[0] = Some(0x0101_1010);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set Negative flag when result is negative
#[test]
pub fn test_bitlogic_flag_neg() {
    println!("\n>>> Bitwise logic ops test case: Set Negative flag when result is negative \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x8001_0000));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x8010_0000));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x0100_0000));
    set_for_all!(vm_states[ops_to_test].r[3] = Some(0xFFFF_FFFF));

    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = Some(true)); // Shouldn't be affected at all
    set_for_all!(vm_states[ops_to_test].v = Some(true)); // Shouldn't be affected at all

    // VM initialization

    // 0: ANDS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ands r0, r1"
    );
    vm_states[0].r[0] = Some(0x8000_0000);

    // 1: TST <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "tst r0, r1"
    );

    // 2: BICS <Rdn>, <Rm> T1"
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "bics r0, r2"
    );
    vm_states[2].r[0] = Some(0x8001_0000);

    // 3: ORRS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "orrs r0, r1"
    );
    vm_states[3].r[0] = Some(0x8011_0000);

    // 4: EORS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "eors r0, r2"
    );
    vm_states[4].r[0] = Some(0x8101_0000);

    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(false));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set Zero flag when result is zero
#[test]
pub fn test_bitlogic_flag_zero() {
    println!("\n>>> Bitwise logic ops test case: Set Zero flag when result is zero \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x1010_1010));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x0101_0101));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x00));

    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(false));
    set_for_all!(vm_states[ops_to_test].c = Some(true)); // Shouldn't be affected at all
    set_for_all!(vm_states[ops_to_test].v = Some(true)); // Shouldn't be affected at all

    // VM initialization

    // 0: ANDS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "ands r0, r1"
    );
    vm_states[0].r[0] = Some(0x00);

    // 1: TST <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "tst r0, r1"
    );

    // 2: BICS <Rdn>, <Rm> T1"
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "bics r0, r0"
    );
    vm_states[2].r[0] = Some(0x00);

    // 3: ORRS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "orrs r2, r2"
    );

    // 4: EORS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "eors r0, r0"
    );
    vm_states[4].r[0] = Some(0x00);

    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(true));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
