extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration tests for Sub operators

Note that ARM uses inverted carry flag for subtractions, so c flag is always set *unless* there is a signed overflow
This is a computational simplification related to 2-complement representation that allows it to use the exact same arithmetics as addition

Included varieties:

SUBS <Rd>, <Rn>, #<imm3> T1     - Rd  <- Rn  - imm (+set all flags)
SUBS <Rdn>, #<imm8> T2          - Rdn <- Rdn - imm (+set all flags)
SUBS <Rd>, <Rn>, <Rm> T1        - Rd  <- Rn  - Rm (+set all flags)
SBCS <Rdn>, <Rm> T1             - Rdn <- Rdn - Rm + Carry flag (+set all flags)
RSBS <Rd>, <Rn>, #<imm0> T1     - Rd  <- 0   - Rn (+set all flags) (Negation)
CMP <Rn>, #<imm8> T1            - _   <- Rn  - imm (+set all flags)
CMP <Rn>, <Rm> T1               - _   <- Rn  - Rm (+set all flags)

General test cases:

- Calculate difference of two registers
- Calculate difference of a register and an immediate value
- Set Negative flag when result is negative
- Set Zero flag when result is zero
- Clear Carry flag when subtraction cause unsigned overflow
- Set V flag when subtraction cause signed overflow

(Behavior for SBCS + carry flag is implicitly tested in the common tests)

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "SUBS <Rd>, <Rn>, #<imm3> T1",
    "SUBS <Rdn>, #<imm8> T2",
    "SUBS <Rd>, <Rn>, <Rm> T1",
    "SBCS <Rdn>, <Rm> T1",
    "RSBS <Rd>, <Rn>, #<imm0> T1",
    "CMP <Rn>, #<imm8> T1",
    "CMP <Rn>, <Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &7;

