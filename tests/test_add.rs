extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration tests for Add operators

Included varieties:

ADDS <Rd>, <Rn>, #<imm3> T1     - Rd  <- Rn  + imm (+set all flags)
ADDS <Rdn>, #<imm8> T2          - Rdn <- Rdn + imm (+set all flags)
ADDS <Rd>, <Rn>, <Rm> T1        - Rd  <- Rn  + Rm (+set all flags)
ADD <Rdn>, <Rm> T2              - Rdn <- Rdn + Rm (one or both should be high register)
ADCS <Rdn>, <Rm> T1             - Rdn <- Rdn + Rm + Carry flag (+set all flags)
CMN  <Rn>, <Rm> T1              - _   <- Rn  + Rm (+set all flags)

TODO: ADD using PC and ADR op should reasonably be tested together with other branching ops

General test cases:

- Calculate sum of two registers
- Calculate sum of a register and an immediate value
- Set Negative flag when result is negative
- Set Zero flag when result is zero
- Set Carry flag when addition cause unsigned overflow
- Set V flag when addition cause signed overflow

Special test case for ADD <Rdn>, <Rm>:

- Calculate sum of two high registers + Preserve flags

(Behavior for ADCS + carry flag is implicitly tested in the common tests)

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "ADDS <Rd>, <Rn>, #<imm3> T1",
    "ADDS <Rdn>, #<imm8> T2",
    "ADDS <Rd>, <Rn>, <Rm> T1",
    "ADD <Rdn>, <Rm> T2",
    "ADCS <Rdn>, <Rm> T1",
    "CMN <Rn>, <Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &6;

// Calculate sum of two registers
#[test]
pub fn test_add_regadd() {
    println!("\n>>> Add ops test case: Calculate sum of two registers \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2, 3, 4];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0x0001_1111));
    common_state!(ops_to_test, vm_states.r[1] = Some(0x0010_3333));
    common_state!(ops_to_test, vm_states.r[2] = Some(0x0100_5555));
    common_state!(ops_to_test, vm_states.r[8] = Some(0x1000_7777));

    // VM initialization

    // 0: ADDS <Rd>, <Rn>, #<imm3> T1 - Not applicable

    // 1: ADDS <Rdn>, #<imm8> T2 - Not applicable

    // 2: ADDS <Rd>, <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 2, "adds r0, r1, r2");
    vm_states[2].r[0] = Some(0x0110_8888);

    // 3: ADD <Rdn>, <Rm> T2
    create_vm!(vms, vm_states, 3, "add  r0, r8");
    vm_states[3].r[0] = Some(0x1001_8888);

    // 4: ADCS <Rdn>, <Rm> T1
    create_vm!(vms, vm_states, 4, "adcs  r0, r1");
    vm_states[4].r[0] = Some(0x0011_4444);

    // 5: CMN <Rn>, <Rm> T1 - Not applicable

    run_test!(vms, vm_states, ops_to_test);
}

// Calculate sum of a register and an immediate value
#[test]
pub fn test_add_immadd() {
    println!(">>> Add ops test case: Calculate sum of a register and an immediate value \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0x0011_3333));
    common_state!(ops_to_test, vm_states.r[1] = Some(0x1100_5555));

    // VM initialization

    // 0: ADDS <Rd>, <Rn>, #<imm3> T1
    create_vm!(vms, vm_states, 0, "adds r0, r1, #0x07");
    vm_states[0].r[0] = Some(0x1100_555C);

    // 1: ADDS <Rdn>, #<imm8> T2
    create_vm!(vms, vm_states, 1, "adds r0, #0xFF");
    vm_states[1].r[0] = Some(0x0011_3432);

    // 2: ADDS <Rd>, <Rn>, <Rm> T1 - Not applicable

    // 3: ADD <Rdn>, <Rm> T2 - Not applicable

    // 4: ADCS <Rdn>, <Rm> T1 - Not applicable

    // 5: CMN <Rn>, <Rm> T1 - Not applicable

    run_test!(vms, vm_states, ops_to_test);
}

// Set Negative flag when result is negative + unset other flags
#[test]
pub fn test_add_flag_neg() {
    println!(
        ">>> Add ops test case: Set Negative flag when result is negative + unset other flags \n"
    );

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 4, 5];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0x8001_1111));
    common_state!(ops_to_test, vm_states.r[1] = Some(0x8010_3333));
    common_state!(ops_to_test, vm_states.r[2] = Some(0x0100_5555));

    common_state!(ops_to_test, vm_states.n = Some(false));
    common_state!(ops_to_test, vm_states.z = Some(true));
    common_state!(ops_to_test, vm_states.c = Some(true));
    common_state!(ops_to_test, vm_states.v = Some(true));

    // VM initialization

    // 0: ADDS <Rd>, <Rn>, #<imm3> T1
    create_vm!(vms, vm_states, 0, "adds r0, r1, #0x07");
    vm_states[0].r[0] = Some(0x8010_333A);

    // 1: ADDS <Rdn>, #<imm8> T2
    create_vm!(vms, vm_states, 1, "adds r0, #0xFF");
    vm_states[1].r[0] = Some(0x8001_1210);

    // 2: ADDS <Rd>, <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 2, "adds r0, r1, r2");
    vm_states[2].r[0] = Some(0x8110_8888);

    // 3: ADD <Rdn>, <Rm> T2 - Not applicable

    // 4: ADCS <Rdn>, <Rm> T1
    create_vm!(vms, vm_states, 4, "adcs  r0, r2"); // + 1 (Carry)
    vm_states[4].r[0] = Some(0x8101_6667);

    // 5: CMN <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 5, "cmn r0, r2");

    // Common expected post-execution state
    common_state!(ops_to_test, vm_states.n = Some(true));
    common_state!(ops_to_test, vm_states.z = Some(false));
    common_state!(ops_to_test, vm_states.c = Some(false));
    common_state!(ops_to_test, vm_states.v = Some(false));

    run_test!(vms, vm_states, ops_to_test);
}

