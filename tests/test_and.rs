extern crate narm;
mod common;

use common::*;

/*
Unit test for bitwise AND operator

Very basic functionality tests here because of the operation's simplicity
*/

// Test if AND(0, 1) = 1
#[test]
pub fn test_and_one(){
    let mut vm = create_test_vm("test_and_one");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0x1);
}

// Test if AND(1010 1010, 0101 0101) = 0000 0000
#[test]
pub fn test_and_negation(){
    let mut vm = create_test_vm("test_and_negation");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0x0);
}

// Test if AND(0, 0) correctly sets ZERO flag
#[test]
pub fn test_and_flag_zero(){
    let mut vm = create_test_vm("test_and_flag_zero");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
}

// Test if AND(1000 0000 0000 0000, 1000 0000 0000 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_and_flag_neg(){
    let mut vm = create_test_vm("test_and_flag_neg");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.n);
}
