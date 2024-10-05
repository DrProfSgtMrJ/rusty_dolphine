use core::fmt;
use bitflags::bitflags;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftResult {
    pub value: u32,
    pub carry: Option<u32>,
}

impl ShiftResult {
    pub fn new(value: u32, carry: Option<u32>) -> Self {
        Self {
            value,
            carry,
        }
    }
}

/*
Special Case: 

Zero Shift Amount (Shift Register by Immediate, with Immediate=0)

  LSL#0: No shift performed, ie. directly Op2=Rm, the C flag is NOT affected.
  LSR#0: Interpreted as LSR#32, ie. Op2 becomes zero, C becomes Bit 31 of Rm.
  ASR#0: Interpreted as ASR#32, ie. Op2 and C are filled by Bit 31 of Rm.
  ROR#0: Interpreted as RRX#1 (RCR), like ROR#1, but Op2 Bit 31 set to old C.


*/
impl ShiftType {
    // Shifts the value by the shift amount
    // Returns the shifted value and the carry flag (if applicable)
    pub fn shift(self, shift_amount: u8, value: u32, carry_in: u32) -> ShiftResult {
        // (1-31) (0 is a special case)
        let shift_amount = shift_amount % 32;
        match self {
            // Logical shift left
            ShiftType::LSL => {
                // Special case for shift amount 0
                if shift_amount == 0 {
                    return ShiftResult::new(value, None);
                }
                let carry = (value >> (32 - shift_amount)) & 1;
                let shifted_value = value << shift_amount;
                ShiftResult::new(shifted_value, Some(carry))
            }, 
            // Logical shift rigtht
            ShiftType::LSR => {
                if shift_amount == 0 {
                    // Bit 31 of Rm = value
                    return ShiftResult::new(0, Some((value >> 31) & 1));
                }
                let shifted_value = value >> shift_amount;
                let carry = (value >> (shift_amount - 1)) & 1;
                ShiftResult::new(shifted_value, Some(carry))
            }, 
            // Arithmetic shift right
            ShiftType::ASR => {
                if shift_amount == 0 {
                    let msb = (value >> 31) & 1;
                    // Bit 31 of Rm = value
                    //set_carry(msb == 1);
                    // Fill with the most significant bit
                    if msb == 1 {
                        return ShiftResult::new(0xFFFFFFFF, Some(1));
                    } else {
                        return ShiftResult::new(0, Some(0));
                    }
                }
                let shifted_value = (((value as i32) >> shift_amount) as u32);
                let carry = (value >> (shift_amount - 1)) & 1;
                ShiftResult::new(shifted_value, Some(carry))
            }, 
            // Rotate right
            ShiftType::ROR => {
                if shift_amount == 0 {
                    // ROR#0 is interpreted as RRX (Rotate Right with Extend)
                    let shifted_value = (value >> 1) | (carry_in << 31);
                    let carry = value & 1;  // The bit that was shifted out becomes the new carry
                    return ShiftResult::new(shifted_value, Some(carry));
                }
                // Regular Rotate Right
                let shifted_value = value.rotate_right(shift_amount.into());
                let carry = (value >> (shift_amount - 1)) & 1;
                ShiftResult::new(shifted_value, Some(carry))
            }, // Rotate right
            _ => ShiftResult::new(value, None),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_lsl() {
        let shift_type = ShiftType::LSL; 
        let shift_amount = 2;
        let value = 0b0000_0000_0000_0000_0000_0000_0000_0001;

        let result = shift_type.shift(shift_amount, value, 0);
        assert_eq!(result.value, 0b0000_0000_0000_0000_0000_0000_0000_0100);
    }

    #[test]
    fn test_shift_lsr() {
        let shift_type = ShiftType::LSR; 
        let value = 0b0000_0000_0000_0000_0000_0000_0001_0000;

        let result = shift_type.shift(2, value, 0);

        assert_eq!(result.value, 0b0000_0000_0000_0000_0000_0000_0000_0100);
    }

    #[test]
    fn test_shift_asr() {
        let shift_type = ShiftType::ASR;
        let result = shift_type.shift(4, 0xF000_0000, 0);
        assert_eq!(result.value, 0xFF00_0000); // Expect ASR on signed value (preserve sign bits)
    }

     #[test]
    fn test_shift_ror() {
        let shift_type = ShiftType::ROR;
        let result = shift_type.shift(4, 0x1000_0001, 0);
        assert_eq!(result.value, 0x1100_0000); // Rotate right by 4 -> lower nibble becomes upper
    }
}