use core::fmt;

use bitflags::bitflags;

use crate::{memory::MemoryBus, register::RegisterSet};

use super::{DataProccessingInstruction, MultiplyInstruction};

#[derive(Debug)]
pub enum InstructionError {
    InvalidOpcode(u8),
    InvalidInstruction(u32),
    InvalidRegister(u32),
    InvalidArgument(String),
    RegisterReadError(String),
    RegisterWriteError(String),
    InvalidCPSR(),
}

pub trait Instruction {
    fn execute(&mut self, register_set: &RegisterSet, memory_bus: &MemoryBus) -> Result<(), InstructionError>;
}

pub trait DecodeInstruction {
    fn decode(value: u32) -> Result<Self, InstructionError>
    where
        Self: Sized;
}

bitflags! {
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct Condition: u8 {
        const EQ = 0b0000; // Equal (Z=1)
        const NE = 0b0001; // Not Equal (Z=0)
        const CS = 0b0010; // Unsigned higher or same (C=1)
        const CC = 0b0011; // Unsigned Lower (C=0)
        const MI = 0b0100; // Minus/Signed Negative (N=1)
        const PL = 0b0101; // Plus/Positive or Zero (N=0)
        const VS = 0b0110; // Signed Overflow (V=1)
        const VC = 0b0111; // Signed No Overflow (V=0)
        const HI = 0b1000; // Unsigned Higher (C=1 and Z=0)
        const LS = 0b1001; // Unsigned Lower or Same (C=0 or Z=1)
        const GE = 0b1010; // Signed Greater Than or Equal (N=V)
        const LT = 0b1011; // Signed Less Than (N!=V)
        const GT = 0b1100; // Signed Greater Than (Z=0 and N=V)
        const LE = 0b1101; // Signed Less Than or Equal (Z=1 or N!=V)
        const AL = 0b1110; // Always (Unconditional)
        const NV = 0b1111; // Never
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.bits() {
            0b0000 => write!(f, "EQ")?,
            0b0001 => write!(f, "NE")?,
            0b0010 => write!(f, "CS")?,
            0b0011 => write!(f, "CC")?,
            0b0100 => write!(f, "MI")?,
            0b0101 => write!(f, "PL")?,
            0b0110 => write!(f, "VS")?,
            0b0111 => write!(f, "VC")?,
            0b1000 => write!(f, "HI")?,
            0b1001 => write!(f, "LS")?,
            0b1010 => write!(f, "GE")?,
            0b1011 => write!(f, "LT")?,
            0b1100 => write!(f, "GT")?,
            0b1101 => write!(f, "LE")?,
            0b1110 => write!(f, "AL")?,
            0b1111 => write!(f, "NV")?,
            _ => write!(f, "Unknown")?,
        }
        Ok(())
    }
}

pub enum InstructionType {
    Multiply(MultiplyInstruction),
    DataProcessing(DataProccessingInstruction),
}

pub fn get_s_flag(value: u32) -> bool {
    (value & (1 << 20)) != 0
}

pub fn is_data_processing_instruction(value: u32) -> bool {
    let bits_27_26 = (value >> 26) & 0b11;
    bits_27_26 == 0b00
}

pub fn is_multiply_instruction(value: u32) -> bool {
    let bits_27_25 = (value >> 25) & 0b111;
    bits_27_25 == 0b000
}

impl Instruction for InstructionType {
    fn execute(&mut self, register_set: &RegisterSet, memory_bus: &MemoryBus) -> Result<(), InstructionError> {
        match self {
            InstructionType::Multiply(multiply_instruction) => todo!(),
            InstructionType::DataProcessing(data_proccessing_instruction) => data_proccessing_instruction.execute(register_set, memory_bus),
        }
    }
}

pub fn get_instruction(value: u32) -> Result<InstructionType, InstructionError> {
    // bits 27-25
    let bits_27_25 = (value >> 25) & 0b111;

    // bits 24-21
    let bits_24_21 = ((value >> 21) & 0b1111) as u8;

    // check if it's a multiply instruction
    if bits_27_25 == 0b000 {
        // for multiply instructions, bits 24-21 must be in range 0x0 to 0xB
        if bits_24_21 <= 0xB {
            match MultiplyInstruction::decode(value) {
                Ok(instruction) => return Ok(InstructionType::Multiply(instruction)),
                Err(e) => return Err(e),
            }
        } else {
            return Err(InstructionError::InvalidOpcode(bits_24_21));
        }
    }

    let bits_27_26 = (value >> 26) & 0b11;
    if bits_27_26 == 0b00 {
        match DataProccessingInstruction::decode(value) {
            Ok(instruction) => return Ok(InstructionType::DataProcessing(instruction)),
            Err(e) => return Err(e),
        }
    }

    Err(InstructionError::InvalidInstruction(value))
}

pub fn execute(value: u32, register_set: &RegisterSet, memory_bus: &MemoryBus) -> Result<(), InstructionError> {
    get_instruction(value)?.execute(register_set, memory_bus)
}