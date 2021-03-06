#![allow(dead_code)] // Stuff used only in/by the macros generate warnings otherwise

extern crate elf;

use narm::narmvm::*;

const DEFAULT_GAS: u64 = 10000;

// These constant make addresses to asm code in testing less magic-number-y
pub const ASM_ENTRY: u32 = 0x01_0000;
pub const STACK_MEM_START: u32 = 0x8100_0000;
pub const THUMBS_MODE: u32 = 0x01;
pub const OP_SIZE: u32 = 0x02;
pub const OP_SIZE_32BIT: u32 = 0x04;
pub const WORD_SIZE: u32 = 0x04;

#[cfg(test)]
pub fn create_vm_from_asm(assembly_code: &str) -> NarmVM {
    let file = asm(assembly_code);

    let text_scn = file.get_section(".text").unwrap();
    assert!(text_scn.shdr.addr == 0x01_0000);
    assert!(text_scn.data.len() < 0x01_0000);

    let mut vm = NarmVM::default();
    vm.memory.add_memory(0x01_0000, 0x01_0000).unwrap();
    vm.copy_into_memory(0x01_0000, &text_scn.data).unwrap();
    //add stack memory
    vm.memory.add_memory(STACK_MEM_START, 0xFFFF).unwrap();
    vm.set_thumb_pc_address(ASM_ENTRY);
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
    println!("--------------\nasm: {}\n---------------", asm);

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

#[derive(Copy, Clone)]
pub struct VMState {
    pub r: [Option<u32>; 15],
    pub n: Option<bool>,
    pub z: Option<bool>,
    pub c: Option<bool>,
    pub v: Option<bool>,
    pub pc_address: Option<u32>,
    pub expect_exec_error: bool, // TODO: Allow asserting specific error???
    pub svc_param: u32,
    pub check_memory_start: Option<u32>, // If set, check 20 memory words starting here
    pub memory: [Option<u32>; 20],       // Only checked if check_memory_start is set
}

impl Default for VMState {
    fn default() -> VMState {
        VMState {
            r: [Some(0); 15],
            n: Some(false),
            z: Some(false),
            c: Some(false),
            v: Some(false),
            pc_address: None, //ignore pc normally
            expect_exec_error: false,
            svc_param: 0xFF,
            check_memory_start: None,
            memory: [Some(0); 20],
        }
    }
}
// Macro to assert values in VMState struct against the actual VM's state
// Includes a custom error message that formats register values to padded hex strings in addition to the default decimal print
// This could be done as a function, but that would bloat a stack trace compared to an inlined macro.
#[macro_export]
macro_rules! assert_vm_eq {
    ( $states:ident, $vms:ident, $index:expr ) => {
        // Registers
        for i in 0..=14 {
            match ($states[$index].r[i]) {
                Some(x) => assert_eq!(x, $vms[$index].external_get_reg(i), "\n\n>>> Register r{}: Expected 0x{}, actually contained 0x{}\n\n", i, format_padded_hex(x), format_padded_hex($vms[$index].external_get_reg(i))),
                None    => (),
            };
        }
        // Negative flag
        match ($states[$index].n) {
            Some(x) => assert_eq!(x, $vms[$index].cpsr.n, "\n\n>>> Condition flag n (Negative): Expected {}, actually contained {}\n\n", x, $vms[$index].cpsr.n),
            None    => (),
        };
        // Zero flag
        match ($states[$index].z) {
            Some(x) => assert_eq!(x, $vms[$index].cpsr.z, "\n\n>>> Condition flag z (Zero): Expected {}, actually contained {}\n\n", x, $vms[$index].cpsr.z),
            None    => (),
        };
        // Carry (Overflow) flag
        match ($states[$index].c) {
            Some(x) => assert_eq!(x, $vms[$index].cpsr.c, "\n\n>>> Condition flag c (Carry/Unsigned Overflow): Expected {}, actually contained {}\n\n", x, $vms[$index].cpsr.c),
            None    => (),
        };
        // V (Signed Overflow) flag
        match ($states[$index].v) {
            Some(x) => assert_eq!(x, $vms[$index].cpsr.v, "\n\n>>> Condition flag v (Signed Overflow): Expected {}, actually contained {}\n\n", x, $vms[$index].cpsr.v),
            None    => (),
        };
        // PC, program counter
        match ($states[$index].pc_address) {
            Some(x) => assert_eq!(x, $vms[$index].get_pc_address(), "\n\n>>> pc: Expected {}, actually contained {}\n\n", x, $vms[$index].get_pc_address()),
            None    => (),
        };
        // Memory, if check_memory_start is set
        if $states[$index].check_memory_start != None {
            let base_address = $states[$index].check_memory_start.unwrap();
            for i in 0..=19 {
                let memory_word = $vms[$index].memory.get_u32(base_address + i*4).unwrap();
                match ($states[$index].memory[i as usize]) {
                    Some(x) => assert_eq!(x, memory_word, "\n\n>>> Memory word {} (address 0x{}): Expected 0x{}, actually contained 0x{}\n\n", i, format_padded_hex(base_address + i*4), format_padded_hex(x), format_padded_hex(memory_word)),
                    None    => (),
                };
            }
        }
    };
}

// Macro to load values in VMState struct into the actual VM's state
// This could be done as a function, but I made the assertion thing above a macro and now it's too late
#[macro_export]
macro_rules! load_into_vm {
    ( $states:ident, $vms:ident, $index:expr ) => {
        // Registers
        for i in 0..=14 {
            match ($states[$index].r[i]) {
                Some(x) => $vms[$index].external_set_reg(i, x),
                None => (),
            };
        }
        // Negative flag
        match ($states[$index].n) {
            Some(x) => $vms[$index].cpsr.n = x,
            None => (),
        };
        // Zero flag
        match ($states[$index].z) {
            Some(x) => $vms[$index].cpsr.z = x,
            None => (),
        };
        // Carry (Overflow) flag
        match ($states[$index].c) {
            Some(x) => $vms[$index].cpsr.c = x,
            None => (),
        };
        // V (Signed Overflow) flag
        match ($states[$index].v) {
            Some(x) => $vms[$index].cpsr.v = x,
            None => (),
        };
        // PC, program counter
        match ($states[$index].pc_address) {
            Some(x) => $vms[$index].set_thumb_pc_address(x),
            None => (),
        };
        // Memory, if check_memory_start is set
        if $states[$index].check_memory_start != None {
            let base_address = $states[$index].check_memory_start.unwrap();
            for i in 0..=19 {
                match ($states[$index].memory[i as usize]) {
                    Some(x) => $vms[$index]
                        .memory
                        .set_u32(base_address + i * 4, x)
                        .unwrap(), // Unwrap here so None doesn't have to return a Result
                    None => 0,
                };
            }
        }
    };
}

// Macro to print values in VMState struct for debug output
// This could be done as a function, but I made the assertion thing above a macro and now it's too late
#[macro_export]
macro_rules! print_vm_state {
    ( $states:ident[$index:expr] ) => {
        // Registers
        for i in 0..=14 {
            match ($states[$index].r[i]) {
                Some(x) => println!("r{}: 0x{}", i, format_padded_hex(x)),
                None => println!("r{}: (Ignored)", i),
            };
        }
        // Negative flag
        match ($states[$index].n) {
            Some(x) => println!("n: {}", x),
            None => println!("n: (Ignored)"),
        };
        // Zero flag
        match ($states[$index].z) {
            Some(x) => println!("z: {}", x),
            None => println!("z: (Ignored)"),
        };
        // Carry (Overflow) flag
        match ($states[$index].c) {
            Some(x) => println!("c: {}", x),
            None => println!("c: (Ignored)"),
        };
        // V (Signed Overflow) flag
        match ($states[$index].v) {
            Some(x) => println!("v: {}", x),
            None => println!("v: (Ignored)"),
        };
        // PC, program counter
        match ($states[$index].pc_address) {
            Some(x) => println!("pc address: 0x{}", format_padded_hex(x)),
            None => println!("pc address: (Ignored)"),
        };
        // Expect execution error?
        println!(
            "expect execution error: {}",
            $states[$index].expect_exec_error
        );
        // Expected svc parameter
        println!(
            "svc parameter: 0x{}",
            format_padded_hex($states[$index].svc_param)
        );
        // Memory, if check_memory_start is set
        if $states[$index].check_memory_start != None {
            let base_address = $states[$index].check_memory_start.unwrap();
            println!(
                "memory starting from address 0x{}:",
                format_padded_hex(base_address)
            );
            for i in 0..=19 {
                match ($states[$index].memory[i as usize]) {
                    Some(x) => println!("memory word {}: 0x{}", i, format_padded_hex(x)),
                    None => println!("memory word {}: (Ignored)", i),
                };
            }
        } else {
            println!("Memory: (Ignored)");
        }
    };
}

/***********************************************
***                                          ***
***   Misc supporting functions and macros   ***
***                                          ***
***********************************************/

// Format u32 to hex string approperiatly padded with zeroes for easy side-by-side comparison
// TODO: Underscores?
// TODO: Replace with build-in formatting macro params used in VM diagnostics print code?
pub fn format_padded_hex(int: u32) -> String {
    let mut string = String::from(format!("{:x}", int));
    while string.len() < 8 {
        string = format!("0{}", string)
    }
    string.to_uppercase()
}

// Functions to easily and clearly define memory addresses
pub fn any_mem_address(base: u32, offset: u32) -> u32 {
    return base + offset;
}

pub fn code_mem_address(offset: u32) -> u32 {
    return ASM_ENTRY + offset + THUMBS_MODE;
}

pub fn mut_mem_address(offset: u32) -> u32 {
    return 0x8000_0000 + offset;
}

pub fn stack_mem_address(offset: u32) -> u32 {
    return STACK_MEM_START + offset;
}

// Macro to reduce boilerplate code when executing VM instance and asserting results
// Note: If you're using stepping without SVC op to execute VM you can't use this macro
#[macro_export]
macro_rules! execute_and_assert {
    ( $states:ident, $vms:ident, $index:expr ) => {
        if $states[$index].expect_exec_error {
            assert!(
                $vms[$index].execute().is_err(),
                "\n\n>>> Execution: Expected error, got none \n\n"
            );
        } else {
            let svc_param = $vms[$index].execute().unwrap();
            assert_eq!(
                svc_param,
                $states[$index].svc_param,
                "\n\n>>> Execution: Expected svc parameter 0x{}, got 0x{} \n\n",
                format_padded_hex($states[$index].svc_param),
                format_padded_hex(svc_param)
            );
        }
        $vms[$index].print_diagnostics();
        assert_vm_eq!($states, $vms, $index);
    };
}

// Macro to reduce boilerplate code when creating a VM with a single op
#[macro_export]
macro_rules! create_vm {
    ( arrays = ($vms:ident, $states:ident), op_id = $index:expr, asm_literal_add_svc = $op:literal ) => {
        println!("\n>>> Creating VM for op variant: {};", OPCODES[$index]);
        println!(">>> Using initial state: \n");
        print_vm_state!($states[$index]);
        println!("\n>>> VM debug output: \n");
        $vms[$index] = create_vm_from_asm(&format!(
            "
                {}
                svc                 #0xFF
                ",
            $op,
        ));
        load_into_vm!($states, $vms, $index);
    };
    ( arrays = ($vms:ident, $states:ident), op_id = $index:expr, asm_literal = $ops:literal ) => {
        println!("\n>>> Creating VM for op variant: {};", OPCODES[$index]);
        println!(">>> Using initial state: \n");
        print_vm_state!($states[$index]);
        println!("\n>>> VM debug output: \n");
        $vms[$index] = create_vm_from_asm($ops);
        load_into_vm!($states, $vms, $index);
    };
    ( arrays = ($vms:ident, $states:ident), op_id = $index:expr, asm_var = $ops_str:ident ) => {
        println!("\n>>> Creating VM for op variant: {};", OPCODES[$index]);
        println!(">>> Using initial state: \n");
        print_vm_state!($states[$index]);
        println!("\n>>> VM debug output: \n");
        $vms[$index] = create_vm_from_asm(&$ops_str);
        load_into_vm!($states, $vms, $index);
    };
}

// Macro to reduce boilerplate code when creating a VM with a single op
#[macro_export]
macro_rules! run_test {
    ( arrays = ($vms:ident, $states:ident), op_ids = $op_id_vec:ident ) => {
        let op_count = $op_id_vec.len();
        println!("\n>>> Running tests for {} op varieties \n", op_count);

        let mut current_op;
        for i in 0..op_count {
            current_op = $op_id_vec[i];
            println!(
                "\n>>> [{}/{}] Testing for op variant: {} \n",
                i + 1,
                op_count,
                OPCODES[current_op]
            );
            execute_and_assert!($states, $vms, current_op);
        }
    };
}

// Macro to reduce boilerplate code when altering all VM state structs in a test
#[macro_export]
macro_rules! set_for_all {
    ( $states:ident[$op_id_vec:ident].r[$index:expr] = Some($value:expr) ) => {
        let iter = $op_id_vec.iter().copied();
        for i in iter {
            $states[i].r[$index] = Some($value);
        }
    };
    ( $states:ident[$op_id_vec:ident].r[$index:expr] = None ) => {
        let iter = $op_id_vec.iter().copied();
        for i in iter {
            $states[i].r[$index] = None;
        }
    };
    ( $states:ident[$op_id_vec:ident].memory[$index:expr] = Some($value:expr) ) => {
        let iter = $op_id_vec.iter().copied();
        for i in iter {
            $states[i].memory[$index] = Some($value);
        }
    };
    ( $states:ident[$op_id_vec:ident].memory[$index:expr] = None ) => {
        let iter = $op_id_vec.iter().copied();
        for i in iter {
            $states[i].memory[$index] = None;
        }
    };
    ( $states:ident[$op_id_vec:ident].$target:ident = Some($value:expr) ) => {
        let iter = $op_id_vec.iter().copied();
        for i in iter {
            $states[i].$target = Some($value);
        }
    };
    ( $states:ident[$op_id_vec:ident].$target:ident = $value:expr ) => {
        let iter = $op_id_vec.iter().copied();
        for i in iter {
            $states[i].$target = $value;
        }
    };
    ( $states:ident[$op_id_vec:ident].$target:ident = None ) => {
        let iter = $op_id_vec.iter().copied();
        for i in iter {
            $states[i].$target = None;
        }
    };
}
