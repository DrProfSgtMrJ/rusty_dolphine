use crate::register::RegisterMap;
use crate::memory::MemoryBus;


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


#[derive(Debug, Default)]
pub struct CPU {
    pub register_map: RegisterMap,
    pub memory_bus: MemoryBus,
}

impl CPU {
    pub fn new(register_map: RegisterMap, memory_bus: MemoryBus) -> CPU {
        CPU { register_map, memory_bus }
    }
}