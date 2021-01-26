extern crate narm;
mod common;

use common::*;

/*

Unit test for ADD operators using the SP

Included varieties:

ADD <Rd>,SP,#<imm8> T1      (sp_imm_reg)    - Rd  <- SP  + imm  NOTE: Actually 10 bit imm with lowest 2 bits omitted and forced to 0
ADD SP,SP,#<imm7> T2        (sp_imm_sp)     - SP  <- SP  + imm  NOTE: Actually 9 bit imm with lowest 2 bits omitted and forced to 0
ADD <Rdm>, SP, <Rdm> T1     (sp_reg_reg)    - Rdn <- Rdn + SP
ADD SP,<Rm> T2              (sp_reg_sp)     - SP  <- SP  + Rm

General test cases:

- Do SP addition with SP as destination + Preserve flags
- Do SP addition with a register as destination + Preserve flags

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Do SP addition with SP as destination + Preserve flags
#[test]
pub fn test_add_sp_to_sp() {
    println!("\n>>> Add sp ops test case: Do SP addition with SP as destination + Preserve flags \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0x01FC);
    vm_state.r[13] = Some(0x01_0000);
    vm_state.n = Some(true);
    vm_state.z = Some(true);
    vm_state.c = Some(true);
    vm_state.v = Some(true);

    println!("\n>>> Creating VM(s) \n");

    // ADD <Rd>,SP,#<imm8> T1 - Not applicable

    // ADD SP,SP,#<imm7> T2
    let mut vm_sp_imm_sp = create_vm_from_asm(
        "
        add  SP, SP,        #0x01FC
        svc                 #0xFF
    ",
    );

    // ADD <Rdm>, SP, <Rdm> T1 - Not applicable

    // ADD SP,<Rm> T2
    let mut vm_sp_reg_sp = create_vm_from_asm(
        "
        add  SP, r0
        svc                 #0xFF
    ",
    );

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_sp_imm_sp);
    load_into_vm!(vm_state, vm_sp_reg_sp);

    // Expected state diff after execution
    vm_state.r[13] = Some(0x01_01FC); // Lower 2 bits of SP are always 0

    println!("\n>>> [1/2] Testing for op variant: ADD SP,SP,#<imm7> T2 \n");
    execute_and_assert!(vm_state, vm_sp_imm_sp);

    println!("\n>>> [2/2] Testing for op variant: ADD SP,<Rm> T2 \n");
    execute_and_assert!(vm_state, vm_sp_reg_sp);
}

// Do SP addition with register as destination + Preserve flags
#[test]
pub fn test_add_sp_to_reg() {
    println!("\n>>> Add sp ops test case: Do SP addition with SP as destination + Preserve flags \n");

    let mut vm_state: VMState = Default::default();

    // Initial state
    vm_state.r[0] = Some(0x03FC);
    vm_state.r[13] = Some(0x01_0000);
    vm_state.n = Some(true);
    vm_state.z = Some(true);
    vm_state.c = Some(true);
    vm_state.v = Some(true);

    println!("\n>>> Creating VM(s) \n");

    // ADD <Rd>, SP, #<imm8> T1
    let mut vm_sp_imm_reg = create_vm_from_asm(
        "
        add  r0, SP,        #0x03FC
        svc                 #0xFF
    ",
    );

    // ADD SP, SP, #<imm7> T2 - Not applicable

    // ADD <Rdm>, SP, <Rdm> T1
    let mut vm_sp_reg_reg = create_vm_from_asm(
        "
        add  r0, SP, r0
        svc                 #0xFF
    ",
    );

    // ADD SP,<Rm> T2 - Not applicable

    println!("\n>>> Loading initial values into VM(s) \n");
    print_vm_state!(vm_state);
    load_into_vm!(vm_state, vm_sp_imm_reg);
    load_into_vm!(vm_state, vm_sp_reg_reg);

    // Expected state diff after execution
    vm_state.r[0] = Some(0x01_03FC); // Lower 2 bits of SP are always 0

    println!("\n>>> [1/2] Testing for op variant: ADD <Rd>, SP, #<imm8> T1 \n");
    execute_and_assert!(vm_state, vm_sp_imm_reg);

    println!("\n>>> [2/2] Testing for op variant: ADD <Rdm>, SP, <Rdm> T1 \n");
    execute_and_assert!(vm_state, vm_sp_reg_reg);
}
