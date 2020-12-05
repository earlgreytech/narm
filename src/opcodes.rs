
//argument masks
pub const MASK_R3_R3:u16        = 0b0000_0000_0011_1111;
pub const MASK_R3_R3_R3:u16     = 0b0000_0001_1111_1111;
pub const MASK_D1_R4_R3:u16     = 0b0000_0000_1111_1111;
pub const MASK_R3_IMM8:u16      = 0b0000_0111_1111_1111;
pub const MASK_IMM7:u16         = 0b0000_0000_0111_1111;
pub const MASK_IMM8:u16         = 0b0000_0000_1111_1111;
pub const MASK_IMM5_R3_R3:u16   = 0b0000_0111_1111_1111;
pub const MASK_C4_IMM8:u16      = 0b0000_1111_1111_1111;
pub const MASK_R4_Q3:u16        = 0b0000_0000_0111_1111;
pub const MASK_X1_RL8:u16       = 0b0000_0001_1111_1111;
pub const MASK_NONE:u16         = 0b0000_0000_0000_0000;

pub fn is_32bit_opcode(opcode: u16) -> bool{
    if (opcode & 0b1110_0000_0000_0000) != 0b1110_0000_0000_0000 {
        return false;
    }
    opcode & 0b0001_1000_0000_0000 != 0
}

#[derive(Default)]
pub struct Arguments{
    pub arg0: u32,
    pub arg1: u32,
    pub arg2: u32
}

pub fn decode_r3_r3(opcode: u16) -> Arguments{
    let a0 = opcode & 0b0000_0000_0011_1000 >> 3;
    let a1 = opcode & 0b0000_0000_0000_0111;
    Arguments{
        arg0: a0 as u32,
        arg1: a1 as u32,
        arg2: 0
    }
}

pub fn decode_r3_imm8(opcode: u16) -> Arguments{
    let mut a = Arguments::default();
    a.arg0 = (opcode & 0b0000_0111_0000_0000 >> 8) as u32;
    a.arg1 = (opcode & 0b0000_0000_1111_1111) as u32;
    a
}



