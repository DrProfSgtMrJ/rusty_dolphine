use core::fmt;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use super::{Mode, RegisterError, CPSR};


#[derive(Debug, Clone, Default)]
pub struct RegisterSet {
    // name -> register
    pub registers: HashMap<String, Rc<RefCell<u32>>>,
    pub cpsr: Rc<RefCell<CPSR>>,
    pub spsr: Rc<RefCell<CPSR>>
}

impl RegisterSet {
    pub fn builder() -> RegisterSetBuilder {
        RegisterSetBuilder::default()
    }

    pub fn get(&self, name: String) -> Option<Rc<RefCell<u32>>> {
        match self.registers.get(&name) {
            Some(value) => Some(value.clone()),
            None => None
        }
    }
}

impl fmt::Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Registers: \n")?;
        write!(f, "{:?}\n", self.registers)?;
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

    pub fn with_register(&mut self, name: String, register: Rc<RefCell<u32>>) -> Result<&mut Self, RegisterError> {
        if self.register_set.registers.insert(name.clone(), register.clone()).is_some() {
            return Err(RegisterError::DuplicateRegister(name));
        }
        Ok(self)
    }

    pub fn with_cpsr(&mut self, cpsr: Rc<RefCell<CPSR>>) -> Result<&mut Self, RegisterError> {
        self.register_set.cpsr = cpsr.clone();
        Ok(self)
    }

    pub fn with_spsr(&mut self, spsr: Rc<RefCell<CPSR>>) -> Result<&mut Self, RegisterError> {
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