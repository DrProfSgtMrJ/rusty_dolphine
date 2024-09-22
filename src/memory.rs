const KBytes: usize = 1024;
const BIOS_SIZE: usize = 16 * KBytes;
const WRAM_ONBOARD_SIZE: usize = 256 * KBytes;
const WRAM_ONCHIP_SIZE: usize = 32 * KBytes;
const IO_REGISTERS_SIZE: usize = 1 * KBytes;

pub trait Memory {
    fn read_byte(&self, address: usize) -> u8;
    fn write_byte(&mut self, address: usize, value: u8);
}


#[derive(Debug)]
pub struct GeneralInternalMemory {
    pub bios: [u8; BIOS_SIZE], // // 00000000-00003FFF (16 KBytes)
    // 00004000-01FFFFFF   Not used
    pub wram_onboard: [u8; WRAM_ONBOARD_SIZE], // 02000000-0203FFFF (256 KBytes)  On-board Work RAM
    // 02040000-02FFFFFF   Not used
    pub wram_onchip: [u8; WRAM_ONCHIP_SIZE], // 03000000-03007FFF (32 KBytes)  On-chip Work RAM
    // 03008000-03FFFFFF   Not used
    io_registers: [u8; IO_REGISTERS_SIZE], // 04000000-040003FE I/O Registers
    // 04000400-04FFFFFF   Not used
}

impl Default for GeneralInternalMemory {
    fn default() -> Self {
        GeneralInternalMemory {
            bios: [0; BIOS_SIZE],
            wram_onboard: [0; WRAM_ONBOARD_SIZE],
            wram_onchip: [0; WRAM_ONCHIP_SIZE],
            io_registers: [0; IO_REGISTERS_SIZE],
        }
    }
}

impl Memory for GeneralInternalMemory {
    fn read_byte(&self, address: usize) -> u8 {
        match address {
            0x00000000..=0x00003FFF => self.bios[address],
            0x02000000..=0x0203FFFF => self.wram_onboard[address - 0x02000000],
            0x03000000..=0x03007FFF => self.wram_onchip[address - 0x03000000],
            0x04000000..=0x040003FE => self.io_registers[address - 0x04000000],
            _ => panic!("Invalid memory address: 0x{:08X}", address),
        }
    }

    fn write_byte(&mut self, address: usize, value: u8) {
        match address {
            0x00000000..=0x00003FFF => self.bios[address] = value,
            0x02000000..=0x0203FFFF => self.wram_onboard[address - 0x02000000] = value,
            0x03000000..=0x03007FFF => self.wram_onchip[address - 0x03000000] = value,
            0x04000000..=0x040003FE => self.io_registers[address - 0x04000000] = value,
            _ => panic!("Invalid memory address: 0x{:08X}", address),
        }
    }
}

const PALLETE_RAM_SIZE: usize = 1 * KBytes;
const VRAM_SIZE: usize = 96 * KBytes;
const OAM_SIZE: usize = 1 * KBytes;

pub struct InternalDisplayMemory {
    pub Bg_Pallete_Ram: [u8; PALLETE_RAM_SIZE], // 05000000-050003FF BG/OBJ Palette RAM (1 Kbyte) 
    // 05000400-05FFFFFF Not used
    pub vram: [u8; VRAM_SIZE], // 06000000-06017FFF VRAM (96 KBytes)
    // 06018000-06FFFFFF Not used
    pub oam: [u8; OAM_SIZE], // 07000000-070003FF OBJ Attributes
    // 07000400-07FFFFFF Not used
}

impl Default for InternalDisplayMemory {
    fn default() -> Self {
        InternalDisplayMemory {
            Bg_Pallete_Ram: [0; PALLETE_RAM_SIZE],
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
        }
    }
}

impl Memory for InternalDisplayMemory {
    fn read_byte(&self, address: usize) -> u8 {
        match address {
            0x05000000..=0x050003FF => self.Bg_Pallete_Ram[address - 0x05000000],
            0x06000000..=0x06017FFF => self.vram[address - 0x06000000],
            0x07000000..=0x070003FF => self.oam[address - 0x07000000],
            _ => panic!("Invalid memory address: 0x{:08X}", address),
        }
    }

    fn write_byte(&mut self, address: usize, value: u8) {
        match address {
            0x05000000..=0x050003FF => self.Bg_Pallete_Ram[address - 0x05000000] = value,
            0x06000000..=0x06017FFF => self.vram[address - 0x06000000] = value,
            0x07000000..=0x070003FF => self.oam[address - 0x07000000] = value,
            _ => panic!("Invalid memory address: 0x{:08X}", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_byte() {
        let mut memory = GeneralInternalMemory::default();
        // check if the memory is zeroed out
        assert_eq!(memory.read_byte(0x00000000), 0x00);

        memory.write_byte(0x00000000, 0x12);
        assert_eq!(memory.read_byte(0x00000000), 0x12);
    }

    #[test]
    #[should_panic]
    fn test_read_invalid_address() {
        let mut memory = GeneralInternalMemory::default();
        memory.read_byte(0x2040000);

        memory.write_byte(0x2040000, 0x20);
    }
}