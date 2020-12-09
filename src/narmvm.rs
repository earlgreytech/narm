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
        //opcodes for r3_imm8, r3_reglist, and imm11
        {
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
                //1001_1xxx_yyyy_yyyy LDR imm T2
                0b1001_1000_0000_0000 => {
                    let address = self.get_sp() + ((imm as u32) << 2);
                    self.sreg[reg] = self.memory.get_u32(address)?;
                    return Ok(0);
                },
                //0010_0xxx_yyyy_yyyy MOV imm T1 flags
                //0010_0000_1111_0001
                0b0010_0000_0000_0000 => {
                    let imm = imm as u32;
                    self.sreg[reg] = imm;
                    //update flags
                    self.cpsr.z = imm == 0;
                    self.cpsr.n = imm.get_bit(31);
                    //C and V flags unchanged
                    return Ok(0);
                },
                //0011_0xxx_yyyy_yyyy ADDS imm T2 flags
                0b0011_0000_0000_0000 => {
                    self.sreg[reg] = self.op_add(self.sreg[reg], imm as u32, false, true);
                    return Ok(0);
                },
                //1010_1xxx_yyyy_yyyy ADD sp+imm T1 noflags
                0b1010_1000_0000_0000 => {
                    self.sreg[reg] = self.op_add(self.get_sp(), (imm as u32) << 2, false, false);
                    return Ok(0);
                },
                //1010_0xxx_yyyy_yyyy ADR T1
                0b1010_0000_0000_0000 => {
                    self.sreg[reg] = self.last_pc.align4() + ((imm as u32) << 2);
                    return Ok(0);
                },
                //0010_1xxx_yyyy_yyyy CMP imm T1
                0b0010_1000_0000_0000 => {
                    self.op_add(self.sreg[reg], !(imm as u32), true, true); //result is unused
                    return Ok(0);
                },
                //1001_0xxx_yyyy_yyyy STR imm T2
                0b1001_0000_0000_0000 => {
                    let address = self.get_sp() + ((imm as u32) << 2);
                    self.memory.set_u32(address, self.sreg[reg])?;
                    return Ok(0);
                },
                //0011_1xxx_yyyy_yyyy SUBS imm T2 flags
                0b0011_1000_0000_0000 => {
                    self.sreg[reg] = self.op_add(self.sreg[reg], !(imm as u32), true, true);
                    return Ok(0);
                },
                //1100_1xxx_yyyy_yyyy LDM T1
                0b1100_1000_0000_0000 => {
                    let reglist = imm; //imm is actually a reg list here
                    let mut address = self.sreg[reg];
                    let wback = !reglist.get_bit(reg as u8);
                    let mut count = 0;
                    for i in 0..7{
                        if reglist.get_bit(i){
                            self.sreg[i as usize] = self.memory.get_u32(address)?;
                            address += 4;
                            count += 1;
                        }
                    }
                    if wback && !reglist.get_bit(reg as u8) {
                        self.sreg[reg] += 4 * count;
                    }
                    return Ok(0);
                },
                //1100_0xxx_yyyy_yyyy STM T1
                0b1100_0000_0000_0000 => {
                    let reglist = imm; //imm is actually a reg list here
                    let mut address = self.sreg[reg];
                    let mut count = 0;
                    for i in 0..7{
                        if reglist.get_bit(i){
                            //NOTE this does not include the "unknown" unpredictable case:
                            //If the base register is included and not the lowest-numbered register in the list, such an instruction stores an unknown value for the base register. 
                            //Use of <Rn> in the register list is deprecated.
                            self.memory.set_u32(address, self.sreg[i as usize])?;
                            address += 4;
                            count += 1;
                        }
                    }
                    self.sreg[reg] += 4 * count;
                    return Ok(0);
                },
                //1110_0xxx_xxxx_xxxx B T2
                0b1110_0000_0000_0000 => {
                    let label = sign_extend32((((reg as u32) << 8) | (imm as u32)) << 1, 12);
                    self.pc = (self.last_pc as i32 + label) as u32;
                    return Ok(0);
                }
                _ => {}
            }
        }
        {
            let op = opcode & !MASK_R3_R3;
            let (reg1, reg2) = decode_r3_r3(opcode);
            let valuen = self.sreg[reg2];
            let valuem = self.sreg[reg1];

            match op{
                //0100_0001_01xx_xyyy ADC reg T1 flags
                0b0100_0001_0100_0000 => {
                    self.sreg[reg2] = self.op_add(self.sreg[reg1], self.sreg[reg2], self.cpsr.c, true);
                    return Ok(0);
                },
                //0100_0000_00xx_xyyy AND reg T1 flags
                0b0100_0000_0000_0000 => {
                    self.sreg[reg2] = self.sreg[reg2] & self.sreg[reg1];
                    self.cpsr.n = self.sreg[reg2].get_bit(31);
                    self.cpsr.z = self.sreg[reg2] == 0;
                    return Ok(0);
                },
                //0100_0001_00xx_xyyy ASRS reg T1 flags
                0b0100_0001_0000_0000 => {
                    let (result, carry) = (self.sreg[reg2] as i32).overflowing_shr(self.sreg[reg1] & 0xFF);
                    self.sreg[reg2] = result as u32;
                    self.set_result_flags(result as u32);
                    self.cpsr.c = carry;
                    return Ok(0);
                },
                //0100_0011_10xx_xyyy BICS T1 flags
                0b0100_0011_1000_0000 => {
                    let result = self.sreg[reg2] & !self.sreg[reg1];
                    self.sreg[reg2] = result;
                    self.set_result_flags(result);
                    return Ok(0);
                },
                //0100_0010_11xx_xyyy CMN T1 flags
                0b0100_0010_1100_0000 => {
                    let _result = self.op_add(valuen, valuem, false, true);
                    return Ok(0);
                },
                //0100_0010_10xx_xyyy CMP reg T1
                0b0100_0010_1000_0000 => {
                    let _result = self.op_add(valuen, !valuem, true, true);
                    return Ok(0);
                }
                //0100_0000_01xx_xyyy EORS reg T1 flags
                0b0100_0000_0100_0000 => {
                    let result = valuen ^ valuem;
                    self.sreg[reg2] = result;
                    self.set_result_flags(result);
                    return Ok(0);
                },
                //0100_0000_10xx_xyyy LSL reg T1 flags
                0b0100_0000_1000_0000 => {
                    let (result, carry) = valuen.overflowing_shl(valuem & 0xFF);
                    self.sreg[reg2] = result;
                    self.set_result_flags(result);
                    self.cpsr.c = carry;
                    return Ok(0);
                },
                //0100_0000_11xx_xyyy LSR reg T1 flags
                0b0100_0000_1100_0000 => {
                    let (result, carry) = valuen.overflowing_shr(valuem & 0xFF);
                    self.sreg[reg2] = result;
                    self.set_result_flags(result);
                    self.cpsr.c = carry;
                    return Ok(0);
                },
                //0100_0110_ZZxx_xyyy MOV reg T1 noflags (Z is to only access r0-r7 for ARMv6-M)
                0b0100_0110_0000_0000 => {
                    self.sreg[reg2] = valuem;
                    return Ok(0);
                },
                //0000_0000_00xx_xyyy MOVS reg T2 flags
                0b0000_0000_0000_0000 => {
                    self.sreg[reg2] = valuem;
                    self.set_result_flags(valuem);
                    return Ok(0);
                },
                //0100_0011_01xx_xyyy MUL T1 flags
                0b0100_0011_0100_0000 => {
                    self.sreg[reg2] = valuen * valuem;
                    self.set_result_flags(self.sreg[reg2]);
                    return Ok(0);
                },
                //0100_0011_11xx_xyyy MVNS T1 flags
                0b0100_0011_1100_0000 => {
                    self.sreg[reg2] = !valuem;
                    self.set_result_flags(!valuem);
                    return Ok(0);
                },
                //0100_0011_00xx_xyyy ORRS reg T1 flags
                0b0100_0011_0000_0000 => {
                    let result = valuen | valuem;
                    self.sreg[reg2] = result;
                    self.set_result_flags(result);
                    return Ok(0);
                },
                //1011_1010_00xx_xyyy REV T1
                0b1011_1010_0000_0000 => {
                    //Operation is to turn 0x11223344 into 0x44332211
                    let result = 
                        ((valuem & 0xFF) << 24) | 
                        ((valuem & 0xFF00) << 8) |
                        ((valuem & 0xFF0000) >> 8) |
                        ((valuem & 0xFF000000) >> 24);
                    self.sreg[reg2] = result;
                    return Ok(0);
                },
                //1011_1010_01xx_xyyy REV16 T1
                0b1011_1010_0100_0000 => {
                    //operation is to turn 0x11223344 into 0x22114433
                    let result = 
                        ((valuem & 0xFF) << 8) | 
                        ((valuem & 0xFF00) >> 8) |
                        ((valuem & 0xFF0000) << 8) |
                        ((valuem & 0xFF000000) >> 8);
                    self.sreg[reg2] = result;
                    return Ok(0);
                },
                //1011_1010_11xx_xyyy REVSH T1
                0b1011_1010_1100_0000 => {
                    //Byte-Reverse Signed Halfword
                    //Reverses the byte order in the lower 16-bit halfword and sign extends the result to 32btis
                    //Operation: 0x11223344 -> 0x00004433
                    //Operation: 0x1122AAFF -> 0xFFFFFFAA
                    let result = (((valuem & 0xFF) as i8 as i32 as u32) << 8) | ((valuem & 0xFF00) >> 8);
                    self.sreg[reg2] = result;
                    return Ok(0);
                },
                //0100_0001_11xx_xyyy ROR reg T1 flags
                0b0100_0001_1100_0000 => {
                    let shift = valuem % 32;
                    let result = valuen.rotate_right(shift);
                    self.sreg[reg2] = result;
                    //TODO needs live testing to confirm carry behavior, under documented and conflicting sources
                    self.cpsr.c = result & (1 << 31) > 0;
                    self.set_result_flags(result);
                    return Ok(0);
                },
                //0100_0010_01xx_xyyy RSB imm T1 flags (ntoe: imm is forced to 0 for ARMv6-M)
                0b0100_0010_0100_0000 => {
                    self.sreg[reg2] = self.op_add(!valuem, 0, true, true);
                    return Ok(0);
                },
                //0100_0001_10xx_xyyy SBCS T1 flags
                0b0100_0001_1000_0000 => {
                    self.sreg[reg2] = self.op_add(valuen, !valuem, self.cpsr.c, true);
                    return Ok(0);
                },
                //1011_0010_01xx_xyyy SXTB T1
                0b1011_0010_0100_0000 => {
                    self.sreg[reg2] = ((self.sreg[reg1] & 0xFF) as u8 as i8 as i32) as u32;
                    return Ok(0);
                },
                //1011_0010_00xx_xyyy SXTH T1
                0b1011_0010_0000_0000 => {
                    self.sreg[reg2] = ((self.sreg[reg1] & 0xFFFF) as u16 as i16 as i32) as u32;
                    return Ok(0);
                },
                //0100_0010_00xx_xyyy TST reg T1 flags
                0b0100_0010_0000_0000 => {
                    let result = valuen & valuem;
                    //result is not written back
                    self.set_result_flags(result);
                    return Ok(0);
                },
                //1011_0010_11xx_xyyy UXTB T1
                0b1011_0010_1100_0000 => {
                    self.sreg[reg2] = self.sreg[reg1] & 0xFF;
                    return Ok(0);
                },
                //1011_0010_10xx_xyyy UXTH T1
                0b1011_0010_1000_0000 => {
                    self.sreg[reg2] = self.sreg[reg1] & 0xFFFF;
                    return Ok(0);
                },
                _ => {}

            }
        }
        Err(NarmError::InvalidOpcode(opcode))
    }
    fn set_result_flags(&mut self, result: u32){
        self.cpsr.n = result.get_bit(31);
        self.cpsr.z = result == 0;
    }
    fn op_add(&mut self, operand1: u32, operand2: u32, carry_in: bool, set_flags: bool) -> u32{
        let prelim_sum = if carry_in{
            operand1.wrapping_add(1)
        }else{
            operand1
        };
        let (result, carry) = prelim_sum.overflowing_add(operand2);
        if set_flags{
            let (_, overflow) = (prelim_sum as i32).overflowing_add(operand2 as i32);
            self.cpsr.v = overflow;
            self.cpsr.c = carry;
            self.cpsr.n = result.get_bit(31);
            self.cpsr.z = result == 0;
        }
        result
    }

    /// This reads the current value of PC according to ARM specification for internal operations
    /// Specifically, pc will be pointing at the current instruction address, plus 4 added, and with the bottom two bits set to 0
    pub fn get_last_pc(&self) -> u32{
        self.last_pc
    }
    pub fn set_pc(&mut self, value: u32){
        self.pc = value;
    }
    pub fn get_sp(&self) -> u32{
        self.long_registers[13 - 8]
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



