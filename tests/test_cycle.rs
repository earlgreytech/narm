extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

#[test]
pub fn test_cycle(){
    let mut vm = create_test_vm("test_cycle");
    vm.cycle().unwrap();
    assert_eq!(vm.external_get_reg(0), 0xF1);
}


