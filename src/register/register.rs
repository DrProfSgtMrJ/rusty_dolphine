use super::{Mode, RegisterError, RegisterMap};


pub fn write_register(register_map: &mut RegisterMap, mode: Mode, register_name: &str, value: u32) -> Result<(), RegisterError> {
    match register_map.get(mode.clone()) {
        Some(register_set) => {
            match register_set.get(register_name.to_string()) {
                Some(register) => {
                    *register.borrow_mut() = value;
                    return Ok(());
                },
                None => {return Err(RegisterError::InvalidRegister(register_name.to_string()));}
            }
        },
        None => {
            return Err(RegisterError::InvalidMode(mode.clone().to_string()));
        }
    }
}

pub fn read_register(register_map: &RegisterMap, mode: Mode, register_name: &str) -> Result<u32, RegisterError> {
    match register_map.get(mode.clone()) {
        Some(register_set) => {
            match register_set.get(register_name.to_string()) {
                Some(register) => {
                    return Ok(*register.borrow());
                },
                None => {return Err(RegisterError::InvalidRegister(register_name.to_string()));}
            }
        },
        None => {
            return Err(RegisterError::InvalidMode(mode.clone().to_string()));
        }
    }
}