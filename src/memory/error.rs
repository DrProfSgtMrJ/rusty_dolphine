
#[derive(Debug, PartialEq)]
pub enum MemoryError {
    InvalidAddress(u32),
    InvalidAddresses(u32, u32),
    InvalidSize(usize),
    OverlappingMemorySectors(u32),
}