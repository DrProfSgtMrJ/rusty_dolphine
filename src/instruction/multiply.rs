use core::fmt;

use crate::instruction::{Condition, DecodeInstruction};


#[derive(Debug, Clone)]
pub struct MultiplyInstruction {
    pub condition: Condition,
    // 27-25 must be 000b for this instruction
    pub opcode: MultiplyOpcode, // Bits 24-21
    pub s_flag: bool, // Bit 20 (Set Condition Codes) (0=No, 1=Yes) (Must be 0 for Halfword & UMAAL)
    pub rd: u8, // (RdHi) Bits 19-16 (Destination Register: R0-R14) 
    pub rn: u8, // (RdLo) Bits 15-12 Accumlate Register (R0-R14) (Set to 0000b if unused)
    pub operand: MultiplyOperand, // Bits 11-0 (Operand Register Rm: R0-R14)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplyOpcode {
    MUL,
    MLA,
    UMAAL,
    UMULL,
    UMLAL,
    SMULL,
    SMLAL,
    SMLAXY,
    SMLAWY,
    SMULWY,
    SMLALXY,
    SMULXY,
}

impl fmt::Display for MultiplyOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiplyOpcode::MUL => write!(f, "MUL"),
            MultiplyOpcode::MLA => write!(f, "MLA"),
            MultiplyOpcode::UMAAL => write!(f, "UMAAL"),
            MultiplyOpcode::UMULL => write!(f, "UMULL"),
            MultiplyOpcode::UMLAL => write!(f, "UMLAL"),
            MultiplyOpcode::SMULL => write!(f, "SMULL"),
            MultiplyOpcode::SMLAL => write!(f, "SMLAL"),
            MultiplyOpcode::SMLAXY => write!(f, "SMLAxy"),
            MultiplyOpcode::SMLAWY => write!(f, "SMLAWy"),
            MultiplyOpcode::SMULWY => write!(f, "SMULWy"),
            MultiplyOpcode::SMLALXY => write!(f, "SMLALxy"),
            MultiplyOpcode::SMULXY => write!(f, "SMULxy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplyOperand {
    // Bit 7-4 must be 1001b for these instructions
    NonHalfwordMultiplies,
    HalfwordMultiplies {
        // Bit 7 must be 1 for these instructions
        y: bool, // Bit 6 (Rs Top/Bottom flag) (0=B=Lower 16bit, 1=T=Upper 16bit)
        x: bool, // Bit 5 (Rm Top/Bottom flag) (0=B=Lower 16bit, 1=T=Upper 16bit) (or 0 for SMLAW, or 1 for SMULW)
        // bit 4 must be 0 for these instructions
        rm: u8 // Bits 3-0 (Operand Register Rm: R0-R14)
    }
}

impl DecodeInstruction for MultiplyInstruction {
    fn decode(value: u32) -> Result<Self, crate::instruction::InstructionError>
        where
            Self: Sized {
                todo!()
    }
}