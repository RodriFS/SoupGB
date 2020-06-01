use gba::{cpu::Cpu, interrupts::Interrupts};
use std::fs::File;
use std::io::Read;

const FPS: f64 = 60.0;

fn main() {
    let mut rom = File::open("./test_rom/Pokemon Blue.gb").unwrap();
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();

    let mut cpu = Cpu::new(buffer);
    let interrupts = Interrupts::new();
    cpu.load(interrupts);
    let refresh = std::time::Duration::from_secs_f64(1.0 / FPS);

    loop {
        cpu.update();
        std::thread::sleep(refresh);
    }
}
