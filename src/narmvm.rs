use crate::memory::*;
use crate::NarmError;
use crate::opcodes::*;


#[derive(Default)]
pub struct NarmVM{
    //general registers. r0-r14 (r15 is PC). Note r13 has special logic around it for keeping the bottom 2 bits cleared
    pub registers: [u32; 15],
    //program counter, although pc belongs in registers, in ARMv6-M it has enough special cases around it, that reading and writing it needs to be done with care and custom logic
    pub pc: u32,
    //APSR register, which contains the 4 logic flags in the top 4 bits (will replace with struct)
    pub apsr: APSR,
    //TBD
    pub gas_remaining: u64,
    //pub charger: GasCharger
    pub memory: MemorySystem
}

#[derive(Default)]
pub struct APSR{
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool
}

impl APSR{
    pub fn get_apsr(&self) -> u32{
        (self.n as u32) << 31 | (self.z as u32) << 30 | (self.c as u32) << 29 | (self.v as u32) << 28
    }
    pub fn set_apsr(&mut self, value: u32){
        self.n = value & (1 << 31) > 0;
        self.z = value & (1 << 30) > 0;
        self.c = value & (1 << 29) > 0;
        self.v = value & (1 << 28) > 0;
    }
}


impl NarmVM{
    //returns either a service call number (from SVC instruction) or an error
    //Note there is no equivalent to x86 "hlt" in ARM
    pub fn execute() -> Result<u32, NarmError>{
        Ok(0)
    }
    pub fn cycle() -> Result<u32, NarmError>{
        
        Ok(0)
    }
}






