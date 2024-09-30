use core::fmt;
use std::collections::HashMap;

use super::{MemoryError, MemorySector};



#[derive(Debug, Default, Clone)]
pub struct MemoryBus {
    // Memory map: start_address -> MemorySector
    memory_map: HashMap<u32, MemorySector>
}

impl MemoryBus {
    pub fn sector(&self, address: u32) -> Option<&MemorySector> {
        self.memory_map.values().find(|sector| {
            sector.start_address <= address && sector.end_address >= address
        })
    }

    pub fn total_memory(&self) -> usize {
        // adding + 1 since the addresses are inclusive
        self.memory_map.values().fold(0, |acc, sector| acc + sector.size())
    }

    pub fn builder() -> MemoryBusBuilder {
        MemoryBusBuilder::new()
    }
}

impl fmt::Display for MemoryBus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, sector) in self.memory_map.iter() {
            write!(f, "{}\n", sector)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct MemoryBusBuilder {
    memory_bus:  MemoryBus
}

impl MemoryBusBuilder {
    pub fn new() -> MemoryBusBuilder {
        MemoryBusBuilder {
            memory_bus: MemoryBus::default()
        }
    }

    pub fn sector_with_size(&mut self, name: String, start_address: u32, size: usize) -> Result<&mut Self, MemoryError>{
        if let Some(_) = self.memory_bus.sector(start_address) {
            return Err(MemoryError::OverlappingMemorySectors(start_address));
        }

        if let Some(_) = self.memory_bus.sector(start_address + size as u32) {
            return Err(MemoryError::OverlappingMemorySectors(start_address + size as u32));
        }

        let sector = MemorySector::with_size(name, start_address, size as usize).map_err(|e| e)?;
        self.memory_bus.memory_map.insert(start_address, sector);
        Ok(self)
    }

    pub fn sector_with_addresses(&mut self, name: String, start_address: u32, end_address: u32) -> Result<&mut Self, MemoryError> {
        if let Some(_) = self.memory_bus.sector(start_address) {
            return Err(MemoryError::OverlappingMemorySectors(start_address));
        }

        if let Some(_) = self.memory_bus.sector(end_address) {
            return Err(MemoryError::OverlappingMemorySectors(end_address));
        }

        let sector = MemorySector::with_addresses(name, start_address, end_address).map_err(|e| e)?;
        self.memory_bus.memory_map.insert(start_address, sector);
        Ok(self)
    }

    pub fn build(&self) -> MemoryBus {
        self.memory_bus.clone()
    }
}

#[cfg(test)]
mod tests {


    use super::*;
    use crate::gba::{BIOS_START, BIOS_END, BIOS_SIZE, WRAM_ONBOARD_SIZE, WRAM_ONBOARD_START, WRAM_ONBOARD_END};

    #[test]
    fn test_memory_bus_builder_sector_with_size() {
        let mut builder = MemoryBus::builder();
        builder.sector_with_size("BIOS".to_string(), BIOS_START, BIOS_SIZE).unwrap();

        let memory_bus = builder.build();
        assert_eq!(memory_bus.memory_map.len(), 1);
        assert_eq!(memory_bus.total_memory(), BIOS_SIZE);

        // check valid sector was added
        let middle_of_bios = BIOS_START + 100;
        let sector = memory_bus.sector(middle_of_bios).unwrap();
        assert_eq!(sector.start_address, BIOS_START);
        assert_eq!(sector.end_address, BIOS_END);
    }

    #[test]
    fn test_memory_bus_builder_multiple_sector_with_size() {
        let mut builder = MemoryBus::builder();
        builder
            .sector_with_size("BIOS".to_string(), BIOS_START, BIOS_SIZE).unwrap()
            .sector_with_size("WRAM_ONBOARD_SIZE".to_string(), WRAM_ONBOARD_START, WRAM_ONBOARD_SIZE).unwrap();

        let memory_bus = builder.build();
        assert_eq!(memory_bus.memory_map.len(), 2);
        assert_eq!(memory_bus.total_memory(), BIOS_SIZE + WRAM_ONBOARD_SIZE);

        let middle_of_bios = BIOS_START + 100;
        let bios_sector = memory_bus.sector(middle_of_bios).unwrap();
        assert_eq!(bios_sector.start_address, BIOS_START);
        assert_eq!(bios_sector.end_address, BIOS_END);
        assert_eq!(bios_sector.size(), BIOS_SIZE);

        let middle_of_wram = WRAM_ONBOARD_START + 10000;
        let wram_sector = memory_bus.sector(middle_of_wram).unwrap();
        assert_eq!(wram_sector.start_address, WRAM_ONBOARD_START);
        assert_eq!(wram_sector.end_address, WRAM_ONBOARD_END);
        assert_eq!(wram_sector.size(), WRAM_ONBOARD_SIZE);
    }

    #[test]
    fn test_memory_bus_builder_sector_with_addresses() {
        let mut builder = MemoryBus::builder();
        builder.sector_with_addresses("BIOS".to_string(), BIOS_START, BIOS_END).unwrap();

        let memory_bus = builder.build();
        assert_eq!(memory_bus.memory_map.len(), 1);
        assert_eq!(memory_bus.total_memory(), BIOS_SIZE);

        // check valid sector was added
        let sector = memory_bus.sector(BIOS_START).unwrap();
        assert_eq!(sector.start_address, BIOS_START);
        assert_eq!(sector.size(), BIOS_SIZE);
    }

    #[test]
    fn test_memory_bus_builder_inavlid_sector() {
        let mut builder = MemoryBus::builder();
        builder.sector_with_size("BIOS".to_string(), BIOS_START, BIOS_SIZE).unwrap();

        let middle_of_bios = BIOS_START + 100;
        match builder.sector_with_size("Invalid".to_string(), middle_of_bios, BIOS_SIZE) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::OverlappingMemorySectors(middle_of_bios))
        }

        match builder.sector_with_addresses("Invalid".to_string(), middle_of_bios, BIOS_END) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::OverlappingMemorySectors(middle_of_bios))
        }
    }

}
