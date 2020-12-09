extern crate elf;

use narm::narmvm::*;


const DEFAULT_GAS:u64 = 10000;

pub fn create_test_vm(file: &str) -> NarmVM {
    let prefix = "tests/bin/";
    let file = elf::File::open_path([prefix, file].join("")).unwrap();

    let text_scn = file.get_section(".text").unwrap();
    assert!(text_scn.shdr.addr == 0x10000);
    assert!(text_scn.data.len() < 0xFFFF);
    //let data_scn = file.get_section(".data").unwrap();
    //assert!(data_scn.shdr.addr == 0x80020000);

    let mut vm = NarmVM::default();
    vm.memory.add_memory(0x10000, 0xFFFF).unwrap();
    vm.copy_into_memory(0x10000, &text_scn.data).unwrap();
    vm.set_pc(0x10000);
    vm.gas_remaining = DEFAULT_GAS;
    vm
}
/*
fn setup_api(api: &mut TestbenchAPI, code: &Vec<u8>, data: &Vec<u8>){
    api.push_sccs(&vec![]).unwrap(); //extra data
    api.push_sccs(data).unwrap();
    api.push_sccs(&vec![1]).unwrap(); //data section count
    api.push_sccs(code).unwrap();
    api.push_sccs(&vec![1]).unwrap(); //code section count
    api.push_sccs(&vec![0, 0, 0, 0]).unwrap(); //vmversion (fill in properly later)
    api.context.exec.gas_limit = MAX_GAS;
    
}

*/

