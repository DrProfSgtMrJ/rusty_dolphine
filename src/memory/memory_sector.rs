use core::fmt;
use std::{cell::RefCell, rc::Rc};
use super::MemoryError;

#[derive(Debug, Clone)]
pub struct MemorySector {
    pub name: String,
    pub start_address: u32,
    pub end_address: u32,
    pub data: Rc<RefCell<Vec<u8>>>,
}

impl fmt::Display for MemorySector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MemorySector: {{ name: {}, start_address: 0x{:08X}, end_address: 0x{:08X}, size: {} }}", self.name, self.start_address, self.end_address, self.size())
    }
}

impl MemorySector {

    pub fn with_size(name: String, start_address: u32, size: usize) -> Result<MemorySector, MemoryError>{
        // make sure the size + start_address does not overflow
        if let Some(end_address) = start_address.checked_add(size as u32) {
            return Ok(MemorySector {
                        name: name,
                        start_address: start_address,
                        end_address: end_address - 1,
                        data: Rc::new(RefCell::new(vec![0; size]))
                    });
        }
        Err(MemoryError::InvalidSize(size))
    }

    pub fn with_addresses(name: String, start_address: u32, end_address: u32) -> Result<MemorySector, MemoryError> {

        // makes sure there is no underflow
        if let Some(size) = end_address.checked_sub(start_address) {
            let size = size + 1;
            return Ok(
                MemorySector {
                    name: name,
                    start_address: start_address,
                    end_address: end_address,
                    data: Rc::new(RefCell::new(vec![0; size as usize]))
                }
            );
        }
        Err(MemoryError::InvalidAddresses(start_address, end_address))
    }

    pub fn size(&self) -> usize {
        self.data.borrow().len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::gba::KBYTES;

    #[test]
    fn test_memory_sector_with_size() {
        let expected_start_address = 0x00000000 as u32;
        let expected_end_address = expected_start_address + KBYTES as u32;
        let sector = MemorySector::with_size("Test".to_string(), expected_start_address, KBYTES).unwrap();
        assert_eq!(sector.start_address, expected_start_address as u32);
        assert_eq!(sector.end_address, expected_end_address - 1);
        assert_eq!(sector.size(), KBYTES);
    }

    #[test]
    fn test_memory_sector_with_addresses() {
        let expected_start_address = 0x00000000 as u32;
        let expected_end_address = expected_start_address + (KBYTES - 1) as u32;
        let sector = MemorySector::with_addresses("Test".to_string(), expected_start_address, expected_end_address).unwrap();
        println!("{}", sector);
        assert_eq!(sector.start_address, expected_start_address as u32);
        assert_eq!(sector.end_address, expected_end_address);
        assert_eq!(sector.size(), KBYTES);
    }

    #[test]
    fn test_memory_sector_invalid_size() {
        // expected to overflow 
        let expected_size = u32::MAX as usize;
        let expected_start_address = 0x01000000;
        match MemorySector::with_size("Test".to_string(), expected_start_address, expected_size) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::InvalidSize(expected_size))
        }
    }

    #[test]
    fn test_memory_sector_invalid_addresses() {
        // expected underflow
        let start_address = 50 as u32;
        let end_address = u32::MIN;

        match MemorySector::with_addresses("Test".to_string(), start_address, end_address) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, MemoryError::InvalidAddresses(start_address, end_address))
        }
    }
}