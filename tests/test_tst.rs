extern crate narm;
mod common;

use common::*;

/*

Unit test for TST operator - bitwise AND that only set flags and discard result

Included cases (same as for AND): 

- Set Zero flag if result is zero
- Set Negative flag if result is negative

Checks include in all cases: 
- Never set Carry or V (signed overflow) flags
TODO: Check that C or V in never *unset*?

The reference for these tests are currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if TST(0, 0) correctly sets ZERO flag
#[test]
pub fn test_tst_flag_zero(){
    let mut vm = create_test_vm("test_tst_flag_zero");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.z);
    assert!(!vm.cpsr.c);
    assert!(!vm.cpsr.v);
}

// Test if TST(1000 ... 0000, 1000 ... 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_tst_flag_neg(){
    let mut vm = create_test_vm("test_tst_flag_neg");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert!(vm.cpsr.n);
    assert!(!vm.cpsr.c);
    assert!(!vm.cpsr.v);
}
