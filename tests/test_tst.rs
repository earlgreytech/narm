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

// Test if AND(... 0000 1111, ... 1010 1010) correctly doesn't alter target registry
#[test]
pub fn test_tst_unaltered(){
    let mut vm = create_vm_from_asm("
        movs r0, #0x00_00_00_FF
        movs r1, #0x00_00_00_0A
        tst r0, r1
        svc #0x00_00_00_FF
    ");
    assert_eq!(vm.execute().unwrap(), 0x00_00_00_FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();
    
    vm_expected.r[0] = Some(0x00_00_00_FF);
    vm_expected.r[1] = Some(0x00_00_00_0A);
    
    assert_vm_eq!(vm_expected, vm);
}

// Test if TST(... 0101 0101, ... 1010 1010) correctly sets ZERO flag
#[test]
pub fn test_tst_flag_zero(){
    let mut vm = create_vm_from_asm("
        movs r0, #0x00_00_00_55
        movs r1, #0x00_00_00_AA
        tst r0, r1
        svc #0x00_00_00_FF
    ");
    assert_eq!(vm.execute().unwrap(), 0x00_00_00_FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();
    
    vm_expected.r[0] = Some(0x00_00_00_55);
    vm_expected.r[1] = Some(0x00_00_00_AA);
    vm_expected.z = Some(true);
    
    assert_vm_eq!(vm_expected, vm);
}

// Test if TST(1000 ... 0000, 1000 ... 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_tst_flag_neg(){
    let mut vm = create_vm_from_asm("
        movs r0, #0x00_00_00_02
        lsls r0, #0x00_00_00_0F
        lsls r0, #0x00_00_00_0F
        tst r0, r0
        svc #0x00_00_00_FF
    ");
    assert_eq!(vm.execute().unwrap(), 0x00_00_00_FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();
    
    vm_expected.r[0] = Some(0x80_00_00_00);
    vm_expected.n = Some(true);
    
    assert_vm_eq!(vm_expected, vm);
}
