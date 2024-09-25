use core::fmt;
use std::collections::{HashMap, HashSet};

use super::{Mode, CPSR};


#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Register {
    pub name: String,
    pub value: u32
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl Register {
    pub fn new(name: String, value: u32) -> Self {
        Register { name: name, value: value }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RegisterSet {
    // Can be accessed by ANY mode
    pub general_purpose_registers: HashSet<Register>,

    // Mode specific registers
    pub banked_registers: HashMap<Mode, HashSet<Register>>,

    pub cpsr: CPSR
}

impl fmt::Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "General Purpose Registers: \n")?;
        for gpr in self.general_purpose_registers.iter() {
            write!(f, "{}\n", gpr)?;
        }

        write!(f, "Banked Registers: \n")?;
        for (mode, br) in self.banked_registers.iter() {
            write!(f, "Mode: {}:\n", mode.to_string())?;
            for r in br.iter() {
                write!(f, "{}\n", r)?;
            }
        }

        write!(f, "{}", self.cpsr)?;
        Ok(())
    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let register = Register::new("r1".to_string(), 0);

        println!("{}", register);
    }

    #[test]
    fn test_register_set() {
        let mut general_purpose: HashSet<Register> = HashSet::new();
        let mut banked_set: HashSet<Register> = HashSet::new();
        let mut banked: HashMap<Mode, HashSet<Register>> = HashMap::new();
        for i in 0..10 {
            general_purpose.insert(Register::new(i.to_string(), 0));
            banked_set.insert(Register::new(format!("{}_fiq", i), 0));
        }

        banked.insert(Mode::FIQ, banked_set);

        let mut cpsr = CPSR::default();

        cpsr |= CPSR::N | CPSR::J;
        // Set some flags
        let reg_set = RegisterSet {general_purpose_registers: general_purpose, banked_registers: banked, cpsr: cpsr};
        println!("{}", reg_set);

    }
}