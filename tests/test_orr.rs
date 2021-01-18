extern crate narm;
mod common;

use common::*;

/*

Unit test for bitwise OR operator

Included cases: 

- Calculate correctly for two register values
- Set Zero flag if result is zero
- Set Negative flag if result is negative

All tests also check for unexpected changes in untargeted lo registers and condition flags
TODO: Add tests where untargeted values are pre-set and check if incorrectly reset?

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if OR(0000 0101, 1010 1010) = 1010 1111
#[test]
pub fn test_orr_register(){
    let mut vm = create_test_vm("test_orr_register");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_lo_regs!(vm, 0xAF, 0xAA);
    assert_flags_nzcv!(vm, false, false, false, false);
}

// Test if OR(0, 0) correctly sets ZERO flag
#[test]
pub fn test_orr_flag_zero(){
    let mut vm = create_test_vm("test_orr_flag_zero");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_lo_regs!(vm);
    assert_flags_nzcv!(vm, false, true, false, false);
}

// Test if OR(1000 ... 0000, 1000 ... 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_orr_flag_neg(){
    let mut vm = create_test_vm("test_orr_flag_neg");
    vm.execute().unwrap();
    vm.print_diagnostics();
    assert_lo_regs!(vm, 0x8000_0000);
    assert_flags_nzcv!(vm, true, false, false, false);
}