// Calculate difference of two registers
#[test]
pub fn test_sub_regsub() {
    println!("\n>>> Sub ops test case: Calculate difference of two registers \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2, 3];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0111_3333));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x0111_5555));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x0010_1111));

    // VM initialization

    // 0: SUBS <Rd>, <Rn>, #<imm3> T1 - Not applicable

    // 1: SUBS <Rdn>, #<imm8> T2 - Not applicable

    // 2: SUBS <Rd>, <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "subs r0, r1, r2"
    );
    vm_states[2].r[0] = Some(0x0101_4444);

    // 3: SBCS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "sbcs r0, r2"
    ); // - 1 (NOT carry)
    vm_states[3].r[0] = Some(0x0101_2221);

    // 4: RSBS <Rd>, <Rn>, #<imm0> T1 - Not applicable

    // 5: CMP <Rn>, #<imm8> T1 - Not applicable

    // 6: CMP <Rn>, <Rm> T1 - Not applicable

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].c = Some(true)); // Set *unless* there is unsigned overflow

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Calculate sum of a register and an immediate value
#[test]
pub fn test_sub_immsub() {
    println!(">>> Sub ops test case: Calculate difference of a register and an immediate value \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x0101_3333));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x0010_5555));

    // VM initialization

    // 0: SUBS <Rd>, <Rn>, #<imm3> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "subs r0, r1, #0x07"
    );
    vm_states[0].r[0] = Some(0x0010_554E);

    // 1: SUBS <Rdn>, #<imm8> T2
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "subs r0, #0xFF"
    );
    vm_states[1].r[0] = Some(0x0101_3234);

    // 2: SUBS <Rd>, <Rn>, <Rm> T1 - Not applicable

    // 3: SBCS <Rdn>, <Rm> T1 - Not applicable

    // 4: RSBS <Rd>, <Rn>, #<imm0> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "rsbs r0, r1, #0x00"
    ); // Actually reverse: imm - reg
    vm_states[4].r[0] = Some(0xFFEF_AAAB);
    vm_states[4].n = Some(true);

    // 5: CMP <Rn>, #<imm8> T1 - Not applicable

    // 6: CMP <Rn>, <Rm> T1 - Not applicable

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].c = Some(true)); // Set *unless* there is unsigned overflow

    vm_states[4].c = Some(false); // Ugly, but here we are. Reverse subtract with its 0 - (>0) will always "set" carry

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set Negative flag when result is negative + unset other flags
#[test]
pub fn test_sub_flag_neg() {
    println!(
        ">>> Sub ops test case: Set Negative flag when result is negative + unset other flags \n"
    );

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x8642_3333));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x8642_3333));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x06));

    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = Some(false));
    set_for_all!(vm_states[ops_to_test].v = Some(true));

    // VM initialization

    // 0: SUBS <Rd>, <Rn>, #<imm3> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "subs r0, r1, #0x07"
    );
    vm_states[0].r[0] = Some(0x8642_332C);

    // 1: SUBS <Rdn>, #<imm8> T2
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "subs r0, #0xFF"
    );
    vm_states[1].r[0] = Some(0x8642_3234);

    // 2: SUBS <Rd>, <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "subs r0, r1, r2"
    );
    vm_states[2].r[0] = Some(0x8642_332D);

    // 3: SBCS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "sbcs r0, r2"
    ); // - 1 (NOT carry)
    vm_states[3].r[0] = Some(0x8642_332C);

    // 4: RSBS <Rd>, <Rn>, #<imm0> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "rsbs r0, r2, #0x00"
    );
    vm_states[4].r[0] = Some(0xFFFF_FFFA);

    // 5: CMP <Rn>, #<imm8> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "cmp r0, #0xFF"
    );

    // 6: CMP <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "cmp r0, r2"
    );

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(false));
    set_for_all!(vm_states[ops_to_test].c = Some(true)); // Set *unless* there is unsigned overflow
    set_for_all!(vm_states[ops_to_test].v = Some(false));

    vm_states[4].c = Some(false); // Ugly, but here we are. Reverse subtract with its 0 - (>0) will always "set" carry

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set Zero flag when result is zero + unset other flags
#[test]
pub fn test_sub_flag_zero() {
    println!(">>> Sub ops test case: Set Zero flag when result is zero + unset other flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFF));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x07));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x07));
    set_for_all!(vm_states[ops_to_test].r[3] = Some(0xFE)); // 0xFF - 1 because SBCS will subtract 1 more

    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(false));
    set_for_all!(vm_states[ops_to_test].c = Some(false));
    set_for_all!(vm_states[ops_to_test].v = Some(true));

    // VM initialization

    // 0: SUBS <Rd>, <Rn>, #<imm3> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "subs r0, r1, #0x07"
    );

    // 1: SUBS <Rdn>, #<imm8> T2
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "subs r0, #0xFF"
    );

    // 2: SUBS <Rd>, <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "subs r0, r1, r2"
    );

    // 3: SBCS <Rdn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "sbcs r0, r3"
    ); // - 1 (NOT carry)

    // 4: RSBS <Rd>, <Rn>, #<imm0> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "rsbs r0, r4, #0x00"
    );

    // 5: CMP <Rn>, #<imm8> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "cmp r0, #0xFF"
    );

    // 6: CMP <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "cmp r1, r2"
    );

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x00));
    vm_states[5].r[0] = None; // Op discards result anyway
    vm_states[6].r[0] = None; // Op discards result anyway

    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = Some(true)); // Set *unless* there is unsigned overflow
    set_for_all!(vm_states[ops_to_test].v = Some(false));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Unset Carry flag when subtraction cause unsigned overflow + unset other flags
