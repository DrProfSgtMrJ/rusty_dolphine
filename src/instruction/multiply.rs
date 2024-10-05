use core::fmt;

use crate::{instruction::{get_s_flag, is_multiply_instruction, Condition, DecodeInstruction}, memory::MemoryBus, register::{ReadRegister, RegisterSet, WriteRegister}};

use super::{Instruction, InstructionError};


#[derive(Debug, Clone)]
pub struct MultiplyInstruction {
    pub condition_bits: u8,
    // 27-25 must be 000b for this instruction
    pub opcode_bits: u8, // Bits 24-21
    pub s_flag: bool, // Bit 20 (Set Condition Codes) (0=No, 1=Yes) (Must be 0 for Halfword & UMAAL)
    pub rd: u8, // (RdHi) Bits 19-16 (Destination Register: R0-R14) 
    pub rn: u8, // (RdLo) Bits 15-12 Accumlate Register (R0-R14) (Set to 0000b if unused)
    pub rs: u8, // Bits 11-8 (Operand Register Rs: R0-R14)
    pub operand: MultiplyOperand, // Bits 11-0 (Operand Register Rm: R0-R14)
}

impl MultiplyInstruction {
    pub fn condition(&self) -> Condition {
        Condition::from_bits_truncate(self.condition_bits)
    }

    pub fn opcode(&self) -> MultiplyOpcode {
        MultiplyOpcode::from(self.opcode_bits)
    }
}

impl fmt::Display for MultiplyInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       // TODO: Implement formatting for MultiplyInstruction
       write!(f, "{}{{{}}}", self.opcode(), self.condition()) 
    }
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
    Invalid,
}

