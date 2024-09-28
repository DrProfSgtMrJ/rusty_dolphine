
#[derive(Debug)]
pub enum RegisterError {
    DuplicateRegister(String),
    DuplicateRegisterName(String),
    InvalidRegister(String),
    InvalidMode(String),
    InvalidCPSR(u32),
}