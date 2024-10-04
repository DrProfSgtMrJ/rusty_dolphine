
use core::fmt;

use bitflags::bitflags;
use strum_macros::Display;

use crate::{instruction::{and_immediate, Condition, DecodeInstruction, Instruction, InstructionError}, memory::MemoryBus, register::{ReadRegister, RegisterSet, WriteRegister}};

use super::and; 

const BIT_31_28_MASK: u32 = 0xF000_0000;
const BIT_27_26_MASK: u32 = 0x0C;
const BIT_25_MASK: u32 = 0x0200_0000;
const BIT_21_24_MASK: u32 = 0x01E0_0000;
const BIT_20_MASK: u32 = 0x0010_0000;
const BIT_16_19_MASK: u32 = 0x000F_0000;
const BIT_12_15_MASK: u32 = 0x0000_F000;
const BIT_8_11_MASK: u32 = 0x0000_0F00;
const BIT_7_11_MASK: u32 = 0x0000_0F80;
const BIT_5_6_MASK: u32 = 0x0000_0060;
const BIT_4_MASK : u32 = 0x0000_0010;
const BIT_0_4_MASK: u32 = 0x0000_000F;
const BIT_0_7_MASK: u32 = 0x0000_00FF;

#[derive(Debug, Clone)]
pub struct DataProccessingInstruction {
    pub condition: Condition, // Bits 31-28
    pub immediate: bool, // Bit 25 (Immediate 2nd Operand Flag) (0=Register, 1=Immediate)
    pub opcode: DataProcessingOpcode, // Bits 24-21
    pub s_flag: bool, // Bit 20 (Set Condition Codes) (0=No, 1=Yes) (Must be 1 for opcode 8-B)
    pub rn: u8, // Bits 19-16 (1st Operand Register: R0-R15) (Including PC=R15) (Must be 0000b for Mov/Mvn)
    pub rd: u8, // Bits 15-12 (Destination Register: R0-R15) (Including PC=R15) (Must be 0000b or 1111b for Cmp/Cmn/Tst/Teq{P})
    pub operand: DataProccessingOperand, // Bits 11-0 (2nd Operand: Immediate Value or Register)
}

impl fmt::Display for DataProccessingInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}} R{},R{},{}", self.opcode, self.condition, self.rd, self.rn, self.operand)
    }
}

#[derive(Debug, Clone, Display, PartialEq, Eq)]
pub enum DataProcessingOpcode {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
    Invalid,
}

impl From<u8> for DataProcessingOpcode {
    fn from(value: u8) -> Self {
        match value {
            0 => DataProcessingOpcode::AND,
            1 => DataProcessingOpcode::EOR,
            2 => DataProcessingOpcode::SUB,
            3 => DataProcessingOpcode::RSB,
            4 => DataProcessingOpcode::ADD,
            5 => DataProcessingOpcode::ADC,
            6 => DataProcessingOpcode::SBC,
            7 => DataProcessingOpcode::RSC,
            8 => DataProcessingOpcode::TST,
            9 => DataProcessingOpcode::TEQ,
            10 => DataProcessingOpcode::CMP,
            11 => DataProcessingOpcode::CMN,
            12 => DataProcessingOpcode::ORR,
            13 => DataProcessingOpcode::MOV,
            14 => DataProcessingOpcode::BIC,
            15 => DataProcessingOpcode::MVN,
            _ => DataProcessingOpcode::Invalid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftBy {
    // (1-31) (0 is a special case)
    Immediate(u8),
    Register(u8),
}

impl fmt::Display for ShiftBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShiftBy::Immediate(value) => write!(f, "#{}", value),
            ShiftBy::Register(value) => write!(f, "R{}", value),
        }
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct ShiftType: u8 {
        const LSL = 0b00000000; // Logical Shift Left
        const LSR = 0b00000001; // Logical Shift Right
        const ASR = 0b00000010; // Arithmetic Shift Right
        const ROR = 0b00000011; // Rotate Right
    }
}

impl ShiftType {
    pub fn shift(self, shift_amount: u8, value: u32) -> u32 {
        match self {
            ShiftType::LSL => todo!(),
            ShiftType::LSR => todo!(),
            ShiftType::ASR => todo!(), 
            ShiftType::ROR => todo!(),
            _ => value,
        }
    }
}

impl fmt::Display for ShiftType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.bits() {
            0b00 => write!(f, "LSL")?,
            0b01 => write!(f, "LSR")?,
            0b10 => write!(f, "ASR")?,
            0b11 => write!(f, "ROR")?,
            _ => write!(f, "Unknown")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataProccessingOperand {
    Immediate {
        shift_amount: u8, // Bits 11-8 (ROR-Shift applied to nn) (0-30, in steps of 2)
        nn: u8, // Bits 7-0 (2nd Operand unsigned 8-bit immediate value)
    }, // Holds the immediate value when I=1
    Register {
        shift_type: ShiftType, // Bits 6-5 
        shift_by: ShiftBy,
        rm: u8, // Bits 3-0 (2nd Operand Register: R0-R15) (Including PC=R15)
    }
}

impl fmt::Display for DataProccessingOperand{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataProccessingOperand::Immediate { shift_amount, nn } => {
                write!(f, "{:08b},ROR#{}", nn, shift_amount )
            }
            DataProccessingOperand::Register { shift_type, shift_by, rm } => {
                write!(f, "R{},{}{}", rm, shift_type, shift_by)
            }
        }
    }
}

