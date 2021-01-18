extern crate narm;
mod common;

use common::*;

/*

Unit test for TST operator - bitwise AND that only set flags and discard result

Included cases: 

- Don't alter target register (Unlike AND)
- Set Zero flag if result is zero
- Set Negative flag if result is negative

All tests also check for unexpected changes in untargeted lo registers and condition flags
TODO: Add tests where untargeted values are pre-set and check if incorrectly reset?

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if AND(0000 1111, 1010 1010) correctly doesn't alter target registry
#[test]
pub fn test_tst_unaltered(){
    let mut vm = create_vm_from_asm("
        movs r0, #0xFF
        movs r1, #0x0A
        tst r0, r1
        svc #0xFF
    ");
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    assert_lo_regs!(vm, 0xFF, 0x0A);
    assert_flags_nzcv!(vm, false, false, false, false);
}

// Test if TST(0101 0101, 1010 1010) correctly sets ZERO flag
#[test]
pub fn test_tst_flag_zero(){
    let mut vm = create_vm_from_asm("
        movs r0, #0x55
        movs r1, #0xAA
        tst r0, r1
        svc #0xFF
    ");
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    assert_lo_regs!(vm, 0x55, 0xAA);
    assert_flags_nzcv!(vm, false, true, false, false);
}

// Test if TST(1000 ... 0000, 1000 ... 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_tst_flag_neg(){
    let mut vm = create_vm_from_asm("
        movs r0, #0x02
        lsls r0, #0x0F
        lsls r0, #0x0F
        tst r0, r0
        svc #0xFF
    ");
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    assert_lo_regs!(vm, 0x8000_0000);
    assert_flags_nzcv!(vm, true, false, false, false);
}
