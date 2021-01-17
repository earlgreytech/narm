extern crate narm;
mod common;

use common::*;

/*
Unit test for TST operator - bitwise AND that only set flags and discard result

Copies the relevant test cases from the AND unit test
*/

// Test if TST(0, 0) correctly sets ZERO flag
#[test]
pub fn test_tst_flag_zero(){
    let mut vm = create_test_vm("test_tst_flag_zero");
    multistep_vm!(1, vm);
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
}

// Test if TST(1000 0000 0000 0000, 1000 0000 0000 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_tst_flag_neg(){
    let mut vm = create_test_vm("test_tst_flag_neg");
    multistep_vm!(4, vm);
    vm.print_diagnostics();
    assert!(vm.cpsr.n);
}
