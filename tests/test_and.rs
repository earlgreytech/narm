extern crate narm;
mod common;

use common::*;

// Test if 1111 AND 1000 = 1000
#[test]
pub fn test_and_one(){
    let mut vm = create_test_vm("test_and_one");
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0x1);
}

// Test if 1010 1010 AND 0101 0101 = 0000 0000
#[test]
pub fn test_and_negation(){
    let mut vm = create_test_vm("test_and_negation");
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0x0);
}
