
use core::fmt;
use std::collections::HashSet;

use strum_macros::Display;

use crate::{instruction::{Condition, DecodeInstruction, Instruction, InstructionError}, memory::MemoryBus, register::{ReadRegister, RegisterCell, RegisterSet, WriteRegister, CPSR}};

use super::{get_s_flag, is_data_processing_instruction, shift, ShiftBy, ShiftResult, ShiftType};

#[derive(Debug, Clone)]
pub struct DataProccessingInstruction {
    pub condition_bits: u8, // Bits 31-28
    pub immediate: bool, // Bit 25 (Immediate 2nd Operand Flag) (0=Register, 1=Immediate)
    pub opcode_bits: u8 , // Bits 24-21
    pub s_flag: bool, // Bit 20 (Set Condition Codes) (0=No, 1=Yes) (Must be 1 for opcode 8-B)
    pub rn: u8, // Bits 19-16 (1st Operand Register: R0-R15) (Including PC=R15) (Must be 0000b for Mov/Mvn)
    pub rd: u8, // Bits 15-12 (Destination Register: R0-R15) (Including PC=R15) (Must be 0000b or 1111b for Cmp/Cmn/Tst/Teq{P})
    pub operand: DataProccessingOperand, // Bits 11-0 (2nd Operand: Immediate Value or Register)
}

impl fmt::Display for DataProccessingInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}} R{},R{},{}", self.opcode(), self.condition(), self.rd, self.rn, self.operand)
    }
}

impl DataProccessingInstruction {
    pub fn condition(&self) -> Condition {
        Condition::from_bits_truncate(self.condition_bits)
    }

    pub fn opcode(&self) -> DataProcessingOpcode {
        DataProcessingOpcode::from(self.opcode_bits)
    }

    pub fn rn_cell(&self, register_set: &RegisterSet) -> Option<RegisterCell> {
        register_set.get(self.rn)
    }

    pub fn rd_cell(&self, register_set: &RegisterSet) -> Option<RegisterCell> {
        register_set.get(self.rd)
    }

    pub fn cpsr(&self, register_set: &RegisterSet) -> Option<CPSR> {
        let cpsr = register_set.cpsr.read().ok()?;
        CPSR::from_bits(cpsr)
    }

    pub fn is_logical(&self) -> bool {
        self.opcode().is_logical()
    }

    pub fn is_arithmetic(&self) -> bool {
        self.opcode().is_arithmetic()
    }
}

#[derive(Debug, Clone, Display, PartialEq, Eq, Hash)]
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

impl DataProcessingOpcode {
    pub fn is_logical(&self) -> bool {
        match self {
            DataProcessingOpcode::AND | DataProcessingOpcode::EOR | DataProcessingOpcode::TST | DataProcessingOpcode::TEQ | DataProcessingOpcode::ORR | DataProcessingOpcode::MOV | DataProcessingOpcode::BIC | DataProcessingOpcode::MVN => true,
            _ => false,
        }
    }

