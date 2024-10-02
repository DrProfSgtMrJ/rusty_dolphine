use core::fmt;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use super::{Mode, ReadRegister, RegisterError, WriteRegister, CPSR};

#[derive(Debug, Clone, Default)]
pub struct RegisterCell {
    pub value: Rc<RefCell<u32>>
}

impl RegisterCell {
    pub fn new(value: u32) -> RegisterCell {
        RegisterCell {
            value: Rc::new(RefCell::new(value))
        }
    }
}

impl ReadRegister for RegisterCell {
    fn read(&self) -> Result<u32, RegisterError> {
        match self.value.try_borrow() {
            Ok(value) => Ok(*value),
            Err(e) => Err(RegisterError::RegisterBorrowError(e.to_string()))
        }
    }
}

impl WriteRegister for RegisterCell {
    fn write(&mut self, value: u32) -> Result<(), RegisterError> {
        *self.value.try_borrow_mut().map_err(|e| RegisterError::RegisterBorrowError(e.to_string()))? = value;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CPSRCell {
    pub value: Rc<RefCell<CPSR>>
}

impl CPSRCell {
    pub fn new(cpsr: CPSR) -> CPSRCell {
        CPSRCell {
            value: Rc::new(RefCell::new(cpsr))
        }
    }
}

impl ReadRegister for CPSRCell {
    fn read(&self) -> Result<u32, RegisterError> {
        match self.value.try_borrow() {
            Ok(value) => Ok(value.bits()),
            Err(e) => Err(RegisterError::RegisterBorrowError(e.to_string()))
        }
    }
}

impl WriteRegister for CPSRCell {
    fn write(&mut self, value: u32) -> Result<(), RegisterError> {
        match self.value.try_borrow_mut() {
            Ok(mut cpsr) => {
                *cpsr = CPSR::from_bits(value).ok_or(RegisterError::InvalidCPSR(value))?;
                return Ok(());
            },
            Err(e) => {
                return Err(RegisterError::RegisterBorrowError(e.to_string()));
            }
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct RegisterSet {
    // R{u8} -> register
    pub registers: HashMap<u8, RegisterCell>,
    pub cpsr: CPSRCell,
    pub spsr: CPSRCell
}

impl RegisterSet {
    pub fn builder() -> RegisterSetBuilder {
        RegisterSetBuilder::default()
    }

    pub fn get(&self, name: u8) -> Option<RegisterCell> {
        match self.registers.get(&name) {
            Some(value) => Some(value.clone()),
            None => None
        }
    }
}

impl fmt::Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Registers: \n")?;
        for (name, value) in self.registers.iter() {
            write!(f, "R{}: {:?}\n", name, value)?;
        }
        write!(f, "CPSR: {:?}\n", self.cpsr)?;
        write!(f, "SPSR: {:?}\n", self.spsr)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct RegisterSetBuilder {
    register_set: RegisterSet
}

impl RegisterSetBuilder {

    pub fn with_register(&mut self, name: u8, register: RegisterCell) -> Result<&mut Self, RegisterError> {
        if self.register_set.registers.insert(name, register.clone()).is_some() {
            return Err(RegisterError::DuplicateRegister(name));
        }
        Ok(self)
    }

    pub fn with_cpsr(&mut self, cpsr: CPSRCell) -> Result<&mut Self, RegisterError> {
        self.register_set.cpsr = cpsr.clone();
        Ok(self)
    }

    pub fn with_spsr(&mut self, spsr: CPSRCell) -> Result<&mut Self, RegisterError> {
        self.register_set.spsr = spsr.clone();
        Ok(self)
    }

    pub fn build(&self) -> RegisterSet {
        self.register_set.clone()
    }
}


#[derive(Debug, Default, Clone)]
pub struct RegisterMap {
    pub register_sets: HashMap<Mode, RegisterSet>
}


#[derive(Debug, Default)]
pub struct RegisterMapBuilder {
    register_map: RegisterMap
}

impl RegisterMapBuilder {
    pub fn with_registers(&mut self, mode: Mode, register_set: RegisterSet) -> Result<&mut Self, RegisterError> {
        self.register_map.register_sets.insert(mode, register_set);
        Ok(self)
    }

    pub fn build(&self) -> RegisterMap {
        self.register_map.clone()
    }
}


impl RegisterMap {
    pub fn builder() -> RegisterMapBuilder {
        RegisterMapBuilder::default()
    }

    pub fn get(&self, mode: Mode) -> Option<RegisterSet> {
        match self.register_sets.get(&mode) {
            Some(reg_set) => Some(reg_set.clone()),
            None => None
        }
    }
}

impl fmt::Display for RegisterMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.register_sets)?;
        Ok(())
    }
}