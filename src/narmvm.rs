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

        if is_32bit_opcode(opcode){
            
            let opcode32 = ((opcode as u32) << 16) | (self.memory.get_u16(self.pc)? as u32);
            self.pc += 2;
            let opcode32 = opcode32 & !MASK32_X1_IMM10_X1_X1_IMM11;
            let (s, imm1, j1, j2, imm2) = decode32_x1_imm10_x1_x1_imm11(opcode32);
            //BL T1, 32bit instruction. J is split into J1 and J2. x, y, and z is combined into one argument using all of the arguments together which control sign extension etc. Allows -16777216 to +16777214
                           //1111_0xyy_yyyy_yyyy_11J1_Jzzz_zzzz_zzzz 
            if opcode32 == 0b1111_0000_0000_0000_1101_0000_0000_0000{
                //I1 = NOT(J1 EOR S);  I2 = NOT(J2 EOR S);  imm32 = SignExtend(S:I1:I2:imm10:imm11:'0', 32);
                let s1 = s as u32;
                let i1 = (!(j1 ^ s)) as u32;
                let i2 = (!(j2 ^ s)) as u32;
                let value = 
                    s1      << 24 |  //1 bit (1+1+10+11+1)
                    i1      << 23 |  //1 bit (1+10+11+1)
                    i2      << 22 |  //1 bit (10+11+1)
                    imm1    << 12 | //10 bits (11+1)
                    imm2    << 1;   //11 bits, bottom bit is 0
                //25 bits total length
                let imm32 = sign_extend32(value, 25);
                let lr = LongRegister{register: 14};
                self.set_reg(&lr, self.virtual_pc | 1);
                self.set_pc((self.virtual_pc as i32).wrapping_add(imm32) as u32);
                return Ok(0);
            }else{
                //later support MSR/MRS?
                return Err(NarmError::InvalidOpcode32(opcode32));
            }
        }

        //NOP pattern
        {
            let op = opcode & !MASK_NOP;
            //1011_1111_1QQQ_QQQQ NOP HINT catch all (can be safely treated as imm8
            //1011_1111_0000_0000 NOP T1
            //1011_1111_0100_0000 SEV nop
            //1011_1111_0010_0000 WFE T1 nop
            //1011_1111_0011_0000 WFI T1 nop
            //1011_1111_0001_0000 YIELD T1 nop
            if op == 0b1011_1111_0000_0000{
                return Ok(0);
            }
            //1101_1110_QQQQ_QQQQ UDF error T1, causes error either way
            if op == 0b1101_1110_0000_0000{
                return Err(NarmError::InvalidOpcode(op));
            }
        }

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
                    self.sreg[reg] = self.virtual_pc.align4() + ((imm as u32) << 2);
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
                    self.pc = (self.virtual_pc as i32 + label) as u32;
                    return Ok(0);
                }
                _ => {}
            }
        }

        //opcodes for r3_r3
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
                    //NOTE: this shares the same identifying "mask" as LSL imm T1.
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

        //opcodes for r3_r3_r3 and imm3_r3_r3
        {
            let op = opcode & !MASK_R3_R3_R3;
            let (reg1, reg2, reg3) = decode_r3_r3_r3(opcode);
            let valuem = self.sreg[reg1];
            let valuen= self.sreg[reg2];
            //reg3 is almost always destination register
            match op{
                //0001_100x_xxyy_yzzz ADDS reg T1 flags
                0b0001_1000_0000_0000 => {
                    self.sreg[reg3] = self.op_add(valuen, valuem, false, true);
                    return Ok(0);
                },
                //0101_100x_xxyy_yzzz LDR reg T1
                0b0101_1000_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.sreg[reg3] = self.memory.get_u32(address)?;
                    return Ok(0);
                },
                //0101_110x_xxyy_yzzz LDRB reg T1
                0b0101_1100_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.sreg[reg3] = self.memory.get_u8(address)? as u32;
                    return Ok(0);
                },
                //0101_101x_xxyy_yzzz LDRH reg T1
                0b0101_1010_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.sreg[reg3] = self.memory.get_u16(address)? as u32;
                    return Ok(0);
                },
                //0101_011x_xxyy_yzzz LDRSB reg T1
                0b0101_0110_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.sreg[reg3] = self.memory.get_u8(address)? as i8 as i32 as u32;
                    return Ok(0);
                },
                //0101_111x_xxyy_yzzz LDRSH reg T1
                0b0101_1110_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.sreg[reg3] = self.memory.get_u16(address)? as i16 as i32 as u32;
                    return Ok(0);
                },
                //0101_000x_xxyy_yzzz STR reg T1
                0b0101_0000_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.memory.set_u32(address, self.sreg[reg3])?;
                    return Ok(0);
                },
                //0101_010x_xxyy_yzzz STRB reg T1
                0b0101_0100_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.memory.set_u8(address, (self.sreg[reg3] & 0xFF) as u8)?;
                    return Ok(0);
                },
                //0101_001x_xxyy_yzzz STRH reg T1
                0b0101_0010_0000_0000 => {
                    let address = valuen.wrapping_add(valuem);
                    self.memory.set_u16(address, (self.sreg[reg3] & 0xFFFF) as u16)?;
                    return Ok(0);
                },
                //0001_101x_xxyy_yzzz SUBS reg T1 flags
                0b0001_1010_0000_0000 => {
                    self.sreg[reg3] = self.op_add(valuen, !valuem, true, true);
                    return Ok(0);
                },
                //0001_111x_xxyy_yzzz SUBS imm T1 flags
                0b0001_1110_0000_0000 => {
                    let imm3 = reg1 as u32;
                    self.sreg[reg3] = self.op_add(self.sreg[reg2], !imm3, true, true);
                    return Ok(0);
                },
                //0001_110x_xxyy_yzzz ADD imm T1 flags
                0b0001_1100_0000_0000 => {
                    let imm3 = reg1 as u32;
                    self.sreg[reg3] = self.op_add(self.sreg[reg2], imm3, false, true);
                    return Ok(0);
                },
                _ => {}
            }
        }

        //n1_r4_rn3
        {
            let op = opcode & !MASK_N1_R4_RN3;
            let (reg1, reg2) = decode_n1_r4_rn3(opcode);
            match op{
                //0100_0101_xyyy_yzzz CMP reg T2 
                0b0100_0101_0000_0000 => {
                    //Either register being from PC (r15) is considered unpredictable by ARM architecture. To prevent weirdness with later upgrades, this will be forced to 0
                    let rm = if reg1.register == 15{
                        0
                    }else{
                        self.get_reg(&reg1)
                    };
                    let rn = if reg2.register == 15{
                        0
                    }else{
                        self.get_reg(&reg2)
                    };
                    self.op_add(rn, !rm, true, true);
                    return Ok(0);
                },
                //0100_0100_xyyy_yzzz ADD reg T2 noflags
                //0100_0100_x110_1yyy ADD sp+reg T1 noflags (2nd arg must be 1101) -PSUEDO
                //0100_0100_1xxx_x101 ADD sp+reg T2 noflags (1st and 3rd args form 1101) -PSUEDO
                0b0100_0100_0000_0000 => {
                    //note! order of deciding to deal with reg2 vs reg1 being equal to 13 is critical!
                    //if reg2 is 13, then the T1 encoding logic must be used
                    if reg1.register == 13{
                        //sp+reg T1
                        //ADD <Rdm>, SP, <Rdm>
                        let rm = self.get_reg(&reg2);
                        let sp = self.get_reg(&reg1);
                        let result = self.op_add(sp, rm, false, false);
                        self.set_reg(&reg2, result);
                    }else if reg2.register == 13{
                        //sp+reg T2
                        //ADD SP,<Rm>
                        let rm = self.get_reg(&reg1);
                        let sp = self.get_reg(&reg2);
                        let result = self.op_add(sp, rm, false, false);
                        self.set_reg(&reg2, result);
                    }else{
                        if reg1.register == 15 && reg2.register == 15{
                            //listed as UNPREDICTABLE, so just exit here
                            return Ok(0);
                        }
                        let rm = self.get_reg(&reg1);
                        let rn = self.get_reg(&reg2);
                        let result = self.op_add(rn, rm, false, false);
                        self.set_reg(&reg2, result);
                    }
                    return Ok(0);
                },
                _ => {}
            }
        }

        //imm5_r3_r3
        {
            let op = opcode & !MASK_IMM5_R3_R3;
            let (imm, reg1, reg2) = decode_imm5_r3_r3(opcode);
            match op{
                //0001_0xxx_xxyy_yzzz ASR imm T1 flags
                0b0001_0000_0000_0000 => {
                    //shift_t = SRType_ASR; shift_n = if imm5 == '00000' then 32 else UInt(imm5);
                    let shift = if imm == 0{
                        32
                    }else{
                        imm
                    };
                    let (result, carry) = (self.sreg[reg1] as i32).overflowing_shr(shift);
                    self.sreg[reg2] = result as u32;
                    self.set_result_flags(result as u32);
                    self.cpsr.c = carry;
                    return Ok(0); 
                },
                //0110_1xxx_xxyy_yzzz LDR imm T1
                0b0110_1000_0000_0000 => {
                    let imm = imm << 2;
                    let address = self.sreg[reg1].wrapping_add(imm);
                    self.sreg[reg2] = self.memory.get_u32(address)?;
                    return Ok(0);
                },
                //0111_1xxx_xxyy_yzzz LDRB imm T1
                0b0111_1000_0000_0000 => {
                    let address = self.sreg[reg1].wrapping_add(imm);
                    self.sreg[reg2] = self.memory.get_u8(address)? as u32;
                    return Ok(0); 
                }
                //1000_1xxx_xxyy_yzzz LDRH imm T1
                0b1000_1000_0000_0000 => {
                    let imm = imm << 1;
                    let address = self.sreg[reg1].wrapping_add(imm);
                    self.sreg[reg2] = self.memory.get_u16(address)? as u32;
                    return Ok(0);   
                }
                //0000_0xxx_xxyy_yzzz LSL imm T1 flags
                0b0000_0000_0000_0000 => {
                    //note this shares the same identifying mask as MOV reg T2
                    //they only conflict if the imm5 argument here is 0. Thus, if imm5 is 0, then defer execution, as the mov instruction should execute instead
                    //this shouldn't ever happen right now, but if the opcode decoding logic were reorganized it could happen in the future, so guard for it now by doing nothing if imm5 is 0
                    if imm != 0{
                        let (result, carry) = self.sreg[reg1].overflowing_shl(imm);
                        self.sreg[reg2] = result;
                        self.set_result_flags(result);
                        self.cpsr.c = carry;
                        return Ok(0);
                    }
                },
                //0000_1xxx_xxyy_yzzz LSR imm T1 flags
                0b0000_1000_0000_0000 => {
                    let imm = if imm == 0{
                        32
                    }else{
                        imm
                    };
                    let (result, carry) = self.sreg[reg1].overflowing_shr(imm);
                    self.sreg[reg2] = result;
                    self.set_result_flags(result);
                    self.cpsr.c = carry; 
                    return Ok(0);
                },
                //0110_0xxx_xxyy_yzzz STR imm T1
                0b0110_0000_0000_0000 => {
                    let imm = imm << 2;
                    let address = self.sreg[reg1].wrapping_add(imm);
                    self.memory.set_u32(address, self.sreg[reg2])?;
                    return Ok(0); 
                },
                //0111_0xxx_xxyy_yzzz STRB imm T1
                0b0111_0000_0000_0000 => {
                    let address = self.sreg[reg1].wrapping_add(imm);
                    self.memory.set_u8(address, (self.sreg[reg2] & 0xFF) as u8)?;
                    return Ok(0); 
                },
                //1000_0xxx_xxyy_yzzz STRH imm T
                0b1000_0000_0000_0000 => {
                    let imm = imm << 1;
                    let address = self.sreg[reg1].wrapping_add(imm);
                    self.memory.set_u16(address, (self.sreg[reg2] & 0xFFFF) as u16)?;
                    return Ok(0); 
                },
                _ => {}
            }
        }
        //B<C> and SVC
        {
            let op = MASK_C4_IMM8;
            //1101_cccc_xxxx_xxxx B<c> T1 (note: if cond == '1110' then UNDEFINED????)
            //1101_1111_xxxx_xxxx SVC T1 (B with condition code 1111)
            if op == 0b1101_0000_0000_0000{
                let (cond, imm) = decode_c4_imm8(opcode);
                if cond == 0b1111{
                    //SVC opcode, so just exit with specified code
                    return Ok(imm);
                }
                let label = sign_extend32(imm, 8);
                self.pc = (self.virtual_pc as i32 + label) as u32;
                return Ok(0);
            }
        }
        //imm5_r3_r3
        {
            let op = opcode & !MASK_X1_RL8;
            let (option, reglist) = decode_x1_rl8(opcode);
            match op{
                //1011_110x_yyyy_yyyy POP T1 (x is if PC should be popped)
                0b1011_1100_0000_0000 => {
                    let mut address = self.get_sp();;
                    let mut count = 0;
                    for i in 0..7{
                        if reglist.get_bit(i){
                            self.sreg[i as usize] = self.memory.get_u32(address)?;
                            address += 4;
                            count += 1;
                        }
                    }
                    if option{
                        //pop PC
                        self.set_pc(self.memory.get_u32(address)?);
                        count += 1;
                    }
                    self.set_sp(self.get_sp() + 4 * count);
                    return Ok(0);
                },
                //1011_010x_yyyy_yyyy PUSH T1 (x is if LR should be pushed)
                0b1011_0100_0000_0000 => {
                    let mut address = self.get_sp() - 4 * reglist.count_ones();
                    if option{
                        address -= 4;
                    }
                    let mut count = 0;
                    for i in 0..7{
                        if reglist.get_bit(i){
                            self.memory.set_u32(address, self.sreg[i as usize])?;
                            address += 4;
                            count += 1;
                        }
                    }
                    if option{
                        //push LR
                        let lr = LongRegister{register: 14};
                        self.set_reg(&lr, self.memory.get_u32(address)?);
                        count += 1;
                    }
                    self.set_sp(self.get_sp() - 4 * count);
                    return Ok(0); 
                },
                _ => {}
            }
        }
        //r4_q3
        {
            let op = opcode & !MASK_R4_Q3;
            let reg = decode_r4_q3(opcode);
            let value = self.get_reg(&reg); 
            match op{
                //0100_0111_0xxx_xLLL BX T1
                0b0100_0111_0000_0000 => {
                    self.set_pc(value);
                    return Ok(0);
                },
                //0100_0111_1xxx_xLLL BLX T1
                0b0100_0111_1000_0000 => {
                    let lr = LongRegister{register: 14};
                    self.set_reg(&lr, (self.virtual_pc - 2) | 1);
                    self.set_pc(value);
                    return Ok(0);
                }
                _ => {}
            }
        }


        Err(NarmError::InvalidOpcode(opcode))
    }
    fn condition_passes(&self, condition: u32) -> bool{
        let c = self.cpsr.c;
        let n = self.cpsr.n;
        let v = self.cpsr.v;
        let z = self.cpsr.z;
        match condition{
            0b0000 => z,
            0b0001 => !z,
            0b0010 => c,
            0b0011 => !c,
            0b0100 => n,
            0b0101 => !n,
            0b0110 => v,
            0b0111 => !v,
            0b1000 => c & !z,
            0b1001 => !c | z,
            0b1010 => n == v,
            0b1011 => n != v,
            0b1100 => !z & (n == v),
            0b1101 => z | (n != v),
            0b1110 => true, //undefined in ARMv6-M? Means ALWAYS otherwise
            0b1111 => false, //means SVC, shouldn't be done here
            _ => false //should never happen
        }
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
    pub fn set_sp(&mut self, value: u32){
        let r = LongRegister{register: 13};
        self.set_reg(&r, value);
    }
    pub fn set_reg(&mut self, reg: &LongRegister, value: u32){
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
    pub fn get_reg(& self, reg: &LongRegister) -> u32{
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



