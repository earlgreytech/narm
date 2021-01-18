extern crate narm;
mod common;

use common::*;

/*

Unit test for bitwise AND operator

Included cases: 

- Calculate correctly for two register values
- Set Zero flag if result is zero
- Set Negative flag if result is negative

All tests also check for unexpected changes in untargeted lo registers and condition flags
TODO: Add tests where untargeted values are pre-set and check if incorrectly reset?

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if AND(1010 1010, 0101 0101) = 0
#[test]
pub fn test_and_register(){
    let mut vm = create_test_vm("test_and_register");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_lo_regs!(vm, 0x0, 0xAA);
    assert_flags_nzcv!(vm, false, true, false, false);
}

// Test if AND(0, 0) correctly sets Zero flag
#[test]
pub fn test_and_flag_zero(){
    let mut vm = create_test_vm("test_and_flag_zero");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_lo_regs!(vm);
    assert_flags_nzcv!(vm, false, true, false, false);
}

// Test if AND(1000 ... 0000, 1000 ... 0000) correctly sets Negative flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_and_flag_neg(){
    let mut vm = create_test_vm("test_and_flag_neg");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_lo_regs!(vm, 0x8000_0000);
    assert_flags_nzcv!(vm, true, false, false, false);
}
