pub const KBYTES: usize = 1024;

// Sizes
// General Internal Memory
pub const BIOS_SIZE: usize = 16 * KBYTES;
pub const WRAM_ONBOARD_SIZE: usize = 256 * KBYTES;
pub const WRAM_ONCHIP_SIZE: usize = 32 * KBYTES;
pub const IO_REGISTERS_SIZE: usize = 1 * KBYTES;


// Internal Display Memory
pub const PALLETE_RAM_SIZE: usize = 1 * KBYTES;
pub const VRAM_SIZE: usize = 96 * KBYTES;
pub const OAM_SIZE: usize = 1 * KBYTES;




pub const BIOS_START: u32 = 0x00000000;
pub const BIOS_END: u32 = 0x00003FFF;

pub const WRAM_ONBOARD_START: u32 = 0x02000000;
pub const WRAM_ONBOARD_END: u32 = 0x0203FFFF;

pub const WRAM_ONCHIP_START: u32 = 0x03000000;
pub const WRAM_ONCHIP_END: u32 = 0x03007FFF;

pub const IO_REGISTERS_START: u32 = 0x04000000;
pub const IO_REGISTERS_END: u32 = 0x040003FE;