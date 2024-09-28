use cpu::init_gba_cpu;

mod cpu;
mod memory;
mod register;


fn main() {
    init_gba_cpu();    
    println!("Hello, world!");
}
