
use crate::{memory::{MemoryBus, MemoryError, BIOS, BIOS_END, BIOS_START, IO_REGISTERS, IO_REGISTERS_END, IO_REGISTERS_START, WRAM, WRAM_ONBOARD_END, WRAM_ONBOARD_START, WRAM_ONCHIP, WRAM_ONCHIP_END, WRAM_ONCHIP_START}, register::{Mode, RegisterError, RegisterMap, RegisterSet, CPSR}};

use super::CpuError;

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