use crate::register::{ReadRegister, RegisterSet, WriteRegister};

use super::{DataProccessingOperand, InstructionError, ShiftBy, ShiftType};

pub fn and(register_set: RegisterSet, s_flag: bool, rd: u8, rn: u8, op2: DataProccessingOperand) -> Result<(), InstructionError> {
    match op2 {
        DataProccessingOperand::Immediate { shift_amount, nn } => {
            and_immediate(register_set, s_flag, rd, rn, shift_amount, nn)
        },
        DataProccessingOperand::Register { shift_type, shift_by, rm } => {
            and_register(register_set, s_flag, rd, rn, shift_type, shift_by, rm)
        }
    }
}

pub fn and_immediate(register_set: RegisterSet, s_flag: bool, rd: u8, rn: u8, shift_amount: u8, nn: u8) -> Result<(), InstructionError> {
    if let Some(rn) = register_set.get(rn) {
        let rn_value = rn.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;
        let op2 = if shift_amount > 0 {
            nn.rotate_right(shift_amount.into()) as u32
        } else {
            nn as u32
        };

        let result = rn_value & op2;

        if let Some(mut rd) = register_set.get(rd) {
            rd.write(result).map_err(|e| InstructionError::RegisterWriteError(e.to_string()))?;
            if s_flag {
                // Set flags
                todo!()
            }
            return Ok(());
        } else {
            return Err(InstructionError::InvalidRegister(rd as u32));
        }
    } else {
        return Err(InstructionError::InvalidRegister(rn as u32));
    }
}

pub fn and_register(register_set: RegisterSet, s_flag: bool, rd: u8, rn: u8, shift_type: ShiftType, shift_by: ShiftBy, rm: u8) -> Result<(), InstructionError> {
    // Get rn, get rm, shift rm, write to rd (rn & rm)
    let rn = register_set.get(rn).ok_or(InstructionError::InvalidRegister(rn as u32))?;

    let rn_value = rn.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;

    let rm = register_set.get(rm).ok_or(InstructionError::InvalidRegister(rm as u32))?;
    let rm_value = rm.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;

    let result = match shift_by {
        ShiftBy::Immediate(shift_amount) => {
            let op2 = shift_type.shift(shift_amount, rm_value); // todo: implement shift

            rn_value & op2
        }
        ShiftBy::Register(rs) => {
            let rs = register_set.get(rs).ok_or(InstructionError::InvalidRegister(rs as u32))?;
            let rs_value = rs.read().map_err(|e| InstructionError::RegisterReadError(e.to_string()))?;

            let op2 = shift_type.shift(rs_value as u8, rm_value); // todo: implement shift

            rn_value & op2
        }
    };

    let mut rd = register_set.get(rd).ok_or(InstructionError::InvalidRegister(rd as u32))?;
    rd.write(result).map_err(|e| InstructionError::RegisterWriteError(e.to_string()))?;
    if s_flag {
        // Set flags
        todo!()
    }
    Ok(())

}

#[cfg(test)]
mod tests {
    use crate::register::RegisterCell;

    use super::*;

    #[test]
    fn test_and_immediate_no_shift() {

        let expected_value: u32 = 1 & 4;
        let register_set = RegisterSet::builder()
            .with_register(0, RegisterCell::new(1)).unwrap() // Rn
            .with_register(2, RegisterCell::new(0)).unwrap() // Rd
            .build();

        and_immediate(register_set.clone(), false, 2, 0, 0, 4).unwrap();
        assert_eq!(register_set.clone().get(2).unwrap().read().unwrap(), expected_value);
    }

    #[test]
    fn test_add_immediate_with_shift() {
        let nn: u32 = 4;
        let expected_value: u32 = 1 & nn.rotate_right(2);
        let register_set = RegisterSet::builder()
            .with_register(0, RegisterCell::new(1)).unwrap() // Rn
            .with_register(2, RegisterCell::new(0)).unwrap() // Rd
            .build();

        and_immediate(register_set.clone(), false, 2, 0, 2, 4).unwrap();
        assert_eq!(register_set.clone().get(2).unwrap().read().unwrap(), expected_value);
    }
}