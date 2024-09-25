use crate::memory::MemoryBus;
use crate::register::RegisterSet;

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
    register_set: RegisterSet,
    memory_bus: MemoryBus,
}
