use crate::{memory::init_gba_memory_bus, register::init_gba_registers};

use super::{CpuError, CPU};


pub fn init_gba_cpu() -> Result<CPU, CpuError> {

    let register_map = init_gba_registers().map_err(|e| {
        return CpuError::InitError(format!("Failed to init registers {:?}", e));
    })?;

    let memory_bus = init_gba_memory_bus().map_err(|e| {
        return CpuError::InitError(format!("Failed to init memory bus {:?}", e));
    })?;

    Ok(CPU::new(register_map, memory_bus))
}