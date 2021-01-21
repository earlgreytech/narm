extern crate narm;
mod common;

use common::*;

#[test]
pub fn test_add_lo_high_register(){
    let mut vm = create_vm_from_asm("
        movs r0,            #0x0000000A
        mov r8, r0
        mov r13, r0
        mov r1, r13
        svc                 #0x000000FF
    ");
    assert_eq!(vm.execute().unwrap(), 0x0000_00FF);
    vm.print_diagnostics();
    let mut vm_expected: VMState = Default::default();
    
    vm_expected.r[0] = Some(0x0000_000A);
    vm_expected.r[8] = Some(0x0000_000A);
    vm_expected.r[13] = Some(0x0000_0008); // A -> 8 because r13/SP always has 0 in first 2 bits
    vm_expected.r[1] = Some(0x0000_0008);
    
    assert_vm_eq!(vm_expected, vm);
}