    pub fn is_arithmetic(&self) -> bool {
        match self {
            DataProcessingOpcode::SUB | DataProcessingOpcode::RSB | DataProcessingOpcode::ADD | DataProcessingOpcode::ADC | DataProcessingOpcode::SBC | DataProcessingOpcode::RSC | DataProcessingOpcode::CMP | DataProcessingOpcode::CMN => true,
            _ => false,
        }
    }
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

impl Into<DataProccessingOperandResult> for ShiftResult {
    fn into(self) -> DataProccessingOperandResult {
        DataProccessingOperandResult {
            result: self.value,
            carry: self.carry,
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProccessingOperandResult {
    pub result: u32,
    pub carry: Option<u32>,
}

impl DataProccessingOperandResult {
    pub fn new(result: u32, carry: Option<u32>) -> Self {
        Self {
            result,
            carry,
        }
    }
}

impl DataProccessingOperand {
    pub fn compute(&self, register_set: &RegisterSet) -> Result<DataProccessingOperandResult, InstructionError> {
        match self {
            DataProccessingOperand::Immediate { shift_amount, nn } => {
                let shift_amount = shift_amount.clone();
                let nn = nn.clone();
                if shift_amount > 0 {
                    let result = nn.rotate_right(shift_amount.into()) as u32;
                    let carry = ((nn >> (shift_amount - 1)) & 1) as u32; 
                    Ok(DataProccessingOperandResult::new(result, Some(carry)))
                } else {
                    Ok(DataProccessingOperandResult::new(nn as u32, None))
                }
            },
            DataProccessingOperand::Register { shift_type, shift_by, rm } => {
                let rm = rm.clone();
                let shift_type = shift_type.clone();
                let rm_value = register_set.get(rm)
                    .ok_or(InstructionError::InvalidRegister(rm as u32))?
                    .read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;
                let cspr = CPSR::from_bits(
                                    register_set.cpsr.read()
                                        .map_err(|e| InstructionError::RegisterReadError(e.to_string()))?);
                
                if cspr.is_none() {
                    return Err(InstructionError::InvalidCPSR());
                }

                let cspr = cspr.unwrap();
                let carry_in = cspr.carry();
                match shift_by {
                    ShiftBy::Immediate(shift_amount) => {
                        Ok(shift_type.shift(shift_amount.clone(), rm_value, carry_in).into())
                    }
                    ShiftBy::Register(rs) => {
                        let rs = rs.clone();
                        let rs = register_set.get(rs)
                            .ok_or(InstructionError::InvalidRegister(rs as u32))?;
                        let rs_value = rs.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;
                        Ok(shift_type.shift(rs_value as u8, rm_value, carry_in).into())
                    }
               }
           }
        }
    }
}

impl fmt::Display for DataProccessingOperand {
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
        
        let condition_bits = (value >> 28) as u8;
        // Bits 27-26 must be 00b
        if !is_data_processing_instruction(value) {
            return Err(InstructionError::InvalidInstruction(value));
        }

        let immediate = (value & (1 << 25)) != 0;
        let opcode_bits = ((value >> 21) & 0xF) as u8;
        let opcode = DataProcessingOpcode::from(opcode_bits);
        if opcode == DataProcessingOpcode::Invalid {
            return Err(InstructionError::InvalidOpcode(opcode_bits));
        }

        let s_flag = get_s_flag(value);

        let rn: u8 = ((value >> 16) & 0xF) as u8;
        let rd = ((value >> 12) & 0xF) as u8;


        let operand = if immediate {
            // Immediate as 2nd Operand
            let immediate_value= (value & 0xFF) as u8;
            // ROR shift amount (0-30, in steps of 2)
            let ror_shift_amount = ((value >> 8) & 0xF) as u8;
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
            condition_bits,
            immediate,
            opcode_bits,
            s_flag,
            rn,
            rd,
            operand,
        })
    }
}

impl Instruction for DataProccessingInstruction {
    fn execute(&mut self, register_set: &RegisterSet, _memory_bus: &MemoryBus) -> Result<(), InstructionError> {

        let mut write_result = true;
        let rn_value = self.rn_cell(register_set)
            .ok_or(InstructionError::InvalidRegister(self.rn as u32))?
            .read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;

        let op2_result = self.operand.compute(register_set)?;

        let mut cpsr = self.cpsr(register_set).ok_or(InstructionError::InvalidCPSR())?;

        let result = match self.opcode() {
            // Logical AND
            DataProcessingOpcode::AND => {
                rn_value & op2_result.result
            },
            // Logical XOR
            DataProcessingOpcode::EOR => {
                rn_value ^ op2_result.result
            },
            // Subtract
            DataProcessingOpcode::SUB => {
                rn_value - op2_result.result
            },
            // Reverse Subtract
            DataProcessingOpcode::RSB => {
                op2_result.result - rn_value
            },
            // Add
            DataProcessingOpcode::ADD => {
                rn_value + op2_result.result
            },
            // Add with Carry
            DataProcessingOpcode::ADC => {
                rn_value + op2_result.result + cpsr.carry()
            },
            // Subtract with Carry
            DataProcessingOpcode::SBC => {
                rn_value - op2_result.result + cpsr.carry() - 1
            },
            // Subtract with Carry Reverse
            DataProcessingOpcode::RSC => {
                op2_result.result - rn_value + cpsr.carry() - 1
            },
            // Test
            DataProcessingOpcode::TST => {
                write_result = false;
                rn_value & op2_result.result
            },
            // Test Exclusive
            DataProcessingOpcode::TEQ => {
                write_result = false;
                rn_value ^ op2_result.result
            },
            // Compare
            DataProcessingOpcode::CMP => {
                write_result = false;
                rn_value - op2_result.result
            },
            // Compare Negative
            DataProcessingOpcode::CMN => {
                write_result = false;
                rn_value + op2_result.result
            },
            // Logical OR
            DataProcessingOpcode::ORR => {
                rn_value | op2_result.result
            },
            // Move 
            DataProcessingOpcode::MOV => {
                op2_result.result
            },
            // Bit Clear
            DataProcessingOpcode::BIC => {
                rn_value & !op2_result.result
            },
            // Not
            DataProcessingOpcode::MVN => {
                !op2_result.result
            },
            DataProcessingOpcode::Invalid => {
                return Err(InstructionError::InvalidOpcode(self.opcode_bits)); 
            },
        };

        if write_result {
            self.rd_cell(register_set)
                .ok_or(InstructionError::InvalidRegister(self.rd as u32))?
                .write(result).map_err(|e| InstructionError::RegisterWriteError(e.to_string()))?;
        }

        // flags to be set: NZc- (Negative, Zero, Carry, Overflow)
        if self.s_flag && self.rd != 15 && self.is_logical() {
            // set zero flag
            cpsr.setz(result == 0);
            // set sign flag
            cpsr.setn((result >> 31 & 1) == 1);
            // set carry flag of shift operation
            if op2_result.carry.is_some() {
                cpsr.setc(op2_result.carry.unwrap() == 1);
            }
        }

        if self.s_flag && self.rd != 15 && self.is_arithmetic() {
            // set zero flag
            cpsr.setz(result == 0);
            // set sign flag
            cpsr.setn((result >> 31 & 1) == 1);
            // set carry flag of shift operation
            if op2_result.carry.is_some() {
                cpsr.setc(op2_result.carry.unwrap() == 1);
            }
            // set overflow flag
            let rn_sign = (rn_value >> 31) & 1;
            let op2_sign = (op2_result.result >> 31) & 1;
            let result_sign = (result >> 31) & 1;
            cpsr.setv((rn_sign == op2_sign) && (rn_sign != result_sign));
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {

    use crate::register::RegisterCell;

    use super::*;

    #[test]
    fn test_invalid_data_processing_instruction() {
        // bits 27-26 must be 00b
        let value = 0b1110_10_0_0010_0_1000_0111_1010_0_11_1_1001;
        let instruction = DataProccessingInstruction::decode(value);
        assert_eq!(instruction.err(), Some(InstructionError::InvalidInstruction(value)));
    }

    #[test]
    fn test_invalid_op_code() {
        let invalid_opcode_value: u8 = 100;
        assert_eq!(DataProcessingOpcode::from(invalid_opcode_value), DataProcessingOpcode::Invalid);
    }

    #[test]
    fn test_alu_instruction_add_registers() {
        let expected_str = "ADD{AL} R2,R1,R3,LSL#0";
        let value: u32 = 0xE0812003;
        let instruction = DataProccessingInstruction::decode(value);
        assert!(instruction.is_ok());
        let instruction = instruction.unwrap();
        assert_eq!(instruction.condition(), Condition::AL);
        assert_eq!(instruction.immediate, false);
        assert_eq!(instruction.opcode(), DataProcessingOpcode::ADD); 
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

        assert_eq!(instruction.condition(), Condition::AL);
        assert!(!instruction.s_flag);
        assert!(!instruction.immediate);
        assert_eq!(instruction.opcode(), DataProcessingOpcode::SUB);
        assert_eq!(instruction.rn, 8);
        assert_eq!(instruction.rd, 7);
        assert_eq!(instruction.operand, DataProccessingOperand::Register {
            shift_type: ShiftType::ROR,
            shift_by: ShiftBy::Register(10),
            rm: 9,
        });
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
        assert_eq!(instruction.condition(), Condition::AL);
        assert_eq!(instruction.opcode(), DataProcessingOpcode::AND);
        assert!(instruction.immediate);
        assert_eq!(instruction.rn, 0);
        assert_eq!(instruction.rd, 1);

        let register_set = RegisterSet::builder()
            .with_register(0, RegisterCell::new(3)).unwrap()
            .with_register(1, RegisterCell::new(0)).unwrap()
            .build();
        let result = instruction.execute(&register_set, &MemoryBus::default());
        assert!(result.is_ok());
        let value = instruction.rd_cell(&register_set).unwrap().read().unwrap();
        assert_eq!(value, expected_value);
    }
}