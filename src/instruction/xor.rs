use crate::register::{ReadRegister, RegisterSet, WriteRegister};

use super::{DataProccessingOperand, InstructionError, ShiftBy, ShiftType};


pub fn xor(register_set: RegisterSet, s_flag: bool, rd: u8, rn: u8, op2: DataProccessingOperand) -> Result<(), InstructionError> {
    match op2 {
        DataProccessingOperand::Immediate { shift_amount, nn } => {
            xor_immediate(register_set, s_flag, rd, rn, shift_amount, nn)
        },
        DataProccessingOperand::Register { shift_type, shift_by, rm } => {
            xor_register(register_set, s_flag, rd, rn, shift_type, shift_by, rm)
        }
    }
}

pub fn xor_immediate(register_set: RegisterSet, s_flag: bool, rd: u8, rn: u8, shift_amount: u8, nn: u8) -> Result<(), InstructionError> {
    let rn = register_set.get(rn).ok_or(InstructionError::InvalidRegister(rn as u32))?;
    let rn_value = rn.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;
    let op2 = if shift_amount > 0 {
        nn.rotate_right(shift_amount.into()) as u32
    } else {
        nn as u32
    };

    let result = rn_value ^ op2;
    let mut rd = register_set.get(rd).ok_or(InstructionError::InvalidRegister(rd as u32))?;
    rd.write(result).map_err(|e| InstructionError::RegisterWriteError(e.to_string()))?;

    if s_flag {
        // Set flags
        todo!()
    }
    Ok(())
}

pub fn xor_register(register_set: RegisterSet, s_flag: bool, rd: u8, rn: u8, shift_type: ShiftType, shift_by: ShiftBy, rm: u8) -> Result<(), InstructionError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::register::RegisterCell;

    use super::*;

    #[test]
    fn test_xor_immediate_no_shift() {
        let expected_value: u32 = 10 ^ 5;
        let register_set = RegisterSet::builder()
            .with_register(0, RegisterCell::new(10)).unwrap()
            .with_register(2, RegisterCell::new(1)).unwrap()
            .build();

        xor_immediate(register_set.clone(), false, 2, 0, 0, 5).unwrap();
        assert_eq!(register_set.clone().get(2).unwrap().read().unwrap(), expected_value);
    }
}