extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Mul operators

Note that result bits that don't fit in 32 bits (overflow) are simply discarded.

Included varieties:

MULS <Rdm>, <Rn>, <Rdm> T1          Rdm <- Rn * Rdm (+set N and Z flags)

General test cases:

- Calculate product that fits in 32 bits
- Calculate product that overflow 32 bits
- Set Negative flag when result is negative
- Set Zero flag when result is zero

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &["MULS <Rdm>, <Rn>, <Rdm> T1"];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &1;

// Calculate product that fits in 32 bits
#[test]
pub fn test_mul_inside_32() {
    println!("\n>>> Mul op test case: Calculate product that fits in 32 bits \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0];

    vm_states[0].r[0] = Some(0x0000_1234);
    vm_states[0].r[1] = Some(0x0003_0002);

    // VM initialization

    // 0: MULS <Rdm>, <Rn>, <Rdm> T1
    create_vm!(vms, vm_states, 0, "muls r0, r1, r0");
    vm_states[0].r[0] = Some(0x369C_2468);

    run_test!(vms, vm_states, ops_to_test);
}

// Calculate product that overflow 32 bits
#[test]
pub fn test_mul_overflow_32() {
    println!("\n>>> Mul op test case: Calculate product that overflow 32 bits \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0];

    vm_states[0].r[0] = Some(0x1111_1234);
    vm_states[0].r[1] = Some(0x0003_0002);

    // VM initialization

    // 0: MULS <Rdm>, <Rn>, <Rdm> T1
    create_vm!(vms, vm_states, 0, "muls r0, r1, r0");
    vm_states[0].r[0] = Some(0x58BE_2468);

    run_test!(vms, vm_states, ops_to_test);
}

// Set Negative flag when result is negative
#[test]
pub fn test_mul_flag_neg() {
    println!("\n>>> Mul op test case: Set Negative flag when result is negative \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0];

    vm_states[0].r[0] = Some(0x0000_1234);
    vm_states[0].r[1] = Some(0x0008_0002);

    vm_states[0].n = Some(false);
    vm_states[0].z = Some(true);
    vm_states[0].c = Some(true); // Shouldn't be affected at all
    vm_states[0].v = Some(true); // Shouldn't be affected at all

    // VM initialization

    // 0: MULS <Rdm>, <Rn>, <Rdm> T1
    create_vm!(vms, vm_states, 0, "muls r0, r1, r0");
    vm_states[0].r[0] = Some(0x91A0_2468);
    vm_states[0].n = Some(true);
    vm_states[0].z = Some(false);

    run_test!(vms, vm_states, ops_to_test);
}

// Set Zero flag when result is zero
#[test]
pub fn test_mul_flag_zero() {
    println!("\n>>> Mul op test case: Set Zero flag when result is zero \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0];

    vm_states[0].r[0] = Some(0x0001_0000);

    vm_states[0].n = Some(true);
    vm_states[0].z = Some(false);
    vm_states[0].c = Some(true); // Shouldn't be affected at all
    vm_states[0].v = Some(true); // Shouldn't be affected at all

    // VM initialization

    // 0: MULS <Rdm>, <Rn>, <Rdm> T1
    create_vm!(vms, vm_states, 0, "muls r0, r0, r0");
    vm_states[0].r[0] = Some(0x00);
    vm_states[0].n = Some(false);
    vm_states[0].z = Some(true);

    run_test!(vms, vm_states, ops_to_test);
}