// Set Zero flag when result is zero + unset other flags
#[test]
pub fn test_add_flag_zero() {
    println!(">>> Add ops test case: Set Zero flag when result is zero + unset other flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 4, 5];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0xFFFF_FF01));
    common_state!(ops_to_test, vm_states.r[1] = Some(0xFFFF_FFF9));
    common_state!(ops_to_test, vm_states.r[2] = Some(0x0000_0007));
    common_state!(ops_to_test, vm_states.r[3] = Some(0x0000_00FF));

    common_state!(ops_to_test, vm_states.n = Some(true));
    common_state!(ops_to_test, vm_states.z = Some(false));
    common_state!(ops_to_test, vm_states.c = Some(false)); // Add wrap around to 0 -> set overflow/carry
    common_state!(ops_to_test, vm_states.v = Some(true));

    // VM initialization

    // 0: ADDS <Rd>, <Rn>, #<imm3> T1
    create_vm!(vms, vm_states, 0, "adds r0, r1, #0x07");

    // 1: ADDS <Rdn>, #<imm8> T2
    create_vm!(vms, vm_states, 1, "adds r0, #0xFF");

    // 2: ADDS <Rd>, <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 2, "adds r0, r1, r2");

    // 3: ADD <Rdn>, <Rm> T2 - Not applicable

    // 4: ADCS <Rdn>, <Rm> T1
    create_vm!(vms, vm_states, 4, "adcs  r0, r3");

    // 5: CMN <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 5, "cmn r0, r3");

    // Common expected post-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0x00));
    vm_states[5].r[0] = None; // Op discards result anyway

    common_state!(ops_to_test, vm_states.n = Some(false));
    common_state!(ops_to_test, vm_states.z = Some(true));
    common_state!(ops_to_test, vm_states.c = Some(true));
    common_state!(ops_to_test, vm_states.v = Some(false));

    run_test!(vms, vm_states, ops_to_test);
}

// Set Carry flag when addition cause unsigned overflow + unset other flags
#[test]
pub fn test_add_flag_carry() {
    println!(">>> Add ops test case: Set Carry flag when addition cause unsigned overflow + unset other flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 4, 5];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0xFFFF_FFFF));
    common_state!(ops_to_test, vm_states.r[1] = Some(0xFFFF_FFFF));
    common_state!(ops_to_test, vm_states.r[2] = Some(0x06));

    common_state!(ops_to_test, vm_states.n = Some(true));
    common_state!(ops_to_test, vm_states.z = Some(true));
    common_state!(ops_to_test, vm_states.c = Some(false));
    common_state!(ops_to_test, vm_states.v = Some(true));

    // VM initialization

    // 0: ADDS <Rd>, <Rn>, #<imm3> T1
    create_vm!(vms, vm_states, 0, "adds r0, r1, #0x07");
    vm_states[0].r[0] = Some(0x06);

    // 1: ADDS <Rdn>, #<imm8> T2
    create_vm!(vms, vm_states, 1, "adds r0, #0xFF");
    vm_states[1].r[0] = Some(0xFE);

    // 2: ADDS <Rd>, <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 2, "adds r0, r1, r2");
    vm_states[2].r[0] = Some(0x05);

    // 3: ADD <Rdn>, <Rm> T2 - Not applicable

    // 4: ADCS <Rdn>, <Rm> T1
    create_vm!(vms, vm_states, 4, "adcs  r0, r2");
    vm_states[4].r[0] = Some(0x05);

    // 5: CMN <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 5, "cmn r0, r2");

    // Common expected post-execution state
    common_state!(ops_to_test, vm_states.n = Some(false));
    common_state!(ops_to_test, vm_states.z = Some(false));
    common_state!(ops_to_test, vm_states.c = Some(true));
    common_state!(ops_to_test, vm_states.v = Some(false));

    run_test!(vms, vm_states, ops_to_test);
}

// Set V flag when addition cause signed overflow + unset other flags
#[test]
pub fn test_add_flag_v() {
    println!(">>> Add ops test case: Set V flag when addition cause signed overflow + unset other flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 4, 5];

    // Common pre-execution state
    common_state!(ops_to_test, vm_states.r[0] = Some(0x7FFF_FFFF));
    common_state!(ops_to_test, vm_states.r[1] = Some(0x7FFF_FFFF));
    common_state!(ops_to_test, vm_states.r[2] = Some(0x06));

    common_state!(ops_to_test, vm_states.n = Some(false)); // Causing sign overflow with add -> negative number
    common_state!(ops_to_test, vm_states.z = Some(true));
    common_state!(ops_to_test, vm_states.c = Some(true));
    common_state!(ops_to_test, vm_states.v = Some(false));

    // VM initialization

    // 0: ADDS <Rd>, <Rn>, #<imm3> T1
    create_vm!(vms, vm_states, 0, "adds r0, r1, #0x07");
    vm_states[0].r[0] = Some(0x8000_0006);

    // 1: ADDS <Rdn>, #<imm8> T2
    create_vm!(vms, vm_states, 1, "adds r0, #0xFF");
    vm_states[1].r[0] = Some(0x8000_00FE);

    // 2: ADDS <Rd>, <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 2, "adds r0, r1, r2");
    vm_states[2].r[0] = Some(0x8000_0005);

    // 3: ADD <Rdn>, <Rm> T2 - Not applicable

    // 4: ADCS <Rdn>, <Rm> T1
    create_vm!(vms, vm_states, 4, "adcs  r0, r2"); // +1 (carry)
    vm_states[4].r[0] = Some(0x8000_0006);

    // 5: CMN <Rn>, <Rm> T1
    create_vm!(vms, vm_states, 5, "cmn r0, r2");

    // Common expected post-execution state
    common_state!(ops_to_test, vm_states.n = Some(true)); // Causing sign overflow with add -> negative number
    common_state!(ops_to_test, vm_states.z = Some(false));
    common_state!(ops_to_test, vm_states.c = Some(false));
    common_state!(ops_to_test, vm_states.v = Some(true));

    run_test!(vms, vm_states, ops_to_test);
}

// ADD <Rdn>, <Rm>: Calculate sum of two high registers + Preserve flags
#[test]
pub fn test_add_high_noflags() {
    println!("\n>>> ADDS <Rd>, <Rn>, <Rm> op special test case: Calculate sum of two high registers + Preserve flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![3];

    // VM initialization

    // 3: ADD <Rdn>, <Rm> T2
    vm_states[3].r[8] = Some(0x0011_3333);
    vm_states[3].r[9] = Some(0x1100_5555);

    vm_states[3].n = Some(true);
    vm_states[3].z = Some(true);
    vm_states[3].c = Some(true);
    vm_states[3].v = Some(true);

    create_vm!(vms, vm_states, 3, "add r8, r9");
    vm_states[3].r[8] = Some(0x1111_8888);

    run_test!(vms, vm_states, ops_to_test);
}
