extern crate narm;
mod common;

use common::*;

/*

Unit test for bitwise OR operator

Included cases:

- Calculate correctly for two register values
- Set Zero flag if result is zero
- Set Negative flag if result is negative

All tests also check for unexpected changes in registers and condition flags
TODO: Add tests where untargeted values are pre-set and check if incorrectly reset?

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// Test if operation calculate correctly for two register values
#[test]
pub fn test_orr_register() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0x05
        movs r1,            #0xAA
        orrs r0, r1
        svc                 #0xFF
    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.r[0] = Some(0xAF);
    vm_expected.r[1] = Some(0xAA);

    assert_vm_eq!(vm_expected, vm);
}

// Test if operation set Zero flag if result is zero
#[test]
pub fn test_orr_flag_zero() {
    let mut vm = create_vm_from_asm(
        "
        orrs r0, r1
        svc                 #0xFF
    ",
    );
    assert_eq!(vm.execute().unwrap(), 0xFF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();

    vm_expected.z = Some(true);

    assert_vm_eq!(vm_expected, vm);
}

// Test if operation set Negative flag if result is negative
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_orr_flag_neg() {
    let mut vm = create_vm_from_asm(
        "
        movs r0,            #0x02
        lsls r0,            #0x0F
        lsls r0,            #0x0F
        orrs r0, r0
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
