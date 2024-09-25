use super::error::MemoryError;

pub trait Read {

    fn read_byte(&self, address: u32, buf: &mut u8) -> Result<(), MemoryError>;

    fn read_bytes(&self, address: u32, size: u32, buf: &mut [u8]) -> Result<(), MemoryError>;

} 