extern crate elf;

use narm::narmvm::*;


const DEFAULT_GAS:u64 = 10000;

#[cfg(test)]
pub fn create_vm_from_asm(assembly_code: &str) -> NarmVM{
    let file = asm(assembly_code);

    let text_scn = file.get_section(".text").unwrap();
    assert!(text_scn.shdr.addr == 0x1_0000);
    assert!(text_scn.data.len() < 0x1_0000);

    let mut vm = NarmVM::default();
    vm.memory.add_memory(0x1_0000, 0x1_0000).unwrap();
    vm.copy_into_memory(0x1_0000, &text_scn.data).unwrap();
    vm.set_pc(0x1_0000);
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
    println!("asm: {}\n---------------", asm);

    let mut f1 = std::fs::File::create(&input).unwrap();
    writeln!(f1,"{}", asm).unwrap();
    f1.flush().unwrap();


    let mut f2 = std::fs::File::create(&linkfile).unwrap();
    writeln!(f2,"{}", linkerscript).unwrap();
    f2.flush().unwrap();

    // The following commands will be executed
    // arm-none-eabi-as -march=armv6s-m -o$OBJECT $INPUT
    // arm-none-eabi-ld -T link.ld -o$OUTPUT $OBJECTF
    
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

    elf::File::open_path(output).unwrap()
}

// Macro to assert values of all condition flags
#[macro_export]
macro_rules! assert_flags_nzcv {
    (
        $vm:ident, 
        $n:expr, 
        $z:expr, 
        $c:expr, 
        $v:expr
    ) => {
        assert_eq!($vm.cpsr.n, $n);
        assert_eq!($vm.cpsr.z, $z);
        assert_eq!($vm.cpsr.c, $c);
        assert_eq!($vm.cpsr.v, $v);
    };
}

// Macro to assert values of all low registers
#[macro_export]
macro_rules! assert_lo_regs_all {
    (
        $vm:ident, 
        $lo0:expr, 
        $lo1:expr, 
        $lo2:expr, 
        $lo3:expr, 
        $lo4:expr, 
        $lo5:expr, 
        $lo6:expr, 
        $lo7:expr
    ) => {
        assert_eq!($vm.external_get_reg(0), $lo0);
        assert_eq!($vm.external_get_reg(1), $lo1);
        assert_eq!($vm.external_get_reg(2), $lo2);
        assert_eq!($vm.external_get_reg(3), $lo3);
        assert_eq!($vm.external_get_reg(4), $lo4);
        assert_eq!($vm.external_get_reg(5), $lo5);
        assert_eq!($vm.external_get_reg(6), $lo6);
        assert_eq!($vm.external_get_reg(7), $lo7);
    };
}

// Helper macro that allows fever arguments by assuming omitted values as zero
#[macro_export]
macro_rules! assert_lo_regs {
    ($vm:ident, $lo0:expr, $lo1:expr, $lo2:expr, $lo3:expr, $lo4:expr, $lo5:expr, $lo6:expr, $lo7:expr) 
    => { assert_lo_regs_all!($vm, $lo0, $lo1, $lo2, $lo3, $lo4, $lo5, $lo6, $lo7) };

    ($vm:ident, $lo0:expr, $lo1:expr, $lo2:expr, $lo3:expr, $lo4:expr, $lo5:expr, $lo6:expr) 
    => { assert_lo_regs_all!($vm, $lo0, $lo1, $lo2, $lo3, $lo4, $lo5, $lo6, 0) };

    ($vm:ident, $lo0:expr, $lo1:expr, $lo2:expr, $lo3:expr, $lo4:expr, $lo5:expr) 
    => { assert_lo_regs_all!($vm, $lo0, $lo1, $lo2, $lo3, $lo4, $lo5, 0, 0) };

    ($vm:ident, $lo0:expr, $lo1:expr, $lo2:expr, $lo3:expr, $lo4:expr) 
    => { assert_lo_regs_all!($vm, $lo0, $lo1, $lo2, $lo3, $lo4, 0, 0, 0) };

    ($vm:ident, $lo0:expr, $lo1:expr, $lo2:expr, $lo3:expr) 
    => { assert_lo_regs_all!($vm, $lo0, $lo1, $lo2, $lo3, 0, 0, 0, 0) };

    ($vm:ident, $lo0:expr, $lo1:expr, $lo2:expr) 
    => { assert_lo_regs_all!($vm, $lo0, $lo1, $lo2, 0, 0, 0, 0, 0) };

    ($vm:ident, $lo0:expr, $lo1:expr) 
    => { assert_lo_regs_all!($vm, $lo0, $lo1, 0, 0, 0, 0, 0, 0) };

    ($vm:ident, $lo0:expr) 
    => { assert_lo_regs_all!($vm, $lo0, 0, 0, 0, 0, 0, 0, 0) };

    ($vm:ident) 
    => { assert_lo_regs_all!($vm, 0, 0, 0, 0, 0, 0, 0, 0) };
}