// Pretty much same as above but substract 1 more so it pushes from 0 -> -1
#[test]
pub fn test_sub_flag_carry() {
    println!(">>> Sub ops test case: Unset Carry flag when subtraction cause unsigned overflow + unset other flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFE));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x06));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x07));
    set_for_all!(vm_states[ops_to_test].r[3] = Some(0xFF));
    set_for_all!(vm_states[ops_to_test].r[4] = Some(0x01));

    set_for_all!(vm_states[ops_to_test].n = Some(false)); // Result will be negative
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = Some(true));
    set_for_all!(vm_states[ops_to_test].v = Some(true));

    // VM initialization

    // 0: SUBS <Rd>, <Rn>, #<imm3> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "subs r0, r1, #0x07"
    );

    // 1: SUBS <Rdn>, #<imm8> T2
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "subs r0, #0xFF"
    );

    // 2: SUBS <Rd>, <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "subs r0, r1, r2"
    );

    // 3: SBCS <Rdn>, <Rm> T1
    vm_states[3].r[0] = Some(0x00); // Also test proper behavior if carry causes overflow
    vm_states[3].c = Some(false);
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "sbcs r0, r3"
    );

    // 4: RSBS <Rd>, <Rn>, #<imm0> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "rsbs r0, r4, #0x00"
    );

    // 5: CMP <Rn>, #<imm8> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "cmp r0, #0xFF"
    );

    // 6: CMP <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "cmp r0, r3"
    );

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xFFFF_FFFF)); // = -1
    vm_states[3].r[0] = Some(0xFFFF_FF00);
    vm_states[5].r[0] = None; // Op discards result anyway
    vm_states[6].r[0] = None; // Op discards result anyway

    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(false));
    set_for_all!(vm_states[ops_to_test].c = Some(false)); // Set *unless* there is unsigned overflow
    set_for_all!(vm_states[ops_to_test].v = Some(false));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Set V flag when subtraction cause signed overflow + unset other flags
// Pretty much same as above but with highest bit set so it overflows signed instead of unsigned
#[test]
pub fn test_sub_flag_v() {
    println!(">>> Sub ops test case: Set V flag when subtraction cause signed overflow + unset other flags \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 5, 6];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x8000_00FE));
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x8000_0006));
    set_for_all!(vm_states[ops_to_test].r[2] = Some(0x07));
    set_for_all!(vm_states[ops_to_test].r[3] = Some(0xFE)); // 0xFF - 1 because SBCS will subtract 1 more

    set_for_all!(vm_states[ops_to_test].n = Some(true));
    set_for_all!(vm_states[ops_to_test].z = Some(true));
    set_for_all!(vm_states[ops_to_test].c = Some(false));
    set_for_all!(vm_states[ops_to_test].v = Some(false));

    // VM initialization

    // 0: SUBS <Rd>, <Rn>, #<imm3> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 0,
        asm_literal_add_svc = "subs r0, r1, #0x07"
    );

    // 1: SUBS <Rdn>, #<imm8> T2
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 1,
        asm_literal_add_svc = "subs r0, #0xFF"
    );

    // 2: SUBS <Rd>, <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "subs r0, r1, r2"
    );

    // 3: SBCS <Rdn>, <Rm> T1
    vm_states[3].r[0] = Some(0x8000_0000); // Also test proper behavior if carry causes overflow
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "sbcs r0, r3"
    ); // - 1 (NOT carry)

    // 4: RSBS <Rd>, <Rn>, #<imm0> T1 - Not applicable
    // It feels like, say 0 - 0x8000_000F *should* cause signed overflow, but doesn't due to how it's handled internally
    // ( -> 0 + bitwise_neg(0x8000_000F) = 0 + 7FFF_FFF0 -> no signed overflow

    // 5: CMP <Rn>, #<imm8> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 5,
        asm_literal_add_svc = "cmp r0, #0xFF"
    );

    // 6: CMP <Rn>, <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 6,
        asm_literal_add_svc = "cmp r1, r2"
    );

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0x7FFF_FFFF));
    vm_states[3].r[0] = Some(0x7FFF_FF01);
    vm_states[5].r[0] = None; // Op discards result anyway
    vm_states[6].r[0] = None; // Op discards result anyway

    set_for_all!(vm_states[ops_to_test].n = Some(false));
    set_for_all!(vm_states[ops_to_test].z = Some(false));
    set_for_all!(vm_states[ops_to_test].c = Some(true)); // Set *unless* there is unsigned overflow
    set_for_all!(vm_states[ops_to_test].v = Some(true));

    vm_states[5].n = None; // Op discards result anyway
    vm_states[6].n = None; // Op discards result anyway

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
