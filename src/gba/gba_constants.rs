// Register Constants
pub const REGISTER_0: &str = "r0";
pub const REGISTER_1: &str = "r1";
pub const REGISTER_2: &str = "r2";
pub const REGISTER_3: &str = "r3";
pub const REGISTER_4: &str = "r4";
pub const REGISTER_5: &str = "r5";
pub const REGISTER_6: &str = "r6";
pub const REGISTER_7: &str = "r7";
pub const REGISTER_8: &str = "r8";
pub const REGISTER_9: &str = "r9";
pub const REGISTER_10: &str = "r10";
pub const REGISTER_11: &str = "r11";
pub const REGISTER_12: &str = "r12";
pub const REGISTER_13: &str = "r13";
pub const REGISTER_14: &str = "r14";
pub const REGISTER_15: &str = "r15";

pub const REGISTER_8_FIQ: &str = "r8_fiq";
pub const REGISTER_9_FIQ: &str = "r9_fiq";
pub const REGISTER_10_FIQ: &str = "r10_fiq";
pub const REGISTER_11_FIQ: &str = "r11_fiq";
pub const REGISTER_12_FIQ: &str = "r12_fiq";
pub const REGISTER_13_FIQ: &str = "r13_fiq";
pub const REGISTER_14_FIQ: &str = "r14_fiq";

pub const REGISTER_13_SVC: &str = "r13_svc";
pub const REGISTER_14_SVC: &str = "r14_svc";

pub const REGISTER_13_ABT: &str = "r13_abt";
pub const REGISTER_14_ABT: &str = "r14_abt";

pub const REGISTER_13_IRQ: &str = "r13_IRQ";
pub const REGISTER_14_IRQ: &str = "r14_IRQ";

pub const REGISTER_13_UND: &str = "r13_UND";
pub const REGISTER_14_UND: &str = "r14_UND";


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






