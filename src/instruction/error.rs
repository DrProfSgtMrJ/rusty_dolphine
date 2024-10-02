
pub enum InstructionError {
    InvalidInstruction,
    InvalidRegister(u8),
    InvalidArgument(String),
}