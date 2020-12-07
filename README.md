# narm
Neutron ARM VM experiment 


# Subset

NARM is a subset of the ARMv6-M instruction set. Specifically it skips most system level instructions and includes no co-processor instructions. 

Instruction List

* ADC
* ADD
* ADR
* AND
* ASR
* B
* BIC
* BKPT -- equivalent to NOP
* BL
* BLX (only BLX register form)
* BX
* CMN
* CMP
* CPS -- not supported?
* CPY
* DMB -- not supported?
* DSB -- not supported?
* EOR
* ISB -- not supported?
* LDM -- (also known as LDMIA/LDMFD)
* LDR
* LDRB
* LDRH
* LDRSB
* LDRSH
* LSL
* LSR
* MOV -- encoding T1 shall only support Rm/Rd being r0-r7
* MSR -- See system register behavior
* MRS -- See system register behavior
* MUL
* MVN
* NOP -- includes support for hint instructions
* ORR
* POP
* PUSH
* REV
* REV16
* REVSH
* ROR
* RSB -- also known as `NEG rd, rm, 0`
* SBC
* SEV -- nop
* STM -- also known as STMIA/STMEA
* STR
* STRB
* STRH
* SUB
* SVC -- Used for Neutron Call System operations
* STXB
* SXTH
* TST
* UDF -- generates abort fault
* UXTB
* UXTH
* WFE -- nop
* WFI -- nop
* YIELD -- nop
* 

Note: Cortex-M0 unsupported instructions:
* CBZ
* CBNZ
* IT

Note: Cortex-M0 supported 32bit instructions:
* BL
* DMB
* DSB
* ISB
* MRS
* MSR



System Register behavior

The following registers will be written as NOP and read as 0:

* IAPSR
* EAPSR
* XPSR
* IPSR
* EPSR
* PSP
* PRIMASK
* CONTROL

APSR and MSP are both readable and writeable, as if the program executing was in a privileged state.


Initial Execution State

PC = 0x1_0000
MSP = 0x81000_0000
Other registers = 0
Little endian mode
Thumb instruction set only

Expected Memory map is similar to qx86:

* 0x1_0000 - 0x10_0000 -- code memories (1Mb total)
* 0x8001_0000 - 0x8010_0000 -- unpreserved writeable data memories (1Mb total)
* 0x8100_0000 - 0x8101_0000 -- RAM scratch space (64kb)
* 0x8200_0000 - 0x8200_8000 -- Stack space (32kb)

Instruction patterns:

