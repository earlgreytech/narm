extern crate elf;

use narm::narmvm::*;


const DEFAULT_GAS:u64 = 10000;

#[cfg(test)]
pub fn create_vm_from_asm(assembly_code: &str) -> NarmVM{
    let prefix = "tests/bin/";
    let file = asm(assembly_code);

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

#[cfg(test)]
pub fn asm(input: &str) -> elf::File{
    use tempfile::*;
    use std::io::Write;
    use std::process::Command;
    let asm = format!("{}{}", "
    .syntax unified
    .section .text
    .thumb_func
    .globl _start
    _start:

    ", input);
    let linkerscript = "
    ENTRY (_start)
    SECTIONS
    {
        . = 0x10000;
        .text : { *(.text*) *(.rodata*) }
        .data : { *(.data*) }
        /*.bss : { *(.bss*) *(COMMON*) }*/
    }
    ";
    let dir = tempdir().unwrap();
    let input = dir.path().join("test_code.asm");
    let object = dir.path().join("test_code.o");
    let output = dir.path().join("test_code.elf");
    let linkfile = dir.path().join("link.ld");
    //println!("input: {}", input.to_str().unwrap());
    println!("asm: {}\n---------------", asm);

    let mut f1 = std::fs::File::create(&input).unwrap();
    writeln!(f1,"{}", asm).unwrap();
    f1.flush().unwrap();


    let mut f2 = std::fs::File::create(&linkfile).unwrap();
    writeln!(f2,"{}", linkerscript).unwrap();
    f2.flush().unwrap();
    
    let result = Command::new("arm-none-eabi-as").
        arg("-march=armv6s-m").
        arg(format!("{}{}", "-o", &object.to_str().unwrap())).
        arg(&input).
        output().unwrap();
    println!("as stdout: {}", std::str::from_utf8(&result.stdout).unwrap());
    println!("as stderr: {}", std::str::from_utf8(&result.stderr).unwrap());

    let result = Command::new("arm-none-eabi-ld").
        arg(format!("{}{}", "-T", &linkfile.to_str().unwrap())).
        arg(format!("{}{}", "-o", &output.to_str().unwrap())).
        arg(&object).
        output().unwrap();

    println!("ld stdout: {}", std::str::from_utf8(&result.stdout).unwrap());
    println!("ld stderr: {}", std::str::from_utf8(&result.stderr).unwrap());
    //  arm-none-eabi-as -march=armv6s-m -o $OBJECTFILE $f
    //  arm-none-eabi-ld -T link.ld -o $FINALFILE $OBJECTFILE

    elf::File::open_path(output).unwrap()
}

