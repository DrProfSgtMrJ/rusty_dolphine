use crate::register::Registers;

#[derive(Debug, Clone)]
pub enum CpuState {

    // Use full 32-bit instructions
    ARM,

    // Use 16-bit instructions
    THUMB,

    // Undefined state
    UNDEFINED,
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState::UNDEFINED
    }
}


#[derive(Debug, Default, Clone)]
pub struct CPU {
    registers: Registers,
}
