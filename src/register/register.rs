use core::fmt;
use std::collections::{HashMap, HashSet};

use super::{Mode, RegisterError, CPSR};

pub trait Register {

    // reads the value
    fn read(&self, buf: &mut u32) -> Result<(), RegisterError>;

    // writes the value to a register
    fn write(&mut self, buf: &u32) -> Result<(), RegisterError>;

    // Move the value from one register to another
    fn move_to(&self, other: &mut dyn Register) -> Result<(), RegisterError>;

    fn clear(&mut self) -> Result<(), RegisterError>;
}


#[derive(Debug, Clone, Default, PartialEq, Hash, Eq)]
pub struct NormalRegister {
    pub value: u32
}

impl NormalRegister {
    pub fn new(value: u32) -> Self {
        NormalRegister {
            value: value
        }
    }
}

impl fmt::Display for NormalRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

impl Register for NormalRegister {
    fn read(&self, buf: &mut u32) -> Result<(), RegisterError> {
        *buf = self.value;
        Ok(())
    }

    fn write(&mut self, buf: &u32) -> Result<(), RegisterError> {
        self.value = *buf; 
        Ok(())
    }

    fn move_to(&self, other: &mut dyn Register) -> Result<(), RegisterError> {
        let mut buf = 0;
        self.read(&mut buf)?;
        other.write(&buf)
    }

    fn clear(&mut self) -> Result<(), RegisterError> {
        self.write(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::{NormalRegister, Register};


    #[test]
    fn test_read() {
        let normal_reg = NormalRegister::new(1);

        let mut buf: u32 = 0;
        match normal_reg.read(&mut buf) {
            Ok(()) => {assert_eq!(buf, 1);}
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_write() {
        let mut normal_reg = NormalRegister::default();

        let buf: u32 = 100;

        match normal_reg.write(&buf) {
            Ok(()) => {assert_eq!(normal_reg.value, 100);}
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_move_to() {
        let mut normal_reg = NormalRegister::default();
        let other = NormalRegister::new(5);

        match other.move_to(&mut normal_reg) {
            Ok(()) => {assert_eq!(normal_reg.value, 5);}
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_clear() {
        let mut reg = NormalRegister::new(5);

        assert_eq!(reg.value, 5);

        match reg.clear() {
            Ok(()) => {assert_eq!(reg.value, 0);}
            Err(_) => assert!(false)
        }

    }
}