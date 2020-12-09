extern crate narm;
mod common;

use common::*;

#[test]
pub fn test_cycle(){
    let mut vm = create_test_vm("test_cycle");
    vm.cycle().unwrap();
    vm.print_diagnostics();
    assert_eq!(vm.external_get_reg(0), 0xF1);
}


