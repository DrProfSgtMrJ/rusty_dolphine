use cpu::init_gba_cpu;

mod cpu;
mod memory;
mod register;


fn main() {
    let gba_cpu = init_gba_cpu().expect("Failed to initialize GBA CPU");    
}