impl DecodeInstruction for DataProccessingInstruction {
    fn decode(value: u32) -> Result<Self, InstructionError>
        where
            Self: Sized {
        
        let condition_bits = ((value >> 28) & 0xF) as u8;
        let condition = Condition::from_bits_truncate(condition_bits);

        // Bits 27-26 must be 00b
        let bits_27_26 = (value >> 26) & 0x3;
        if bits_27_26 != 0 {
            return Err(InstructionError::InvalidInstruction(value));
        }

        let immediate = (value & (1 << 25)) != 0;
        let opcode_value = ((value >> 21) & 0xF) as u8;
        let opcode = DataProcessingOpcode::from(opcode_value);

        if opcode == DataProcessingOpcode::Invalid {
            return Err(InstructionError::InvalidOpcode(opcode_value));
        }

        let s_flag = (value & (1 << 20)) != 0;

        let rn = ((value >> 16) & 0xF) as u8;
        let rd = ((value >> 12) & 0xF) as u8;


        let operand = if immediate {
            // Immediate as 2nd Operand
            let immediate_value= (value & 0xFF) as u8;
            // ROR shift amount (0-30, in steps of 2)
            let ror_shift_amount = (((value >> 8) & 0x1F) * 2) as u8;
            DataProccessingOperand::Immediate {
                shift_amount: ror_shift_amount,
                nn: immediate_value,
            }
        } else {
            // Register as 2nd Operand
            let shift_type_bits = ((value >> 5) & 0x3) as u8;
            let shift_type = ShiftType::from_bits_retain(shift_type_bits);
            let shift_by = if (value & (1 << 4)) != 0 {
                // bit 7 must be 0 TODO: Check this
                let shift_reg = ((value >> 8) & 0xF) as u8;
                ShiftBy::Register(shift_reg) 
            } else {
                let shift_imm = ((value >> 7) & 0x1F) as u8;
                ShiftBy::Immediate(shift_imm)
            };
            let rm = (value & 0xF) as u8;
            DataProccessingOperand::Register {
                shift_type,
                shift_by,
                rm,
            }
        };

        Ok(DataProccessingInstruction {
            condition,
            immediate,
            opcode,
            s_flag,
            rn,
            rd,
            operand,
        })
    }
}

impl Instruction for DataProccessingInstruction {
    fn execute(&mut self, register_set: RegisterSet, memory_bus: MemoryBus) -> Result<(), InstructionError> {
       match self.opcode {
            DataProcessingOpcode::AND => {
                and(register_set, self.s_flag, self.rd, self.rn, self.operand.clone())
            },
            DataProcessingOpcode::EOR => todo!(),
            DataProcessingOpcode::SUB => todo!(),
            DataProcessingOpcode::RSB => todo!(),
            DataProcessingOpcode::ADD => todo!(),
            DataProcessingOpcode::ADC => todo!(),
            DataProcessingOpcode::SBC => todo!(),
            DataProcessingOpcode::RSC => todo!(),
            DataProcessingOpcode::TST => todo!(),
            DataProcessingOpcode::TEQ => todo!(),
            DataProcessingOpcode::CMP => todo!(),
            DataProcessingOpcode::CMN => todo!(),
            DataProcessingOpcode::ORR => todo!(),
            DataProcessingOpcode::MOV => todo!(),
            DataProcessingOpcode::BIC => todo!(),
            DataProcessingOpcode::MVN => todo!(),
            DataProcessingOpcode::Invalid => todo!(),
        } 
    }
}



