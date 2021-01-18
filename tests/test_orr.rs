extern crate narm;
mod common;

use common::*;

/*

Unit test for bitwise OR operator

Included cases: 

- Calculate for two register values
- Set Zero flag if result is zero
- Set Negative flag if result is negative

The reference for these tests are currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if OR(1010 1010, 0101 0101) = 1111 1111
#[test]
pub fn test_orr_register(){
    let mut vm = create_test_vm("test_orr_register");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0xFF);
}

// Test if OR(0, 0) correctly sets ZERO flag
#[test]
pub fn test_orr_flag_zero(){
    let mut vm = create_test_vm("test_orr_flag_zero");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
}

// Test if OR(1000 ... 0000, 1000 ... 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_orr_flag_neg(){
    let mut vm = create_test_vm("test_orr_flag_neg");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.n);
}
