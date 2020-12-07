extern crate strum;
#[macro_use]
extern crate strum_macros;


/// Helper functions used for bit manipulation
mod bitmanip;
pub mod memory;
pub mod narmvm;
mod decode;

#[derive(PartialEq, Debug, Display, Copy, Clone)]
pub enum  NarmError{
    None,
    //unloaded memory means that an unloaded memory area was access
    UnloadedMemoryRead(u32),
    UnloadedMemoryWrite(u32),
    //empty memory means that a loaded memory area was access out of bounds of it's respective area
    EmptyMemoryRead(u32),
    EmptyMemoryWrite(u32),
    //triggered when writing to read only memory
    ReadOnlyMemoryWrite(u32),
    UnalignedMemoryAddition,
    ConflictingMemoryAddition,
    InvalidOpcode(u16)
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
