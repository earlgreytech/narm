extern crate elf;

use narm::narmvm::*;

const DEFAULT_GAS: u64 = 10000;

#[cfg(test)]
pub fn create_vm_from_asm(assembly_code: &str) -> NarmVM {
    let file = asm(assembly_code);

    let text_scn = file.get_section(".text").unwrap();
    assert!(text_scn.shdr.addr == 0x01_0000);
    assert!(text_scn.data.len() < 0x01_0000);

    let mut vm = NarmVM::default();
    vm.memory.add_memory(0x01_0000, 0x01_0000).unwrap();
    vm.copy_into_memory(0x01_0000, &text_scn.data).unwrap();
    vm.set_thumb_pc_address(0x01_0000);
    vm.gas_remaining = DEFAULT_GAS;
    vm
}

#[cfg(test)]
pub fn asm(input: &str) -> elf::File {
    use std::io::Write;
    use std::process::Command;
    use tempfile::*;
    let asm = format!(
        "{}{}",
        "
    .syntax unified
    .section .text
    .thumb_func
    .globl _start
    _start:

    ",
        input
    );
    let linkerscript = "
    ENTRY (_start)
    SECTIONS
    {
        . = 0x010000;
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
    println!("asm: {}\n---------------", asm);

    let mut f1 = std::fs::File::create(&input).unwrap();
    writeln!(f1, "{}", asm).unwrap();
    f1.flush().unwrap();

    let mut f2 = std::fs::File::create(&linkfile).unwrap();
    writeln!(f2, "{}", linkerscript).unwrap();
    f2.flush().unwrap();

    // The following commands will be executed
    // arm-none-eabi-as -march=armv6s-m -o$OBJECT $INPUT
    // arm-none-eabi-ld -T link.ld -o$OUTPUT $OBJECTF

    let result = Command::new("arm-none-eabi-as")
        .arg("-march=armv6s-m")
        .arg(format!("{}{}", "-o", &object.to_str().unwrap()))
        .arg(&input)
        .output()
        .unwrap();
    println!(
        "as stdout: {}",
        std::str::from_utf8(&result.stdout).unwrap()
    );
    println!(
        "as stderr: {}",
        std::str::from_utf8(&result.stderr).unwrap()
    );

    let result = Command::new("arm-none-eabi-ld")
        .arg(format!("{}{}", "-T", &linkfile.to_str().unwrap()))
        .arg(format!("{}{}", "-o", &output.to_str().unwrap()))
        .arg(&object)
        .output()
        .unwrap();

    println!(
        "ld stdout: {}",
        std::str::from_utf8(&result.stdout).unwrap()
    );
    println!(
        "ld stderr: {}",
        std::str::from_utf8(&result.stderr).unwrap()
    );

    elf::File::open_path(output).unwrap()
}

/***************************************************************
***                                                          ***
***   VM state assertion infrastructure                      ***
***                                                          ***
***   Contains a structure holding a VM state,               ***
***   where values can easily be set (or ignored).           ***
***   Also contains macros to assert, load and print state   ***
***                                                          ***
***************************************************************/

// TODO: Handle special registers differently?
// TODO: Implement memory area assertion? Maybe too advanced?
#[allow(dead_code)]
pub struct VMState {
    pub r: [Option<u32>; 15], // Exclude PC for the time being
    pub n: Option<bool>,
    pub z: Option<bool>,
    pub c: Option<bool>,
    pub v: Option<bool>,
    pub pc_address: Option<u32>,
}

impl Default for VMState {
    fn default() -> VMState {
        VMState {
            r: [
                Some(0), // r0
                Some(0), // r1
                Some(0), // r2
                Some(0), // r3
                Some(0), // r4
                Some(0), // r5
                Some(0), // r6
                Some(0), // r7
                Some(0), // r8
                Some(0), // r9
                Some(0), // r10
                Some(0), // r11
                Some(0), // r12
                Some(0), // r13
                Some(0), // r14
            ],
            n: Some(false),
            z: Some(false),
            c: Some(false),
            v: Some(false),
            pc_address: None, //ignore pc normally
        }
    }
}
// Macro to assert values in VMState struct against the actual VM's state
// Includes a custom error message that formats register values to padded hex strings in addition to the default decimal print
// This could be done as a function, but that would bloat a stack trace compared to an inlined macro.
#[macro_export]
macro_rules! assert_vm_eq {
    ( $vmstate:ident, $vm:ident ) => {
        // Registers
        for i in 0..=14 {
            match ($vmstate.r[i]) {
                Some(x) => assert_eq!(x, $vm.external_get_reg(i), "\n\nRegister r{}: Expected 0x{}, actually contained 0x{}\n\n", i, format_padded_hex(x), format_padded_hex($vm.external_get_reg(i))),
                None    => (),
            };
        }
        // Negative flag
        match ($vmstate.n) {
            Some(x) => assert_eq!(x, $vm.cpsr.n, "\n\nCondition flag n (Negative): Expected {}, actually contained {}\n\n", x, $vm.cpsr.n),
            None    => (),
        };
        // Zero flag
        match ($vmstate.z) {
            Some(x) => assert_eq!(x, $vm.cpsr.z, "\n\nCondition flag z (Zero): Expected {}, actually contained {}\n\n", x, $vm.cpsr.z),
            None    => (),
        };
        // Carry (Overflow) flag
        match ($vmstate.c) {
            Some(x) => assert_eq!(x, $vm.cpsr.c, "\n\nCondition flag c (Carry/Unsigned Overflow): Expected {}, actually contained {}\n\n", x, $vm.cpsr.c),
            None    => (),
        };
        // V (Signed Overflow) flag
        match ($vmstate.v) {
            Some(x) => assert_eq!(x, $vm.cpsr.v, "\n\nCondition flag v (Signed Overflow): Expected {}, actually contained {}\n\n", x, $vm.cpsr.v),
            None    => (),
        };
        // PC, program counter
        match ($vmstate.pc_address) {
            Some(x) => assert_eq!(x, $vm.get_pc_address(), "\n\npc: Expected {}, actually contained {}\n\n", x, $vm.get_pc_address()),
            None    => (),
        };
    };
}

// Macro to load values in VMState struct into the actual VM's state
// This could be done as a function, but I made the assertion thing above a macro and now it's too late
#[macro_export]
macro_rules! load_into_vm {
    ( $vmstate:ident, $vm:ident ) => {
        // Registers
        for i in 0..=14 {
            match ($vmstate.r[i]) {
                Some(x) => $vm.external_set_reg(i, x),
                None    => (),
            };
        }
        // Negative flag
        match ($vmstate.n) {
            Some(x) => $vm.cpsr.n = x,
            None    => (),
        };
        // Zero flag
        match ($vmstate.z) {
            Some(x) => $vm.cpsr.z = x,
            None    => (),
        };
        // Carry (Overflow) flag
        match ($vmstate.c) {
            Some(x) => $vm.cpsr.c = x,
            None    => (),
        };
        // V (Signed Overflow) flag
        match ($vmstate.v) {
            Some(x) => $vm.cpsr.v = x,
            None    => (),
        };
        // PC, program counter
        match ($vmstate.pc_address) {
            Some(x) => $vm.set_thumb_pc_address(x),
            None    => (),
        };
    };
}

// Macro to print values in VMState struct for debug output
// This could be done as a function, but I made the assertion thing above a macro and now it's too late
#[macro_export]
macro_rules! print_vm_state {
    ( $vmstate:ident ) => {
        // Registers
        for i in 0..=14 {
            match ($vmstate.r[i]) {
                Some(x) => println!("r{}: 0x{}", i, format_padded_hex(x)),
                None => println!("r{}: (Ignored)", i),
            };
        }
        // Negative flag
        match ($vmstate.n) {
            Some(x) => println!("n: {}", x),
            None => println!("n: (Ignored)"),
        };
        // Zero flag
        match ($vmstate.z) {
            Some(x) => println!("z: {}", x),
            None => println!("z: (Ignored)"),
        };
        // Carry (Overflow) flag
        match ($vmstate.c) {
            Some(x) => println!("c: {}", x),
            None => println!("c: (Ignored)"),
        };
        // V (Signed Overflow) flag
        match ($vmstate.v) {
            Some(x) => println!("v: {}", x),
            None => println!("v: (Ignored)"),
        };
        // PC, program counter
        match ($vmstate.pc_address) {
            Some(x) => println!("pc address: {}", format_padded_hex(x)),
            None    => println!("pc address: (Ignored)"),
        };
    };
}

/***********************************************
***                                          ***
***   Misc supporting functions and macros   ***
***                                          ***
***********************************************/

// Format u32 to hex string approperiatly padded with zeroes for easy side-by-side comparison
// TODO: Underscores?
// TODO: Replace with build-in functionality used in VM code?
pub fn format_padded_hex(int: u32) -> String {
    let mut string = String::from(format!("{:x}", int));
    while string.len() < 8 {
        string = format!("0{}", string)
    }
    string.to_uppercase()
}

// Macro to reduce boilerplate code when executing VM instance and asserting results
#[macro_export]
macro_rules! execute_and_assert {
    ( $vmstate:ident, $vm:ident ) => {
        assert_eq!($vm.execute().unwrap(), 0xFF);
        $vm.print_diagnostics();
        assert_vm_eq!($vmstate, $vm);
    }
}