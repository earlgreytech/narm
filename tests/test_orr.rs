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

// Test if OR(... 0000 0101, ... 1010 1010) = ... 1010 1111
#[test]
pub fn test_orr_register(){
    let mut vm = create_vm_from_asm("
        movs r0, #0x00000005
        movs r1, #0x000000AA
        orrs r0, r1
        svc #0x000000FF
    ");
    assert_eq!(vm.execute().unwrap(), 0x0000_00FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();
    
    vm_expected.r[0] = Some(0x0000_00AF);
    vm_expected.r[1] = Some(0x0000_00AA);
    
    assert_vm_eq!(vm_expected, vm);
}

// Test if OR(... 0000, ... 0000) correctly sets ZERO flag
#[test]
pub fn test_orr_flag_zero(){
    let mut vm = create_vm_from_asm("
        orrs r0, r1
        svc #0x000000FF
    ");
    assert_eq!(vm.execute().unwrap(), 0x0000_00FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();
    
    vm_expected.z = Some(true);
    
    assert_vm_eq!(vm_expected, vm);
}

// Test if OR(1000 ... 0000, 1000 ... 0000) correctly sets NEGATIVE flag
// ("highest" bit indicate sign in int representation, so setting it to 1 -> negative number)
#[test]
pub fn test_orr_flag_neg(){
    let mut vm = create_vm_from_asm("
        movs r0, #0x00000002
        lsls r0, #0x0000000F
        lsls r0, #0x0000000F
        orrs r0, r0
        svc #0x000000FF
    ");
    assert_eq!(vm.execute().unwrap(), 0x0000_00FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();
    
    vm_expected.r[0] = Some(0x8000_0000);
    vm_expected.n = Some(true);
    
    assert_vm_eq!(vm_expected, vm);
}
