extern crate narm;
mod common;

use common::*;

/*

Unit test for bitwise AND operator

Included cases: 

- Calculate for two register values
- Set Zero flag if result is zero
- Set Negative flag if result is negative

*/

// Test if AND(1010 1010, 0101 0101) = 0
#[test]
pub fn test_and_register(){
    let mut vm = create_test_vm("test_and_register");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0x0);
}

// Test if AND(0, 0) correctly sets Zero flag
#[test]
pub fn test_and_flag_zero(){
    let mut vm = create_test_vm("test_and_flag_zero");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
}

// Test if AND(1000 ... 0000, 1000 ... 0000) correctly sets Negative flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_and_flag_neg(){
    let mut vm = create_test_vm("test_and_flag_neg");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.n);
}
