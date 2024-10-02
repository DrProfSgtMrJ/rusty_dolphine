
#[derive(Debug)]
pub enum RegisterError {
    DuplicateRegister(u8),
    InvalidRegister(u8),
    InvalidMode(String),
    InvalidCPSR(u32),
    RegisterBorrowError(String),
}