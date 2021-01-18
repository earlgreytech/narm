extern crate narm;
mod common;

use common::*;

#[test]
pub fn test_cycle(){
    let mut vm = create_vm_from_asm("
        movs r0, #0xF1
    ");
    vm.cycle().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0xF1);
}

#[test]
pub fn test_execute(){
    let mut vm = create_vm_from_asm("
        movs r0, #0xF1
        svc #0xFF
    ");
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0xF1);
}

