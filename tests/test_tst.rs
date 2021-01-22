extern crate narm;
mod common;

use common::*;

/*

Unit test for TST operator - bitwise AND that only set flags and discard result

Included cases:

- Don't alter target register
- Set Zero flag if result is zero
- Set Negative flag if result is negative

All tests also check for unexpected changes in registers and condition flags
TODO: Add tests where untargeted values are pre-set and check if incorrectly reset?

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if operation don't alter target register
#[test]
pub fn test_tst_unaltered() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0xFF
        movs r1,            #0x0A
        tst r0, r1
        svc                 #0xFF
    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0xFF);
    vm_expected.r[1] = Some(0x0A);

    assert_vm_eq!(vm_expected, vm);
}

// Test if operation set Zero flag if result is zero
#[test]
pub fn test_tst_flag_zero() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0x55
        movs r1,            #0xAA
        tst r0, r1
        svc                 #0xFF
    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0x55);
    vm_expected.r[1] = Some(0xAA);
    vm_expected.z = Some(true);

    assert_vm_eq!(vm_expected, vm);
}

// Test if operation set Negative flag if result is negative
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_tst_flag_neg() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0x08
        lsls r0,            #0x0E
        lsls r0,            #0x0E
        tst r0, r0
        svc                 #0xFF
    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0x8000_0000);
    vm_expected.n = Some(true);

    assert_vm_eq!(vm_expected, vm);
}
