

#[derive(Default)]
pub struct NarmVM{
    //general registers. r0-r14 (r15 is PC). Note r13 has special logic around it for keeping the bottom 2 bits cleared
    pub registers: [u32; 15],
    //program counter, although pc belongs in registers, in ARMv6-M it has enough special cases around it, that reading and writing it needs to be done with care and custom logic
    pub pc: u32,
    //APSR register, which contains the 4 logic flags in the top 4 bits (will replace with struct)
    //pub apsr: u32
    //TBD
    pub gas_remaining: u64,
    //pub charger: GasCharger
    //pub memory: MemorySystem
}

