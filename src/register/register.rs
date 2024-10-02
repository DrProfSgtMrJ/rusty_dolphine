use super::{Mode, RegisterError, RegisterMap, RegisterSet};

pub trait ReadRegister {
    fn read(&self) -> Result<u32, RegisterError>;
}

pub trait WriteRegister {
    fn write(&mut self, value: u32) -> Result<(), RegisterError>;
}

pub fn read_register_map(register_map: &RegisterMap, mode: Mode, register: u8) -> Result<u32, RegisterError> {
    match register_map.get(mode.clone()) {
        Some(register_set) => {
            read_register_set(&register_set, register)
        },
        None => {return Err(RegisterError::InvalidMode(mode.clone().to_string()));}
    }
}

pub fn write_register_map(register_map: &mut RegisterMap, mode: Mode, register: u8, value: u32) -> Result<(), RegisterError> {
    match register_map.get(mode.clone()) {
        Some(mut register_set) => {
            write_register_set(&mut register_set, register, value)
        },
        None => {return Err(RegisterError::InvalidMode(mode.clone().to_string()));}
    }
}

pub fn write_register_set(register_set: &mut RegisterSet, register: u8, value: u32) -> Result<(), RegisterError> {
    match register_set.get(register) {
        Some(mut register) => {
            register.write(value)
        },
        None => {return Err(RegisterError::InvalidRegister(register));}
    }
}


pub fn read_register_set(register_set: &RegisterSet, register: u8) -> Result<u32, RegisterError> {
    match register_set.get(register) {
        Some(register) => {
            register.read()
        },
        None => {return Err(RegisterError::InvalidRegister(register));}
    }
}