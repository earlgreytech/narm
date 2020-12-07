use crate::memory::*;
use crate::NarmError;
use crate::decode::*;
use crate::bitmanip::*;


#[derive(Default)]
pub struct NarmVM{
    /// "short registers". General registers. r0-r7 
    sreg: [u32; 8],
    /// General registers r8-r14 (r15 is PC). Note r13 has special logic around it for keeping the bottom 2 bits cleared
    /// These registers are separated from the "short" registers for performance reasons, as a significant number of opcodes can only access the short registers which have no logic attached
    long_registers: [u32; 7],
    /// Program counter, although pc belongs in registers, in ARMv6-M it has enough special cases around it, that reading and writing it needs to be done with care and custom logic
    pc: u32,
    /// "Virtual" program counter. This tracks the value of PC which would be used for all opcode purposes (ie, with alignment applied, and 4 added)
    virtual_pc: u32,
    /// "Last" program counter. This points at the instruction which was just executed. Used for error tracking purposes
    last_pc: u32,
    //CPSR register, which contains the 4 logic flags in the top 4 bits
    pub cpsr: CPSR,
    //TBD
    pub gas_remaining: u64,
    //pub charger: GasCharger
    pub memory: MemorySystem
}

#[derive(Default)]
pub struct CPSR{
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool
}

impl CPSR{
    pub fn get_cpsr(&self) -> u32{
        (self.n as u32) << 31 | (self.z as u32) << 30 | (self.c as u32) << 29 | (self.v as u32) << 28
    }
    pub fn set_cpsr(&mut self, value: u32){
        self.n = value & (1 << 31) > 0;
        self.z = value & (1 << 30) > 0;
        self.c = value & (1 << 29) > 0;
        self.v = value & (1 << 28) > 0;
    }
}


impl NarmVM{
    //returns either a service call number (from SVC instruction) or an error
    //Note there is no equivalent to x86 "hlt" in ARM
    pub fn execute(&mut self) -> Result<u32, NarmError>{
        Ok(0)
    }
    pub fn cycle(&mut self) -> Result<u32, NarmError>{
        self.virtual_pc = self.pc.align4() + 4;
        self.last_pc = self.pc;
        let opcode = self.memory.get_u16(self.pc)?;
        self.pc += 2;
        {
            /*
                rd3,imm8
                0011_0xxx_yyyy_yyyy ADDS imm T2 flags
                1010_1xxx_yyyy_yyyy ADD sp+imm T1 noflags
                1010_0xxx_yyyy_yyyy ADR T1
                0010_1xxx_yyyy_yyyy CMP imm T1
                1001_1xxx_yyyy_yyyy LDR imm T2
                0100_1xxx_yyyy_yyyy LDR lit T1
                0010_0xxx_yyyy_yyyy MOV imm T1 noflags
                1001_0xxx_yyyy_yyyy STR imm T2
                0011_1xxx_yyyy_yyyy SUBS imm T2 flags
                rn,reglist: (compatible encoding, but imm8 is treated as reglist)
                1100_1xxx_yyyy_yyyy LDM T1
                1100_0xxx_yyyy_yyyy STM T1
                imm10: (compatible encoding, but rd3 is top bits of an imm10)
                1110_00xx_xxxx_xxxx B T2
            */
            let op = opcode & !MASK_R3_IMM8;
            let (reg, imm) = decode_r3_imm8(opcode);
            match op{
                //0100_1xxx_yyyy_yyyy LDR lit T1
                0b0100_1000_0000_0000 => {
                    /* t = UInt(Rt);  imm32 = ZeroExtend(imm8:'00', 32);  add = TRUE;
                        base = Align(PC,4);
                        address = if add then (base + imm32) else (base - imm32);
                        R[t] = MemU[address,4];
                    */
                    let address = self.virtual_pc + (imm as u32);
                    self.sreg[reg] = self.memory.get_u32(address)?;
                    return Ok(0);
                },
                //0010_0xxx_yyyy_yyyy MOV imm T1 flags
                0b0010_0000_0000_0000 => {
                    println!("opcode: {}, reg: {}, imm: {}", opcode, reg, imm);
                    let imm = imm as u32;
                    self.sreg[reg] = imm;
                    //update flags
                    self.cpsr.z = imm == 0;
                    self.cpsr.n = imm.get_bit(31);
                    //C and V flags unchanged
                    return Ok(0);
                }
                _ => {}
            }
        }
        Err(NarmError::InvalidOpcode(opcode))
    }
    /// This reads the current value of PC according to ARM specification for internal operations
    /// Specifically, pc will be pointing at the current instruction address, plus 4 added, and with the bottom two bits set to 0
    pub fn get_last_pc(&self) -> u32{
        self.last_pc
    }
    pub fn set_pc(&mut self, value: u32){
        self.pc = value;
    }
    pub fn set_reg(&mut self, reg: LongRegister, value: u32){
        let mut final_value = value;
        let reg = reg.register;
        if reg == 13{
            final_value = value.align4(); //special handling of r13/LR
        } else if reg == 15{
            //special handling for r15/PC
            self.pc = value.align4();
            return;
        }
        if reg < 8{
            self.sreg[reg as usize] = final_value;
        }else{
            self.long_registers[reg as usize - 8] = final_value;
        }
    }
    pub fn get_reg(& self, reg: LongRegister) -> u32{
        let reg = reg.register;
        if reg == 15{
            //special handling for r15/PC
            return self.virtual_pc;
        }
        if reg < 8{
            self.sreg[reg as usize]
        }else{
            self.long_registers[reg as usize - 8]
        }
    }
    pub fn external_get_reg(&self, reg: usize) -> u32{
        if reg < 8 {
            return self.sreg[reg];
        }
        if reg < 15 {
            return self.long_registers[reg - 8];
        }
        if reg == 15 {
            return self.pc;
        }
        0
    }
    /// Helper function to simplify copying a set of data into VM memory
    pub fn copy_into_memory(&mut self, address: u32, data: &[u8]) -> Result<(), NarmError>{
        let m = self.memory.get_mut_sized_memory(address, data.len() as u32)?;
        m[0..data.len()].copy_from_slice(data);
        Ok(())
    }
    /// Helper function to simplify copying a set of data out of VM memory
    pub fn copy_from_memory(&mut self, address: u32, size: u32) -> Result<&[u8], NarmError>{
        return Ok(self.memory.get_sized_memory(address, size)?);
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cspr_bits() {
        let mut cpsr = CPSR::default();
        cpsr.n = true;
        assert_eq!(cpsr.get_cpsr(), 0x8000_0000);
    }
}



