use super::error::InstructionError;

pub trait Instruction {
    fn execute(&mut self) -> Result<(), InstructionError>;
}