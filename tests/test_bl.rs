extern crate narm;
mod common;

use common::*;

//this will test small nearby jumps using bl
#[test]
pub fn test_bl_small_jump() {
    let mut vm = create_vm_from_asm(
        "
        bl test1
        svc #0x01
        test1:
        movs r0, #1
        svc #0xFF
    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(1);
    vm_expected.r[14] = Some(0x1_0000 + 4 + 1); //+4 because bl is 32bit opcode, +1 to indicate thumb mode

    assert_vm_eq!(vm_expected, vm);
}

#[test]
pub fn test_bl_big_jump() {
    //note this test will access unavailable memory
    let mut vm = create_vm_from_asm(
        "
        bl #0xF0020
        svc #0x01
    ",
    );
    assert!(vm.execute().is_err());
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[14] = Some(0x1_0000 + 4 + 1); //+4 because bl is 32bit opcode, +1 to indicate thumb mode
    vm_expected.pc_address = Some(0xF_0020);

    assert_vm_eq!(vm_expected, vm);
}

#[test]
pub fn test_bl_backward_jump() {
    //note this test will access unavailable memory
    let mut vm = create_vm_from_asm(
        "
        bl #0x50
        svc #0x01
    ",
    );
    assert!(vm.execute().is_err());
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[14] = Some(0x1_0000 + 4 + 1); //+4 because bl is 32bit opcode, +1 to indicate thumb mode
    vm_expected.pc_address = Some(0x50);

    assert_vm_eq!(vm_expected, vm);
}
