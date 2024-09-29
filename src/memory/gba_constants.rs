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






