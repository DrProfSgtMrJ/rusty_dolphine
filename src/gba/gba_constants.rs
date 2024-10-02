// Register Constants
pub const REGISTER_0: u8 = 0;
pub const REGISTER_1: u8 = 1;
pub const REGISTER_2: u8 = 2;
pub const REGISTER_3: u8 = 3;
pub const REGISTER_4: u8 = 4;
pub const REGISTER_5: u8 = 5;
pub const REGISTER_6: u8 = 6;
pub const REGISTER_7: u8 = 7;
pub const REGISTER_8: u8 = 8;
pub const REGISTER_9: u8 = 9;
pub const REGISTER_10: u8 = 10;
pub const REGISTER_11: u8 = 11;
pub const REGISTER_12: u8 = 12;
pub const REGISTER_13: u8 = 13;
pub const REGISTER_14: u8 = 14;
pub const REGISTER_15: u8 = 15;


pub const KBYTES: usize = 1024;

// Sizes
// General Internal Memory
pub const BIOS: &str = "BIOS - System ROM";
pub const BIOS_SIZE: usize = 16 * KBYTES;
pub const BIOS_START: u32 = 0x00000000;
pub const BIOS_END: u32 = 0x00003FFF;

pub const WRAM: &str = "WRAM - Work RAM";
pub const WRAM_ONBOARD_SIZE: usize = 256 * KBYTES;
pub const WRAM_ONBOARD_START: u32 = 0x02000000;
pub const WRAM_ONBOARD_END: u32 = 0x0203FFFF;

pub const WRAM_ONCHIP: &str = "WRAM - On-chip Work RAM";
pub const WRAM_ONCHIP_SIZE: usize = 32 * KBYTES;
pub const WRAM_ONCHIP_START: u32 = 0x03000000;
pub const WRAM_ONCHIP_END: u32 = 0x03007FFF;

pub const IO_REGISTERS: &str = "IO REGISTERS";

// 1023 bytes (according to spec)
pub const IO_REGISTERS_SIZE: usize = (1 * KBYTES) - 1;
pub const IO_REGISTERS_START: u32 = 0x04000000;
pub const IO_REGISTERS_END: u32 = 0x040003FE;


// Internal Display Memory
pub const PALLETE_RAM: &str = "PALLETE RAM";
pub const PALLETE_RAM_SIZE: usize = 1 * KBYTES;

pub const VRAM: &str = "VRAM - Video RAM";
pub const VRAM_SIZE: usize = 96 * KBYTES;

pub const OAM: &str = "OAM - Object Attributes";
pub const OAM_SIZE: usize = 1 * KBYTES;






