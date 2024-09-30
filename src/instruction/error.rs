
pub enum InstructionError {
    InvalidInstruction,
    InvalidRegister(String),
    InvalidArgument(String),
}