use crate::memory::{MemoryBus, MemoryError, BIOS, BIOS_END, BIOS_START, IO_REGISTERS, IO_REGISTERS_END, IO_REGISTERS_START, WRAM, WRAM_ONBOARD_END, WRAM_ONBOARD_START, WRAM_ONCHIP, WRAM_ONCHIP_END, WRAM_ONCHIP_START};


pub fn init_gba_memory_bus() -> Result<MemoryBus, MemoryError> {
    let memory_bus = MemoryBus::builder()
        .sector_with_addresses(BIOS.to_string(), BIOS_START, BIOS_END)?
        .sector_with_addresses(WRAM.to_string(), WRAM_ONBOARD_START, WRAM_ONBOARD_END)?
        .sector_with_addresses(WRAM_ONCHIP.to_string(), WRAM_ONCHIP_START, WRAM_ONCHIP_END)?
        .sector_with_addresses(IO_REGISTERS.to_string(), IO_REGISTERS_START, IO_REGISTERS_END)?
        .build();

    Ok(memory_bus)
}

#[cfg(test)]
mod tests {

    use crate::memory::{BIOS_SIZE, IO_REGISTERS_SIZE, WRAM_ONBOARD_SIZE, WRAM_ONCHIP_SIZE};

    use super::*;

    #[test]
    fn test_gba_memory_bus() {
        let expected_total_size = BIOS_SIZE + WRAM_ONBOARD_SIZE + WRAM_ONCHIP_SIZE + IO_REGISTERS_SIZE;
        match init_gba_memory_bus() {
            Ok(memory_bus) => {
                match memory_bus.sector(BIOS_START) {
                    Some(sector) => {
                        assert_eq!(sector.size(), BIOS_SIZE);
                    }
                    None => {
                        assert!(false);
                    }
                }

                match memory_bus.sector(WRAM_ONBOARD_START) {
                    Some(sector) => {
                        assert_eq!(sector.size(), WRAM_ONBOARD_SIZE);
                    }
                    None => {
                        assert!(false);
                    }
                }

                match memory_bus.sector(WRAM_ONCHIP_START) {
                    Some(sector) => {
                        assert_eq!(sector.size(), WRAM_ONCHIP_SIZE);
                    }
                    None => {
                        assert!(false);
                    }
                }

                match memory_bus.sector(IO_REGISTERS_START) {
                    Some(sector) => {
                        assert_eq!(sector.size(), IO_REGISTERS_SIZE);
                    }
                    None => {
                        assert!(false);
                    }
                }

                assert_eq!(memory_bus.total_memory(), expected_total_size);

                // check invalid addresses
                // addresses that are not in use according to GBA spec
                let invalid_addresses = [0x00004000, 0x01FFFFFF, 0x02040000, 0x02FFFFFF, 0x03008000, 0x03FFFFFF, 0x04000400, 0x04FFFFFF];
                for invalid_address in invalid_addresses.iter() {
                    match memory_bus.sector(*invalid_address) {
                        Some(_) => {
                            assert!(false);
                        }
                        None => {
                            assert!(true);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to init memory bus {:?}", e);
                assert!(false);
            }
        }
    }
}