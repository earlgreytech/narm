extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for bit shift operators

Note: When registers are the source of shift amount, only the bottom byte is read

Included varieties:

LSLS <Rd>,<Rm>,#<imm5> T1   - Shift left by imm, fill with zeroes
LSRS <Rd>,<Rm>,#<imm5> T1   - Shift right by imm, fill with zeroes
ASRS <Rd>,<Rm>,#<imm5> T1   - Shift right by imm, fill with copies of sign bit
LSLS <Rdn>,<Rm> T1          - Shift left by value in Rm, fill with zeroes
LSRS <Rdn>,<Rm> T1          - Shift right by value in Rm, fill with zeroes
ASRS <Rdn>,<Rm> T1          - Shift right by value in Rm, fill with copies of sign bit
RORS <Rdn>,<Rm> T1          - Shift right by value in Rm, fill with out-shifted bits.

General test cases:

- Shift register
- Shift register a lot
- Set Negative flag when result is negative
- Set Zero flag when result is zero
- Set Carry flag to last bit shifted out

Special test case for both ASRS:

- Fill with 1 instead of 0 if sign bit is 1

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "LSLS <Rd>,<Rm>,#<imm5> T1",
    "LSRS <Rd>,<Rm>,#<imm5> T1",
    "ASRS <Rd>,<Rm>,#<imm5> T1",
    "LSLS <Rdn>,<Rm> T1",
    "LSRS <Rdn>,<Rm> T1",
    "ASRS <Rdn>,<Rm> T1",
    "RORS <Rdn>,<Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &7;

// Shift register
#[test]
pub fn test_bitshift_smallshift() {
    println!("\n>>> Bit shift ops test case: Shift register \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0001_0000));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x0001_0000));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x02));

    // VM initialization

    // 0: LSLS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "lsls r0, r1, #0x02"
    );
    vm_states[0].r[0] = Some(0x0004_0000);

    // 1: LSRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "lsrs r0, r1, #0x02"
    );
    vm_states[1].r[0] = Some(0x0000_4000);

    // 2: ASRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "asrs r0, r1, #0x02"
    );
    vm_states[2].r[0] = Some(0x0000_4000);

    // 3: LSLS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "lsls r0, r2"
    );
    vm_states[3].r[0] = Some(0x0004_0000);

    // 4: LSRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "lsrs r0, r2"
    );
    vm_states[4].r[0] = Some(0x0000_4000);

    // 5: ASRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "asrs r0, r2"
    );
    vm_states[5].r[0] = Some(0x0000_4000);

    // 6: RORS <Rdn>,<Rm> T1
    vm_states[6].r[0] = Some(0x0001_0001);
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "rors r0, r2"
    );
    vm_states[6].r[0] = Some(0x4000_4000);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Shift register a lot
#[test]
pub fn test_bitshift_bigshift() {
    println!("\n>>> Bit shift ops test case: Shift register a lot \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x4000_0002));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x4000_0002));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x1C));

    // VM initialization

    // 0: LSLS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "lsls r0, r1, #0x1C"
    );
    vm_states[0].r[0] = Some(0x2000_0000);

    // 1: LSRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "lsrs r0, r1, #0x1C"
    );
    vm_states[1].r[0] = Some(0x0000_0004);

    // 2: ASRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "asrs r0, r1, #0x1C"
    );
    vm_states[2].r[0] = Some(0x0000_0004);

    // 3: LSLS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "lsls r0, r2"
    );
    vm_states[3].r[0] = Some(0x2000_0000);

    // 4: LSRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "lsrs r0, r2"
    );
    vm_states[4].r[0] = Some(0x0000_0004);

    // 5: ASRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "asrs r0, r2"
    );
    vm_states[5].r[0] = Some(0x0000_0004);

    // 6: RORS <Rdn>,<Rm> T1
    vm_states[6].r[0] = Some(0x8000_8000);
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "rors r0, r2"
    );
    vm_states[6].r[0] = Some(0x0008_0008);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set Negative flag when result is negative