Codes:
x = first argument
y = second argument
z = third argument
Q = ignored
c = condition code
? = unknown what this does
Z = must be zero in this version
H = should be 1, but not necessarily enforced (unpredictable)
L = should be 0, but not necessarily enforced (unpredictable)


    0100_0001_01xx_xyyy ADC
    0001_110x_xxyy_yzzz ADD imm T1
    0011_0xxx_yyyy_yyyy ADDS imm T2
    0001_100x_xxyy_yzzz ADDS reg T1
    0100_0100_xyyy_yzzz ADD reg T2
    1010_1xxx_yyyy_yyyy ADD sp+imm T1
    1011_0000_0xxx_xxxx ADD sp+imm T2
    0100_0100_x110_1yyy ADD sp+reg T1
    0100_0100_1xxx_x101 ADD sp+reg T1
    1010_0xxx_yyyy_yyyy ADR T1
    0100_0000_00xx_xyyy AND reg T1
    0001_0xxx_xxyy_yzzz ASR imm T1
    0100_0001_00xx_xyyy ASRS reg T1
    1101_cccc_xxxx_xxxx B<c> T1
    1110_00xx_xxxx_xxxx B T2
    0100_0011_10xx_xyyy BICS T1
    1011_1110_QQQQ_QQQQ BKPT
    1111_0xyy_yyyy_yyyy_11J1_Jzzz_zzzz_zzzz BL T1, 32bit instruction. J is split into J1 and J2. x, y, and z is combined into one argument using all of the arguments together which control sign extension etc
    0100_0111_1xxx_xQQQ BLX T1
    0100_0111_0xxx_xQQQ BX T1
    0100_0010_11xx_xyyy CMN T1
    0010_1xxx_yyyy_yyyy CMP imm T1
    0100_0010_10xx_xyyy CMP reg T1
    0100_0101_?xxx_xyyy CMP reg T2
    1111_0011_1011_QQQQ_10Q0_QQQQ_0101_QQQQ DMB
    1111_0011_1011_QQQQ_10Q0_QQQQ_0100_QQQQ DSB
    0100_0000_01xx_xyyy EORS reg T1
    1111_0011_1011_QQQQ_10Q0_QQQQ_0110_QQQQ ISB
    1100_1xxx_yyyy_yyyy LDM T1
    0110_1xxx_xxyy_yzzz LDR imm T1
    1001_1xxx_yyyy_yyyy LDR imm T2
    0100_1xxx_yyyy_yyyy LDR lit T1
    0101_100x_xxyy_yzzz LDR reg T1
    0111_1xxx_xxyy_yzzz LDRB imm T1
    0101_110x_xxyy_yzzz LDRB reg T1
    1000_1xxx_xxyy_yzzz LDRH imm T1
    0101_101x_xxyy_yzzz LDRH reg T1
    0101_011x_xxyy_yzzz LDRSB reg T1
    0101_111x_xxyy_yzzz LDRSH reg T1
    0000_0xxx_xxyy_yzzz LSL imm T1
    0100_0000_10xx_xyyy LSL reg T1
    0000_1xxx_xxyy_yzzz LSR imm T1
    0100_0000_11xx_xyyy LSR reg T1
    0010_0xxx_yyyy_yyyy MOV imm T1
    0100_0110_ZZxx_xyyy MOV reg T1 (Z is to only access r0-r7)
    0000_0000_00xx_xyyy MOVS reg T2
    1111_0011_111Q_QQQQ_10Q0_xxxx_yyyy_yyyy MRS T1
    1111_0011_100Q_xxxx_10Q0_QQQQ_yyyy_yyyy MSR reg T1
    0100_0011_01xx_xyyy MUL T1
    0100_0011_11xx_xyyy MVNS T1
    1011_1111_0000_0000 NOP T1
    0100_0011_00xx_xyyy ORRS reg T1
    1011_110x_yyyy_yyyy POP T1 (x is if PC should be popped)
    1011_010x_yyyy_yyyy PUSH T1 (x is if LR should be pushed)
    1011_1010_00xx_xyyy REV T1
    1011_1010_01xx_xyyy REV16 T1
    1011_1010_11xx_xyyy REVSH T1
    0100_0001_11xx_xyyy ROR reg T1
    0100_0010_01xx_xyyy RSB imm T1
    0100_0001_10xx_xyyy SBCS T1
    1011_1111_0100_0000 SEV nop
    1100_0xxx_yyyy_yyyy STM T1
    0110_0xxx_xxyy_yzzz STR imm T1
    1001_0xxx_yyyy_yyyy STR imm T2
    0101_000x_xxyy_yzzz STR reg T1
    0111_0xxx_xxyy_yzzz STRB imm T1
    0101_010x_xxyy_yzzz STRB reg T1
    1000_0xxx_xxyy_yzzz STRH imm T1
    0101_001x_xxyy_yzzz STRH reg T1
    0001_111x_xxyy_yzzz SUBS imm T1
    0011_1xxx_yyyy_yyyy SUBS imm T2
    0001_101x_xxyy_yzzz SUBS reg T1
    1011_0000_1xxx_xxxx SUB sp-imm T1
    1101_1111_xxxx_xxxx SVC T1
    1011_0010_01xx_xyyy SXTB T1
    1011_0010_00xx_xyyy SXTH T1
    0100_0010_00xx_xyyy TST reg T1
    1101_1110_QQQQ_QQQQ UDF error T1
    1111_0111_1111_QQQQ_1010_QQQQ_QQQQ_QQQQ UDF error T2
    1011_0010_11xx_xyyy UXTB T1
    1011_0010_10xx_xyyy UXTH T1
    1011_1111_0010_0000 WFE T1 nop
    1011_1111_0011_0000 WFI T1 nop
    1011_1111_0001_0000 YIELD T1 nop
    1011_1111_1QQQ_QQQQ NOP HINT catch all



Note: if top 3 bits are 111 and following 2 bits are NOT 00, then 32 bit instruction encoding

