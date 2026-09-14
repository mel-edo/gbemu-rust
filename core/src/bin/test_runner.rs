use gb_core::cpu::Cpu;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut gb = Cpu::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut f = File::open("../cpu_instrs.gb").expect("Error opening ROM file");
    f.read_to_end(&mut buffer).expect("Error loading ROM");
    gb.load_rom(&buffer);

    for _ in 0..10_000_000 {
        gb.tick();
    }
}