#[test]
pub fn test_bitshift_flag_neg() {
    println!("\n>>> Bit shift ops test case: Negative flag when result is negative \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x8000_0001));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x8000_0001));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x1F));
    set_for_all!(vm_states[ops_to_test].r[3] = Some(0x00));
    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = None); // Semi-pointless to check here because shift by 0 doesn't alter carry flag

    // VM initialization

    // 0: LSLS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "lsls r0, r1, #0x1F"
    );
    vm_states[0].r[0] = Some(0x8000_0000);

    // 1: LSRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "lsrs r0, r1, #0x00"
    );

    // 2: ASRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "asrs r0, r1, #0x00"
    );

    // 3: LSLS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "lsls r0, r2"
    );
    vm_states[3].r[0] = Some(0x8000_0000);

    // 4: LSRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "lsrs r0, r3"
    );

    // 5: ASRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "asrs r0, r3"
    );

    // 6: RORS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "rors r0, r3"
    );

    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(false));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set Zero flag when result is zero
#[test]
pub fn test_bitshift_flag_zero() {
    println!("\n>>> Bit shift ops test case: Set Zero flag when result is zero \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0001_8000));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x0001_8000));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x12));
    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(false));
    set_for_all!(vm_states[ops_to_test].c = Some(true));

    // VM initialization

    // 0: LSLS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "lsls r0, r1, #0x12"
    );

    // 1: LSRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "lsrs r0, r1, #0x12"
    );

    // 2: ASRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "asrs r0, r1, #0x12"
    );

    // 3: LSLS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "lsls r0, r2"
    );

    // 4: LSRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "lsrs r0, r2"
    );

    // 5: ASRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "asrs r0, r2"
    );

    // 6: RORS <Rdn>,<Rm> T1
    vm_states[6].r[0] = Some(0x00);
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "rors r0, r2"
    );

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x00));
    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = Some(false));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set Carry flag to last bit shifted out
#[test]
pub fn test_bitshift_flag_carry() {
    println!("\n>>> Bit shift ops test case: Set Carry flag to last bit shifted out \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x5000_000A));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x5000_000A));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x02));
    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = Some(false));

    // VM initialization

    // 0: LSLS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "lsls r0, r1, #0x02"
    );
    vm_states[0].r[0] = Some(0x4000_0028);

    // 1: LSRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "lsrs r0, r1, #0x02"
    );
    vm_states[1].r[0] = Some(0x1400_0002);

    // 2: ASRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "asrs r0, r1, #0x02"
    );
    vm_states[2].r[0] = Some(0x1400_0002);

    // 3: LSLS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "lsls r0, r2"
    );
    vm_states[3].r[0] = Some(0x4000_0028);

    // 4: LSRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "lsrs r0, r2"
    );
    vm_states[4].r[0] = Some(0x1400_0002);

    // 5: ASRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "asrs r0, r2"
    );
    vm_states[5].r[0] = Some(0x1400_0002);

    // 6: RORS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "rors r0, r2"
    );
    vm_states[6].r[0] = Some(0x9400_0002);

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(false));
    set_for_all!(vm_states[ops_to_test].c = Some(true));

    vm_states[6].n = Some(true); // Since we're shifting out a 1 as last bit, the sign bit will be 1

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Fill with 1 instead of 0 if sign bit is 1
#[test]
pub fn test_bitshift_asrs_signfill() {
    println!("\n>>> ASRS special test case: Fill with 1 instead of 0 if sign bit is 1 \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2, 5];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x8001_0000));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x8001_0000));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x10));

    // VM initialization

    // 2: ASRS <Rd>,<Rm>,#<imm5> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "asrs r0, r1, #0x10"
    );

    // 5: ASRS <Rdn>,<Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "asrs r0, r2"
    );

    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFFFF_8001));
    set_for_all!(vm_states[ops_to_test].n = Some(true));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