PC reads, where allowed, are typically "current instruction + 4" and with bits 1:0 set to 0
PC writes ignores bit[0], treats as 0 (by spec, should be enforced to be 1 but treated as 0, indicating all interworking branches are to thumb code... but we don't need to be strict)

PC loading instructions: 
* ADD reg T2
* ADD sp+reg T1
* B
* BL
* BLX
* BX
* (future) MOV reg T1
* POP



R13 is limited. Bottom 2 bits are always read as zero and when written to are ignored
Can only be used:
* in MOV as source or destination
* using SUB/ADD SP +/- imm/register forms
* R13 can be used as the first operand rm of add sp+reg where rd is not SP
* R13 can be used as the first operand rn in cmp reg 
* R13 can be used as the address in pop/push

Condition codes, counted from 0:
* equal z=1
* not equal z=0
* carry set c=1
* carry clear c=0
* negative n=1
* positive n=0
* overflow v=1
* no overflow v=0
* unsigned higher than c=1 and z=0
* unsigned lower or same c=0 or z=1
* signed greater than or equal n==v
* signed less than n!=v
* signed greater than z=0 and n=v
* signed less than or equal  z=1 or n!=v
* always


Opcodes organized by encoding:

    32bit:
    1111_0011_1011_QQQQ_10Q0_QQQQ_0100_QQQQ DSB supported??
    1111_0011_1011_QQQQ_10Q0_QQQQ_0101_QQQQ DMB supported??
    1111_0011_1011_QQQQ_10Q0_QQQQ_0110_QQQQ ISB supported??
    1111_0011_111L_HHHH_10L0_xxxx_yyyy_yyyy MRS T1
    1111_0011_100L_xxxx_10L0_HLLL_yyyy_yyyy MSR reg T1
    1111_0111_1111_QQQQ_1010_QQQQ_QQQQ_QQQQ UDF error T2
    1111_0xyy_yyyy_yyyy_11J1_Jzzz_zzzz_zzzz BL T1, 32bit instruction. J is split into J1 and J2. x, y, and z is combined into one argument using all of the arguments together which control sign extension etc. Allows -16777216 to +16777214

    rm3,rdn3:
    0100_0001_01xx_xyyy ADC reg T1 flags
    0100_0000_00xx_xyyy AND reg T1 flags
    0100_0001_00xx_xyyy ASRS reg T1 flags
    0100_0011_10xx_xyyy BICS T1 flags
    0100_0010_11xx_xyyy CMN T1 flags
    0100_0010_10xx_xyyy CMP reg T1
    0100_0000_01xx_xyyy EORS reg T1 flags
    0100_0000_10xx_xyyy LSL reg T1 flags
    0100_0000_11xx_xyyy LSR reg T1 flags
    0100_0110_ZZxx_xyyy MOV reg T1 noflags (Z is to only access r0-r7 for ARMv6-M)
    0000_0000_00xx_xyyy MOVS reg T2 flags
    0100_0011_01xx_xyyy MUL T1 flags
    0100_0011_11xx_xyyy MVNS T1 flags
    0100_0011_00xx_xyyy ORRS reg T1 flags
    1011_1010_00xx_xyyy REV T1
    1011_1010_01xx_xyyy REV16 T1
    1011_1010_11xx_xyyy REVSH T1
    0100_0001_11xx_xyyy ROR reg T1 flags
    0100_0010_01xx_xyyy RSB imm T1 flags (ntoe: imm is forced to 0 for ARMv6-M)
    0100_0001_10xx_xyyy SBCS T1 flags
    1011_0010_01xx_xyyy SXTB T1
    1011_0010_00xx_xyyy SXTH T1
    0100_0010_00xx_xyyy TST reg T1 flags
    1011_0010_11xx_xyyy UXTB T1
    1011_0010_10xx_xyyy UXTH T1

    rm3,rn3,rd3:
    0001_100x_xxyy_yzzz ADDS reg T1 flags
    0101_100x_xxyy_yzzz LDR reg T1
    0101_110x_xxyy_yzzz LDRB reg T1
    0101_101x_xxyy_yzzz LDRH reg T1
    0101_011x_xxyy_yzzz LDRSB reg T1
    0101_111x_xxyy_yzzz LDRSH reg T1
    0101_000x_xxyy_yzzz STR reg T1
    0101_010x_xxyy_yzzz STRB reg T1
    0101_001x_xxyy_yzzz STRH reg T1
    0001_101x_xxyy_yzzz SUBS reg T1 flags
    imm3,rn3,rd3: (compatible encoding, but rm3 is an immediate)
    0001_111x_xxyy_yzzz SUBS imm T1 flags
    0001_110x_xxyy_yzzz ADD imm T1 flags
    
    d/n1,rm4,rdn3: (rdn4=dn1:rdn3)
    0100_0101_xyyy_yzzz CMP reg T2 
    0100_0100_xyyy_yzzz ADD reg T2 noflags
    0100_0100_x110_1yyy ADD sp+reg T1 noflags (2nd arg must be 1101) -PSUEDO
    0100_0100_1xxx_x101 ADD sp+reg T2 noflags (1st and 3rd args form 1101) -PSUEDO

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

    imm7:
    1011_0000_0xxx_xxxx ADD sp+imm T2 noflags
    1011_0000_1xxx_xxxx SUB sp-imm T1 noflags
    1011_1111_1QQQ_QQQQ NOP HINT catch all (can be safely treated as imm8)

    imm8:
    1011_1110_QQQQ_QQQQ BKPT, argument ignored
    1101_1110_QQQQ_QQQQ UDF error T1, causes error either way

    imm5,rm3,rd3:
    0001_0xxx_xxyy_yzzz ASR imm T1 flags
    0110_1xxx_xxyy_yzzz LDR imm T1
    0111_1xxx_xxyy_yzzz LDRB imm T1
    1000_1xxx_xxyy_yzzz LDRH imm T1
    0000_0xxx_xxyy_yzzz LSL imm T1 flags
    0000_1xxx_xxyy_yzzz LSR imm T1 flags
    0110_0xxx_xxyy_yzzz STR imm T1
    0111_0xxx_xxyy_yzzz STRB imm T1
    1000_0xxx_xxyy_yzzz STRH imm T1

    <c>imm8:
    1101_cccc_xxxx_xxxx B<c> T1 (note: if cond == '1110' then UNDEFINED????)
    1101_1111_xxxx_xxxx SVC T1 (B with condition code 1111)


    rm4,q3:
    0100_0111_1xxx_xLLL BLX T1
    0100_0111_0xxx_xLLL BX T1

    x,reglist:
    1011_110x_yyyy_yyyy POP T1 (x is if PC should be popped)
    1011_010x_yyyy_yyyy PUSH T1 (x is if LR should be pushed)


    no-arg:
    1011_1111_0000_0000 NOP T1
    1011_1111_0100_0000 SEV nop
    1011_1111_0010_0000 WFE T1 nop
    1011_1111_0011_0000 WFI T1 nop
    1011_1111_0001_0000 YIELD T1 nop


qx86 style of opcode handling:

Defining opcode:

0110_0xxx_xxyy_yzzz STR imm T1
define_opcode(0110_0000_0000_0000b).with_encoding(IMM5_RM3_RD3).calls(str_imm5_rm3_rd3).into_table(&mut opcodes);

Opcode definition:

fn str_imm5_rm3_rd3(vm: &mut VM, pipeline: &Pipeline, _hv: &mut dyn Hypervisor) -> Result<(), VMError>{
    let address = get_sreg(pipeline.arg1) + zero_extend32(pipeline.arg0 << 2);
    vm.write_memory32(address, get_sreg(pipeline.arg2))?;
}

Decoding example:

foreach mask in encoding_masks{
    let masked_opcode = opcode & mask;
    opcode = opcode_table[masked_opcode];
}


alternative simplified approach, without pipelining:

foreach mask in encoding_masks{
    let masked_opcode = opcode & mask;
    let arguments = decode_imm5_r3_r3(opcode);
    match masked_opcode{
        case STR_IMM_T1{
            str_imm5_r3_r3(...)?; //for simple operations, can be done inline
        }
    }
    opcode = opcode_table[masked_opcode];
}


    imm32 = ZeroExtend(imm5:'00', 32);
    offset_addr = (R[n] + imm32);
    address = offset_addr;
    MemU[address,4] = R[t];
