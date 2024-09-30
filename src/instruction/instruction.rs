use bitflags::bitflags;

use super::error::InstructionError;

pub trait Instruction {
    fn execute(&mut self) -> Result<(), InstructionError>;
}

#[derive(Debug, Clone)]
pub struct ALUInstruction {
    pub condition: Condition, // Bits 31-28
    pub immediate: bool, // Bit 25 (Immediate 2nd Operand Flag) (0=Register, 1=Immediate)
    pub opcode: u8, // Bits 24-21
    pub s_flag: bool, // Bit 20 (Set Condition Codes) (0=No, 1=Yes) (Must be 1 for opcode 8-B)
    pub rn: u8, // Bits 19-16 (1st Operand Register: R0-R15) (Including PC=R15) (Must be 0000b for Mov/Mvn)
    pub rd: u8, // Bits 15-12 (Destination Register: R0-R15) (Including PC=R15) (Must be 0000b or 1111b for Cmp/Cmn/Tst/Teq{P})
    pub operand: Operand, // Bits 11-0 (2nd Operand: Immediate Value or Register)
}

bitflags! {
    #[derive(Debug, Default, Clone)]
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

#[derive(Debug, Clone)]
pub enum Operand {
    Immediate {
        ror: u8, // Bits 11-8 (ROR-Shift applied to nn) (0-30, in steps of 2)
        nn: u8, // Bits 7-0 (2nd Operand unsigned 8-bit immediate value)
    }, // Holds the immediate value when I=1
    Register {
        shift_type: ShiftType, // Bits 6-5 
        shift_by: ShiftBy,
        rm: u8, // Bits 3-0 (2nd Operand Register: R0-R15) (Including PC=R15)
    }
}

#[derive(Debug, Clone)]
enum ShiftBy {
    Immediate(u8),
    Register(u8),
}

bitflags! {
    #[derive(Debug, Default, Clone)]
    struct ShiftType: u8 {
        const LSL = 0b00; // Logical Shift Left
        const LSR = 0b01; // Logical Shift Right
        const ASR = 0b10; // Arithmetic Shift Right
        const ROR = 0b11; // Rotate Right
    }
}

// Decodes u32 to ALUInstruction
impl From<u32> for ALUInstruction {
    fn from(value: u32) -> Self {
        let condition_bits: u8 = ((value >> 28) & 0xF) as u8;
        let condition = Condition::from_bits_truncate(condition_bits);

        let immediate = (value & (1 << 25)) != 0;

        let opcode: u8 = ((value >> 21) & 0xF) as u8;

        let s_flag = (value & (1 << 20)) != 0;

        let rn: u8 = ((value >> 16) & 0xF) as u8;
        let rd: u8 = ((value >> 12) & 0xF) as u8;

        let operand = if immediate {
            // Immediate as 2nd Operand
            let immediate_value: u8 = (value & 0xFF) as u8;
            let ror_shift_amount: u8 = ((value >> 8) & 0x1F) as u8;
            Operand::Immediate {
                ror: ror_shift_amount,
                nn: immediate_value,
            }
        } else {
            // Register as 2nd Operand
            let shift_type_bits: u8 = ((value >> 5) & 0x3) as u8;
            let shift_type = ShiftType::from_bits_truncate(shift_type_bits);
            let shift_by = if (value & (1 << 4)) != 0 {
                ShiftBy::Register(((value >> 8) & 0xF) as u8)
            } else {
                ShiftBy::Immediate(((value >> 7) & 0x1F) as u8)
            };
            let rm: u8 = (value & 0xF) as u8;
            Operand::Register {
                shift_type,
                shift_by,
                rm,
            }
        };

        ALUInstruction {
            condition,
            immediate,
            opcode,
            s_flag,
            rn,
            rd,
            operand,
        }
    }
}