impl From<u8> for MultiplyOpcode {
    fn from(value: u8) -> Self {
       match value {
        0 => MultiplyOpcode::MUL,
        1 => MultiplyOpcode::MLA,
        2 => MultiplyOpcode::UMAAL,
        3 => MultiplyOpcode::UMULL,
        4 => MultiplyOpcode::UMLAL,
        5 => MultiplyOpcode::SMULL,
        6 => MultiplyOpcode::SMLAL,
        7 => MultiplyOpcode::SMLAXY,
        8 => MultiplyOpcode::SMLAWY,
        9 => MultiplyOpcode::SMULWY,
        10 => MultiplyOpcode::SMLALXY,
        11 => MultiplyOpcode::SMULXY,
        _ => MultiplyOpcode::Invalid,
       } 
    }
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
            _ => write!(f, "Invalid"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplyOperand {
    // Bit 7-4 must be 1001b for these instructions
    NonHalfwordMultiplies {
        rm: u8 // Bits 3-0 (Operand Register Rm: R0-R14)
    },
    HalfwordMultiplies {
        // Bit 7 must be 1 for these instructions
        y: bool, // Bit 6 (Rs Top/Bottom flag) (0=B=Lower 16bit, 1=T=Upper 16bit)
        x: bool, // Bit 5 (Rm Top/Bottom flag) (0=B=Lower 16bit, 1=T=Upper 16bit) (or 0 for SMLAW, or 1 for SMULW)
        // bit 4 must be 0 for these instructions
        rm: u8 // Bits 3-0 (Operand Register Rm: R0-R14)
    }
}

impl DecodeInstruction for MultiplyInstruction {
    fn decode(value: u32) -> Result<Self, InstructionError>
        where
            Self: Sized {

        let condition_bits = (value >> 28) as u8;

        // Bits 27-25 must be 000b
        if !is_multiply_instruction(value) {
            return Err(InstructionError::InvalidInstruction(value));
        }

        // Bits 24-21
        let opcode_bits = ((value >> 21) & 0xF) as u8;
        let opcode = MultiplyOpcode::from(opcode_bits);
        if opcode == MultiplyOpcode::Invalid {
            return Err(InstructionError::InvalidOpcode(opcode_bits));
        }

        let s_flag = get_s_flag(value);
        // s_flag must be 0 for Halfword & UMAAL
        if s_flag && opcode == MultiplyOpcode::SMLAL {
            return Err(InstructionError::InvalidInstruction(value));
        }

        let rd: u8 = ((value >> 16) & 0xF) as u8; // or rd_hi
        let rn: u8 = ((value >> 12) & 0xF) as u8; // or rd_lo
        let rs: u8 = ((value >> 8) & 0xF) as u8;

        let operand = if matches!(opcode, MultiplyOpcode::SMLAXY | MultiplyOpcode::SMLALXY | MultiplyOpcode::SMLAWY | MultiplyOpcode::SMULWY | MultiplyOpcode::SMULXY) {
            // check bit 7 - must be 1 for these instructions   
            if ((value >> 7) & 0x1) != 1 {
                return Err(InstructionError::InvalidInstruction(value));
            }

            let y = ((value >> 6) & 0x1) != 0;
            let x = ((value >> 5) & 0x1) != 0;

            // bit 0-3
            let rm = (value  & 0xF) as u8;

            // my be 0 for Halfword
            if s_flag {
                return Err(InstructionError::InvalidInstruction(value));
            }
            MultiplyOperand::HalfwordMultiplies { y, x , rm }
        } else {
            // check bits 7-4 - must be 1001b for these instructions
            if ((value >> 4) & 0xF) != 0b1001 {
                return Err(InstructionError::InvalidInstruction(value));
            }
            // bit 0-3
            let rm = (value  & 0xF) as u8;
            MultiplyOperand::NonHalfwordMultiplies { rm }
        };

        Ok(MultiplyInstruction {
            condition_bits,
            opcode_bits,
            s_flag,
            rd,
            rn,
            rs,
            operand
        })
    }
}

impl Instruction for MultiplyInstruction {
    fn execute(&mut self, register_set: RegisterSet, memory_bus: MemoryBus) -> Result<(), InstructionError> {
        let mut rd_cell = register_set.get(self.rd).ok_or(InstructionError::InvalidRegister(self.rd as u32))?;
        let rn_cell = register_set.get(self.rn).ok_or(InstructionError::InvalidRegister(self.rn as u32))?;
        let rs_cell = register_set.get(self.rs).ok_or(InstructionError::InvalidRegister(self.rs as u32))?;

        let result = match self.operand {
            MultiplyOperand::NonHalfwordMultiplies { rm } => {
                let rm_cell = register_set.get(rm).ok_or(InstructionError::InvalidRegister(rm as u32))?;
                let rm_value = rm_cell.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;
                
               let rs_value = rs_cell.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?; 
               let rn_value = rn_cell.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;

                match self.opcode() {
                    MultiplyOpcode::MUL => rm_value * rs_value,
                    MultiplyOpcode::MLA => rm_value * rs_value + rn_value, // TODO: Check restrictions such as rd != rm and rd, rm, rs, rn != 15
                    MultiplyOpcode::UMAAL => todo!(),
                    MultiplyOpcode::UMULL => todo!(),
                    MultiplyOpcode::UMLAL => todo!(),
                    MultiplyOpcode::SMULL => todo!(),
                    MultiplyOpcode::SMLAL => todo!(),
                    MultiplyOpcode::SMLAXY => {
                        return Err(InstructionError::InvalidOpcode(self.opcode_bits));
                    },
                    MultiplyOpcode::SMLAWY => {
                        return Err(InstructionError::InvalidOpcode(self.opcode_bits));
                    },
                    MultiplyOpcode::SMULWY => {
                        return Err(InstructionError::InvalidOpcode(self.opcode_bits));
                    },
                    MultiplyOpcode::SMLALXY => {
                        return Err(InstructionError::InvalidOpcode(self.opcode_bits));
                    },
                    MultiplyOpcode::SMULXY => {
                        return Err(InstructionError::InvalidOpcode(self.opcode_bits));
                    },
                    MultiplyOpcode::Invalid => {
                        return Err(InstructionError::InvalidOpcode(self.opcode_bits));
                    }, // TODO: need to use correct opcode
                }
            },
            MultiplyOperand::HalfwordMultiplies { y, x, rm } => todo!(),
        };

        rd_cell.write(result).map_err(|e| InstructionError::RegisterWriteError(e.to_string()))?;
       Ok(())
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mul_decode() {
        let value: u32 = 0b0001_000_0000_1_0010_0100_0011_1001_0001;
        // Condition:    0001 (NE)
        // Opcode:       0000 (MUL)
        // S Flag:       1
        // Rd:           0010 (R2)
        // Rn:           0100 (R4)
        // Rs:           0011 (R3)
        // Rm:           0001 (R1)

        let instruction = MultiplyInstruction::decode(value);
        assert!(instruction.is_ok());
        let instruction = instruction.unwrap();

        assert_eq!(instruction.condition(), Condition::NE);
        assert_eq!(instruction.opcode(), MultiplyOpcode::MUL);
        assert_eq!(instruction.s_flag, true);

        assert_eq!(instruction.rd, 2);
        assert_eq!(instruction.rn, 4);
        assert_eq!(instruction.rs, 3);
        assert_eq!(instruction.operand, MultiplyOperand::NonHalfwordMultiplies { rm: 1 });
    }
}