#[cfg(test)]
mod tests {

    use crate::register::RegisterCell;

    use super::*;

    #[test]
    fn test_alu_instruction_add_registers() {
        let expected_str = "ADD{AL} R2,R1,R3,LSL#0";
        let value: u32 = 0xE0812003;
        let instruction = DataProccessingInstruction::decode(value);
        assert!(instruction.is_ok());
        let instruction = instruction.unwrap();
        assert_eq!(instruction.condition, Condition::AL);
        assert_eq!(instruction.immediate, false);
        assert_eq!(instruction.opcode, DataProcessingOpcode::ADD); 
        assert_eq!(instruction.s_flag, false); // don't update condition flag

        assert_eq!(instruction.rn, 1); // R1
        assert_eq!(instruction.rd, 2); // R2
        assert_eq!(instruction.operand, DataProccessingOperand::Register {
            shift_type: ShiftType::LSL,
            shift_by: ShiftBy::Immediate(0),
            rm: 3, // R3
        });

        assert_eq!(instruction.to_string(), expected_str);
    }

    #[test]
    fn test_alu_instruction_add_with_shift_register() {
        let value = 0b1110_00_0_0010_0_1000_0111_1010_0_11_1_1001;
        // Condition:    1110 (AL)
        // I:           0 (Register Operand)
        // Opcode:      0010 (SUB)
        // S:           0 (No condition code flag set)
        // Rn:          1000 (R8)
        // Rd:          0111 (R7)
        // Rs:          1010 (R10 - Shift by value in R10)
        // Shift type:  11 (ROR)
        // R:           1 (Shift by register flag)
        // Rm:          1001 (R9)
        let instruction = DataProccessingInstruction::decode(value);
        assert!(instruction.is_ok());
        let instruction = instruction.unwrap();

        assert_eq!(instruction.condition, Condition::AL);
        assert!(!instruction.s_flag);
        assert!(!instruction.immediate);
        assert_eq!(instruction.opcode, DataProcessingOpcode::SUB);
        assert_eq!(instruction.rn, 8);
        assert_eq!(instruction.rd, 7);
        assert_eq!(instruction.operand, DataProccessingOperand::Register {
            shift_type: ShiftType::ROR,
            shift_by: ShiftBy::Register(10),
            rm: 9,
        });
        println!("{}", instruction.to_string());
    }

    #[test]
    fn test_and_immediate_execute() {
        let value = 0b1110_00_1_0000_0_0000_0001_0000_00000010;
        // Condition:    1110 (AL)
        // I:           1 (Immediate Operand)
        // Opcode:      0000 (AND)
        // S:           0 (No condition code flag set)
        // Rn:          0000 (R0)
        // Rd:          0001 (R1)
        // Is:          0000 (0)
        // nn:          00000010 (2)

        let expected_value: u32 = 3 & 2;
        let instruction = DataProccessingInstruction::decode(value);
        assert!(instruction.is_ok());
        let mut instruction = instruction.unwrap();
        assert_eq!(instruction.condition, Condition::AL);
        assert_eq!(instruction.opcode, DataProcessingOpcode::AND);
        assert!(instruction.immediate);
        assert_eq!(instruction.rn, 0);
        assert_eq!(instruction.rd, 1);

        let register_set = RegisterSet::builder()
            .with_register(0, RegisterCell::new(3)).unwrap()
            .with_register(1, RegisterCell::new(0)).unwrap()
            .build();
        let result = instruction.execute(register_set.clone(), MemoryBus::default());
        assert!(result.is_ok());
        let register = register_set.clone().get(1).unwrap();
        let value = register.read().unwrap();
        assert_eq!(value, expected_value);
    }
}