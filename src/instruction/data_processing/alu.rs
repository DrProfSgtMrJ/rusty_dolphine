use crate::register::{read_register_set, write_register_set, RegisterSet};

use super::super::{Instruction, InstructionError};

#[derive(Debug, Clone)]
pub struct AndImmediate {
    pub register_set: RegisterSet,
    pub rd: String,
    pub rn: String,
    pub op2: u32
}

impl AndImmediate {
    pub fn new(register_set: RegisterSet, rd: String, rn: String, op2: u32) -> AndImmediate {
        AndImmediate {
            register_set,
            rd,
            rn,
            op2
        }
    }
}

impl Instruction for AndImmediate {
    fn execute(&mut self) -> Result<(), InstructionError> {
       let rn = read_register_set(&self.register_set, &self.rn).map_err(|_| {
            InstructionError::InvalidRegister(self.rn.clone())
       })?;

       let result = rn & self.op2;

       write_register_set(&mut self.register_set, &self.rd, result).map_err(| _ |{
              InstructionError::InvalidRegister(self.rd.clone()) 
       })?;

       Ok(())
    }
}