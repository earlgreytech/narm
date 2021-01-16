extern crate narm;
mod common;

use common::*;

// Test if 0 AND 0 = 0 using status flag
#[test]
pub fn test_tst_zero(){
    let mut vm = create_test_vm("test_tst_zero");
    multistep_vm!(1, vm);
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
}

// Test if 1010 1010 AND 0101 0101 = 0 using status flag
#[test]
pub fn test_tst_negation(){
    let mut vm = create_test_vm("test_tst_negation");
    multistep_vm!(3, vm);
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
}