// Below are structs used to define a VM state that can be checked against test results



// These two are helper structures
#[allow(dead_code)]
pub struct Register {
    pub assert: bool, 
    pub value: u32, 
}

#[allow(dead_code)]
pub struct CondFlag {
    pub assert: bool, 
    pub value: bool, 
}

// This is the primary structure used 
// TODO: Handle special registers differently?
// TODO: Implement memory area assertion? Maybe too advanced?
#[allow(dead_code)]
pub struct VmState {
    pub r0:  Register,
    pub r1:  Register,
    pub r2:  Register,
    pub r3:  Register,
    pub r4:  Register,
    pub r5:  Register,
    pub r6:  Register,
    pub r7:  Register,
    pub r8:  Register,
    pub r9:  Register,
    pub r10: Register,
    pub r11: Register,
    pub r12: Register,
    pub r13: Register,
    pub r14: Register,
    pub r15: Register, // PC
    pub n:   CondFlag,
    pub z:   CondFlag,
    pub c:   CondFlag,
    pub v:   CondFlag,
}

// This implementation is used to actually check constructed state against VM state
#[allow(dead_code)]
impl VmState {
    pub fn assert(&self, vm: NarmVM) -> () {
        let reg_vals = self.reg_val_array();
        let reg_asserts = self.reg_assert_array();
        for i in 0..=15 {
            if reg_asserts[i] {
                assert_eq!(vm.external_get_reg(i), reg_vals[i]);
            }
        }
        if self.n.assert {
            assert_eq!(vm.cpsr.n, self.n.value);
        }
        if self.z.assert {
            assert_eq!(vm.cpsr.z, self.z.value);
        }
        if self.c.assert {
            assert_eq!(vm.cpsr.c, self.c.value);
        }
        if self.v.assert {
            assert_eq!(vm.cpsr.v, self.v.value);
        }
    }

    fn reg_val_array(&self) -> [u32;16] {
        [
            self.r0.value, 
            self.r1.value, 
            self.r2.value, 
            self.r3.value, 
            self.r4.value, 
            self.r5.value, 
            self.r6.value, 
            self.r7.value, 
            self.r8.value, 
            self.r9.value, 
            self.r10.value, 
            self.r11.value, 
            self.r12.value, 
            self.r13.value, 
            self.r14.value, 
            self.r15.value, 
        ]
    }
    fn reg_assert_array(&self) -> [bool;16] {
        [
            self.r0.assert, 
            self.r1.assert, 
            self.r2.assert, 
            self.r3.assert, 
            self.r4.assert, 
            self.r5.assert, 
            self.r6.assert, 
            self.r7.assert, 
            self.r8.assert, 
            self.r9.assert, 
            self.r10.assert, 
            self.r11.assert, 
            self.r12.assert, 
            self.r13.assert, 
            self.r14.assert, 
            self.r15.assert, 
        ]
    }
}

// Default values for VmState
#[allow(dead_code)]
pub const DEFAULT_VMSTATE: VmState<> = VmState {
    r0:  Register { assert: true, value: 0, },
    r1:  Register { assert: true, value: 0, },
    r2:  Register { assert: true, value: 0, },
    r3:  Register { assert: true, value: 0, },
    r4:  Register { assert: true, value: 0, },
    r5:  Register { assert: true, value: 0, },
    r6:  Register { assert: true, value: 0, },
    r7:  Register { assert: true, value: 0, },
    r8:  Register { assert: true, value: 0, },
    r9:  Register { assert: true, value: 0, },
    r10: Register { assert: true, value: 0, },
    r11: Register { assert: true, value: 0, },
    r12: Register { assert: true, value: 0, },
    r13: Register { assert: true, value: 0, },
    r14: Register { assert: true, value: 0, },
    r15: Register { assert: false, value: 0, }, // PC
    n:   CondFlag { assert: true, value: false, },
    z:   CondFlag { assert: true, value: false, },
    c:   CondFlag { assert: true, value: false, },
    v:   CondFlag { assert: true, value: false, },
};

