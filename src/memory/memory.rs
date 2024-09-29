use super::{error::MemoryError, MemoryBus};


pub fn read_memory(memory_bus: &MemoryBus, address: u32, buf: &mut [u8]) -> Result<(), MemoryError> {
    match memory_bus.sector(address) {
        Some(sector) => {
            let data = sector.data.borrow();
            let offset = address - sector.start_address;
            let buf_len = buf.len();
            if offset as usize + buf_len > data.len() {
                return Err(MemoryError::OutOfBounds(address));
            }
            buf.copy_from_slice(&data[offset as usize..offset as usize + buf_len]);
        }
        None => {
            return Err(MemoryError::InvalidAddress(address));
        }
    }
    Ok(())
}

pub fn write_memory(memory_bus: &MemoryBus, address: u32, buf: &[u8]) -> Result<(), MemoryError> {
    match memory_bus.sector(address) {
        Some(sector) => {
            let mut data = sector.data.borrow_mut();
            let offset = address - sector.start_address;
            let buf_len = buf.len();
            if offset as usize + buf_len > data.len() {
                return Err(MemoryError::OutOfBounds(address));
            }
            data[offset as usize..offset as usize + buf_len].copy_from_slice(buf);
        }
        None => {
            return Err(MemoryError::InvalidAddress(address));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_memory() {
        let start_address = 0x00000000;
        let size = 1024;
        let memory_bus = MemoryBus::builder().sector_with_size("Test".to_string(), start_address, size).unwrap().build();
        let mut buf = [0; 4];
        read_memory(&memory_bus, start_address, &mut buf).unwrap();
        assert_eq!(buf, [0, 0, 0, 0]);

        // Now only read two bytes starting from the middle of the sector
        let mut buf_2: [u8; 2] = [0; 2];
        let middle_of_sector = start_address + size as u32 / 2;
        read_memory(&memory_bus, middle_of_sector, &mut buf_2).unwrap();
        assert_eq!(buf_2, [0, 0]);
    }

    #[test]
    fn test_read_memory_invalid_address() {
        let start_address = 0x00000000;
        let size = 1024;
        let memory_bus = MemoryBus::builder().sector_with_size("Test".to_string(), start_address, size).unwrap().build();
        let mut buf = [0; 4];
        let invalid_address = start_address + 1 + size as u32;
        match read_memory(&memory_bus, invalid_address, &mut buf) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::InvalidAddress(invalid_address))
        }
    }

    #[test]
    fn test_write_invalid_address() {
        let start_address = 0x00000000;
        let size = 1024;
        let memory_bus = MemoryBus::builder().sector_with_size("Test".to_string(), start_address, size).unwrap().build();
        let buf: [u8; 4] = [1, 2, 3, 4];
        let invalid_address = start_address + 1 + size as u32;
        match write_memory(&memory_bus, invalid_address, &buf) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::InvalidAddress(invalid_address))
        }
    }

    #[test]
    fn test_write_memory() {
        let start_address = 0x00000000;
        let size = 1024;
        let memory_bus = MemoryBus::builder().sector_with_size("Test".to_string(), start_address, size).unwrap().build();
        let buf: [u8; 4] = [1, 2, 3, 4];
        write_memory(&memory_bus, start_address, &buf).unwrap();

        let mut read_buf = [0; 4];
        assert_ne!(read_buf, buf);
        read_memory(&memory_bus, start_address, &mut read_buf).unwrap();
        assert_eq!(read_buf, buf);
    }

    #[test]
    fn test_read_out_of_bounds() {
        let start_address = 0x00000000;
        let size = 512;
        let memory_bus = MemoryBus::builder().sector_with_size("Test".to_string(), start_address, size).unwrap().build();
        let mut over_size_buf: [u8; 1024] = [0; 1024];

        match read_memory(&memory_bus, start_address, &mut over_size_buf) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::OutOfBounds(start_address))
        }
    }

    #[test]
    fn test_write_out_of_bounds() {
        let start_address = 0x00000000;
        let size = 512;
        let memory_bus = MemoryBus::builder().sector_with_size("Test".to_string(), start_address, size).unwrap().build();
        let over_size_buf: [u8; 1024] = [0; 1024];

        match write_memory(&memory_bus, start_address, &over_size_buf) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::OutOfBounds(start_address))
        }
    }
}