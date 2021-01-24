extern crate narm;
mod common;

use common::*;

// this file is for basic smoke testing of the actual common.rs test functions

#[test]
pub fn test_cycle() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0xF1
    ",
    );
    vm.cycle().unwrap();
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0xF1);

    assert_vm_eq!(vm_expected, vm);
}

#[test]
pub fn test_execute() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0xF1
        svc                 #0xFF
    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0xF1);

    assert_vm_eq!(vm_expected, vm);
}
