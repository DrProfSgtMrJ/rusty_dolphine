
mod cpu;
mod memory;
mod register;
mod gba;
mod instruction;

use gba::init_gba_cpu;


fn main() {
    let gba_cpu = init_gba_cpu().expect("Failed to initialize GBA CPU");    
}
