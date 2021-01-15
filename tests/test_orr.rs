extern crate narm;
mod common;

use common::*;

// Test if 1000 OR 0000 = 1000
#[test]
pub fn test_orr_one(){
    let mut vm = create_test_vm("test_orr_one");
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0x1);
}

// Test if 1010 1010 OR 0101 0101 = 1111 1111
#[test]
pub fn test_orr_saturation(){
    let mut vm = create_test_vm("test_orr_saturation");
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.cycle().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0xFF);
}
