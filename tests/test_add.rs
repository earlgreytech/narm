extern crate narm;
mod common;

use common::*;

/*

Unit test for Add operators

Included varieties:

ADDS <Rd>, <Rn>, #<imm3>    (smallimm)      - Rd  <- Rn  + imm
ADDS <Rdn>, #<imm8>         (bigimm)        - Rdn <- Rdn + imm
ADDS <Rd>, <Rn>, <Rm>       (3reg)          - Rd  <- Rn  + Rm
ADD <Rdn>, <Rm>             (high)          - Rdn <- Rdn + Rm (one or both should be high register)
ADCS <Rdn>, <Rm>            (carry)         - Rdn <- Rdn + Rm + Carry flag

General test cases:

- Calculate sum of two registers
- Calculate sum of a register and an immediate value
- Set Negative flag when result is negative (highest bit set)
- Set Zero flag when result is zero
- Set Carry flag when addition cause unsigned overflow
- Set V flag when addition cause signed overflow

Special test case for ADD <Rdn>, <Rm>:

- Calculate sum of two high registers + Preserve flags

Special test case for ADCS <Rdn>, <Rm>

- Add with carry flag set

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Calculate sum of two registers
#[test]
pub fn test_add_regadd() {
    println!("\n>>> Add ops test case: Calculate sum of two registers \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0x06);
    vm_state.r[1] = Some(0x06);
    vm_state.r[2] = Some(0x0F);
    vm_state.r[8] = Some(0x0F);

    println!("\n>>> Creating VM(s) \n");

    // ADDS <Rd>, <Rn>, #<imm3> - Not applicable

    // ADDS <Rdn>, #<imm8> - Not applicable

    // ADDS <Rd>, <Rn>, <Rm>
    let mut vm_3reg = create_vm_from_asm(
        "
        adds r0, r1, r2
        svc                 #0xFF
    ",
    );

    // ADD <Rdn>, <Rm>
    let mut vm_high = create_vm_from_asm(
        "
        add  r0, r8
        svc                 #0xFF
    ",
    );
    
    // ADCS <Rdn>, <Rm>
    let mut vm_carry = create_vm_from_asm(
        "
        adcs  r0, r2
        svc                 #0xFF
    ",
    );

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_3reg);
    load_into_vm!(vm_state, vm_high);
    load_into_vm!(vm_state, vm_carry);

    // Expected state diff after execution
    vm_state.r[0] = Some(0x15);

    println!("\n>>> [1/3] Testing for op variant: ADDS <Rd>, <Rn>, <Rm> \n");
    execute_and_assert!(vm_state, vm_3reg);

    println!("\n>>> [2/3] Testing for op variant: ADD <Rdn>, <Rm> \n");
    execute_and_assert!(vm_state, vm_high);
    
    println!("\n>>> [3/3] Testing for op variant: ADCS <Rdn>, <Rm> \n");
    execute_and_assert!(vm_state, vm_carry);
}

// Calculate sum of a register and an immediate value
#[test]
pub fn test_add_immadd() {
    println!(">>> Add ops test case: Calculate sum of a register and an immediate value \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0x07);
    vm_state.r[1] = Some(0xFF);

    // ADDS <Rd>, <Rn>, #<imm3>
    let mut vm_smallimm = create_vm_from_asm(
        "
        adds r0, r1,        #0x07
        svc                 #0xFF
    ",
    );

    // ADDS <Rdn>, #<imm8>
    let mut vm_bigimm = create_vm_from_asm(
        "
        adds r0,            #0xFF
        svc                 #0xFF
    ",
    );

    // ADDS <Rd>, <Rn>, <Rm> - Not applicable

    // ADD <Rdn>, <Rm> - Not applicable

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_smallimm);
    load_into_vm!(vm_state, vm_bigimm);

    // Expected state diff after execution
    vm_state.r[0] = Some(0x0106);

    println!(">>> [1/2] Testing for op variant: ADDS <Rd>, <Rn>, #<imm3> ");
    execute_and_assert!(vm_state, vm_smallimm);

    println!(">>> [2/2] Testing for op variant: ADDS <Rdn>, #<imm8> ");
    execute_and_assert!(vm_state, vm_bigimm);
}

// Set Negative flag when result is negative (highest bit set)
#[test]
pub fn test_add_flag_neg() {
    println!(
        ">>> Add ops test case: Set Negative flag when result is negative (highest bit set) \n"
    );

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0x8000_0000);
    vm_state.r[1] = Some(0x8000_00F8);
    vm_state.r[2] = Some(0x07);
    vm_state.r[3] = Some(0xFE);
    vm_state.n = Some(false);
    vm_state.z = Some(true);
    vm_state.c = Some(true);
    vm_state.v = Some(true);

    // ADDS <Rd>, <Rn>, #<imm3>
    let mut vm_smallimm = create_vm_from_asm(
        "
        adds r0, r1,        #0x7
        svc                 #0xFF
    ",
    );

    // ADDS <Rdn>, #<imm8>
    let mut vm_bigimm = create_vm_from_asm(
        "
        adds r0,            #0xFF
        svc                 #0xFF
    ",
    );

    // ADDS <Rd>, <Rn>, <Rm>
    let mut vm_3reg = create_vm_from_asm(
        "
        adds r0, r1, r2
        svc                 #0xFF
    ",
    );

    // ADD <Rdn>, <Rm> - Not applicable
    
    // ADCS <Rdn>, <Rm>
    let mut vm_carry = create_vm_from_asm(
        "
        adcs r0, r3
        svc                 #0xFF
    ",
    );

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_smallimm);
    load_into_vm!(vm_state, vm_bigimm);
    load_into_vm!(vm_state, vm_3reg);
    load_into_vm!(vm_state, vm_carry);

    // Expected state diff after execution
    vm_state.r[0] = Some(0x8000_00FF);
    vm_state.n = Some(true);
    vm_state.z = Some(false);
    vm_state.c = Some(false);
    vm_state.v = Some(false);

    println!(">>> [1/4] Testing for op variant: ADDS <Rd>, <Rn>, #<imm3> ");
    execute_and_assert!(vm_state, vm_smallimm);

    println!(">>> [2/4] Testing for op variant: ADDS <Rdn>, #<imm8> ");
    execute_and_assert!(vm_state, vm_bigimm);

    println!(">>> [3/4] Testing for op variant: ADDS <Rd>, <Rn>, <Rm> ");
    execute_and_assert!(vm_state, vm_3reg);
    
    println!(">>> [4/4] Testing for op variant: ADCD <Rdn>, <Rm> ");
    execute_and_assert!(vm_state, vm_carry);
}

// Set Zero flag when result is zero
#[test]
pub fn test_add_flag_zero() {
    println!(">>> Add ops test case: Set Zero flag when result is zero \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0xFFFF_FF01); // -0xFF
    vm_state.r[1] = Some(0xFFFF_FFF9); // -0xF8
    vm_state.r[2] = Some(0x07);
    vm_state.r[3] = Some(0xFF);
    vm_state.n = Some(true);
    vm_state.z = Some(false);
    vm_state.c = Some(false);
    vm_state.v = Some(true);

    // ADDS <Rd>, <Rn>, #<imm3>
    let mut vm_smallimm = create_vm_from_asm(
        "
        adds r0, r1,        #0x07
        svc                 #0xFF
    ",
    );

    // ADDS <Rdn>, #<imm8>
    let mut vm_bigimm = create_vm_from_asm(
        "
        adds r0,            #0xFF
        svc                 #0xFF
    ",
    );

    // ADDS <Rd>, <Rn>, <Rm>
    let mut vm_3reg = create_vm_from_asm(
        "
        adds r0, r1, r2
        svc                 #0xFF
    ",
    );
    
    // ADCS <Rdn>, <Rm>
    let mut vm_carry = create_vm_from_asm(
        "
        adcs r0, r3
        svc                 #0xFF
    ",
    );

    // ADD <Rdn>, <Rm> - Not applicable

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_smallimm);
    load_into_vm!(vm_state, vm_bigimm);
    load_into_vm!(vm_state, vm_3reg);
    load_into_vm!(vm_state, vm_carry);

    // Expected state diff after execution
    vm_state.r[0] = Some(0x00);
    vm_state.n = Some(false);
    vm_state.z = Some(true);
    vm_state.c = Some(true);
    vm_state.v = Some(false);

    println!(">>> [1/3] Testing for op variant: ADDS <Rd>, <Rn>, #<imm3> ");
    execute_and_assert!(vm_state, vm_smallimm);

    println!(">>> [2/3] Testing for op variant: ADDS <Rdn>, #<imm8> ");
    execute_and_assert!(vm_state, vm_bigimm);

    println!(">>> [3/3] Testing for op variant: ADDS <Rd>, <Rn>, <Rm> ");
    execute_and_assert!(vm_state, vm_3reg);
    
    println!(">>> [3/3] Testing for op variant: ADCS <Rdn>, <Rm> ");
    execute_and_assert!(vm_state, vm_carry);
}

// Set Carry flag when addition cause unsigned overflow
#[test]
pub fn test_add_flag_carry() {
    println!(">>> Add ops test case: Set Carry flag when addition cause unsigned overflow \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0xFFFF_FFFF);
    vm_state.r[1] = Some(0xFF);
    vm_state.r[2] = Some(0xFFFF_FFFF);
    vm_state.n = Some(true);
    vm_state.z = Some(true);
    vm_state.c = Some(false);
    vm_state.v = Some(true);

    // ADDS <Rd>, <Rn>, #<imm3>
    let mut vm_smallimm = create_vm_from_asm(
        "
        adds r0, r2,        #0x07
        svc                 #0xFF
    ",
    );

    // ADDS <Rdn>, #<imm8>
    let mut vm_bigimm = create_vm_from_asm(
        "
        adds r0,            #0xFF
        svc                 #0xFF
    ",
    );

    // ADDS <Rd>, <Rn>, <Rm>
    let mut vm_3reg = create_vm_from_asm(
        "
        adds r0, r1, r2
        svc                 #0xFF
    ",
    );

    // ADD <Rdn>, <Rm> - Not applicable

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_smallimm);
    load_into_vm!(vm_state, vm_bigimm);
    load_into_vm!(vm_state, vm_3reg);

    // Expected state diff after execution
    vm_state.r[0] = None;
    vm_state.n = Some(false);
    vm_state.z = Some(false);
    vm_state.c = Some(true);
    vm_state.v = Some(false);

    println!(">>> [1/3] Testing for op variant: ADDS <Rd>, <Rn>, #<imm3> ");
    execute_and_assert!(vm_state, vm_smallimm);

    println!(">>> [2/3] Testing for op variant: ADDS <Rdn>, #<imm8> ");
    execute_and_assert!(vm_state, vm_bigimm);

    println!(">>> [3/3] Testing for op variant: ADDS <Rd>, <Rn>, <Rm> ");
    execute_and_assert!(vm_state, vm_3reg);
}

// Set V flag when addition cause signed overflow
#[test]
pub fn test_add_flag_v() {
    println!(">>> Add ops test case: Set V flag when addition cause signed overflow \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0x7FFF_FFFF);
    vm_state.r[1] = Some(0xFF);
    vm_state.r[2] = Some(0x7FFF_FFFF);
    vm_state.n = Some(false); // Will be set too because signed overflow by definition changes sign
    vm_state.z = Some(true);
    vm_state.c = Some(true);
    vm_state.v = Some(false);

    // ADDS <Rd>, <Rn>, #<imm3>
    let mut vm_smallimm = create_vm_from_asm(
        "
        adds r0, r2,        #0x07
        svc                 #0xFF
    ",
    );

    // ADDS <Rdn>, #<imm8>
    let mut vm_bigimm = create_vm_from_asm(
        "
        adds r0,            #0xFF
        svc                 #0xFF
    ",
    );

    // ADDS <Rd>, <Rn>, <Rm>
    let mut vm_3reg = create_vm_from_asm(
        "
        adds r0, r1, r2
        svc                 #0xFF
    ",
    );

    // ADD <Rdn>, <Rm> - Not applicable

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_smallimm);
    load_into_vm!(vm_state, vm_bigimm);
    load_into_vm!(vm_state, vm_3reg);

    // Expected state diff after execution
    vm_state.r[0] = None;
    vm_state.n = Some(true);
    vm_state.z = Some(false);
    vm_state.c = Some(false);
    vm_state.v = Some(true);

    println!(">>> [1/3] Testing for op variant: ADDS <Rd>, <Rn>, #<imm3> ");
    execute_and_assert!(vm_state, vm_smallimm);

    println!(">>> [2/3] Testing for op variant: ADDS <Rdn>, #<imm8> ");
    execute_and_assert!(vm_state, vm_bigimm);

    println!(">>> [3/3] Testing for op variant: ADDS <Rd>, <Rn>, <Rm> ");
    execute_and_assert!(vm_state, vm_3reg);
}

// ADD <Rdn>, <Rm>: Calculate sum of two high registers + Preserve flags
#[test]
pub fn test_add_high_noflags() {
    println!("\n>>> ADDS <Rd>, <Rn>, <Rm> op special test case: Calculate sum of two high registers + Preserve flags \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[8] = Some(0xFF);
    vm_state.r[9] = Some(0x0A);
    vm_state.n = Some(true);
    vm_state.z = Some(true);
    vm_state.c = Some(true);
    vm_state.v = Some(true);
    

    println!("\n>>> Creating VM(s) \n");

    // ADD <Rdn>, <Rm>
    let mut vm_high = create_vm_from_asm(
        "
        add  r8, r9
        svc                 #0xFF
    ",
    );

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_high);

    // Expected state diff after execution
    vm_state.r[8] = Some(0x0109);

    println!("\n>>> [1/1] Testing for op variant: ADD <Rdn>, <Rm> \n");
    execute_and_assert!(vm_state, vm_high);
}