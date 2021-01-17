extern crate narm;
mod common;

use common::*;

/*
Unit test for bitwise OR operator

Very basic functionality tests here because of the operation's simplicity
*/

// Test if 1010 1010 OR 0101 0101 = 1111 1111
#[test]
pub fn test_orr_register(){
    let mut vm = create_test_vm("test_orr_register");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0xFF);
}

// Test if 0 OR 0 correctly sets ZERO flag
#[test]
pub fn test_orr_flag_zero(){
    let mut vm = create_test_vm("test_orr_flag_zero");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
}

// Test if 1000 0000 0000 0000 OR 1000 0000 0000 0000 correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_orr_flag_neg(){
    let mut vm = create_test_vm("test_orr_flag_neg");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.n);
}
