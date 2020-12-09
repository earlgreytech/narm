
//argument masks
pub const MASK_R3_R3:u16        = 0b0000_0000_0011_1111;
pub const MASK_R3_R3_R3:u16     = 0b0000_0001_1111_1111;
pub const MASK_N1_R4_RN3:u16    = 0b0000_0000_1111_1111;
pub const MASK_R3_IMM8:u16      = 0b0000_0111_1111_1111;
pub const MASK_NOP:u16          = 0b0000_0000_1111_1111;
pub const MASK_IMM5_R3_R3:u16   = 0b0000_0111_1111_1111;
pub const MASK_C4_IMM8:u16      = 0b0000_1111_1111_1111;
pub const MASK_R4_Q3:u16        = 0b0000_0000_0111_1111;
pub const MASK_X1_RL8:u16       = 0b0000_0001_1111_1111;

pub const MASK32_X1_IMM10_X1_X1_IMM11:u32 = 0b0000_0111_1111_1111_0010_1111_1111_1111;

/// This specifies a register beyond r0-r7
/// It is not strictly necessary to be organized like this, but used to prevent programmer errors
pub struct LongRegister{
    pub register: usize
}


pub fn is_32bit_opcode(opcode: u16) -> bool{
    if (opcode & 0b1110_0000_0000_0000) != 0b1110_0000_0000_0000 {
        return false;
    }
    opcode & 0b0001_1000_0000_0000 != 0
}


pub fn decode_r3_r3(opcode: u16) -> (usize, usize){
    (((opcode & 0b0000_0000_0011_1000) >> 3) as usize,
        (opcode & 0b0000_0000_0000_0111) as usize)
}

pub fn decode_r3_imm8(opcode: u16) -> (usize, u8){
    (((opcode & 0b0000_0111_0000_0000) >> 8) as usize,
        (opcode & 0b0000_0000_1111_1111) as u8)
}

pub fn decode_r3_r3_r3(opcode: u16) -> (usize, usize, usize){
    (
        ((opcode & 0b0000_0001_1100_0000) >> 6) as usize,
        ((opcode & 0b0000_0000_0011_1000) >> 3) as usize,
        ((opcode & 0b0000_0000_0000_0111)) as usize
    )
}
pub fn decode_n1_r4_rn3(opcode: u16) -> (LongRegister, LongRegister){
    let n =     opcode & 0b0000_0000_1000_0000 >> 7;
    let r4 =    opcode & 0b0000_0000_0111_1000 >> 3;
    let r3 =    opcode & 0b0000_0000_0000_0111;
    let rn3 = (n << 3) | r3;
    (
        LongRegister{register: r4 as usize},
        LongRegister{register: rn3 as usize}
    )
}

pub fn decode_imm5_r3_r3(opcode: u16) -> (u32, usize, usize){
    (
        ((opcode & 0b0000_0111_1100_0000) >> 6) as u32,
        ((opcode & 0b0000_0000_0011_1000) >> 3) as usize,
        ((opcode & 0b0000_0000_0000_0111)) as usize
    )
}

pub fn decode_c4_imm8(opcode: u16) -> (u32, u32){
    (
        ((opcode & 0b0000_1111_0000_0000) >> 8) as u32,
        ((opcode & 0b0000_0000_1111_1111)) as u32,
    ) 
}

pub fn decode_x1_rl8(opcode: u16) -> (bool, u8){
    (
        (opcode & 0b0000_0001_0000_0000) > 0,
        ((opcode & 0b0000_0000_1111_1111)) as u8,
    )
}

pub fn decode_r4_q3(opcode: u16) -> LongRegister{
        LongRegister{register: ((opcode & 0b0000_0000_0111_1000) >> 3) as usize}
}

pub fn decode32_x1_imm10_x1_x1_imm11(opcode: u32) -> (bool, u32, bool, bool, u32){
    (
        ((opcode & 0b0000_0100_0000_0000_0000_0000_0000_0000)) > 0,
        ((opcode & 0b0000_0011_1111_1111_0000_0000_0000_0000) >> 16),
        ((opcode & 0b0000_0000_0000_0000_0010_0000_0000_0000) > 0),
        ((opcode & 0b0000_0000_0000_0000_0000_1000_0000_0000) > 0),
        ((opcode & 0b0000_0000_0000_0000_0000_0111_1111_1111))
    ) 
}