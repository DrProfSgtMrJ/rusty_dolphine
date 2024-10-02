
pub enum InstructionError {
    InvalidOpcode(u8),
    InvalidInstruction,
    InvalidRegister(u8),
    InvalidArgument(String),
}