use gba::constants::*;
use gba::cpu::Cpu;
use gba::interrupts::Interrupts;
use gba::memory::Memory;
use gba::timers::Timers;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

struct Emulator {
    cpu: Cpu,
    timers: Timers, //memory: Rc<RefCell<Memory>>,
}

impl Emulator {
    fn new(buffer: Vec<u8>) -> Self {
        let interrupts = Rc::new(RefCell::new(Interrupts::new()));
        let memory = Rc::new(RefCell::new(Memory::new(buffer)));
        Self {
            cpu: Cpu::new(Rc::clone(&memory), Rc::clone(&interrupts)),
            timers: Timers::new(Rc::clone(&memory)), //memory,
        }
    }
}

fn main() {
    let mut rom = File::open("./test_rom/Pokemon Blue.gb").unwrap();
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    let mut emulator = Emulator::new(buffer);
    let refresh = std::time::Duration::from_secs_f64(1.0 / FPS as f64);

    loop {
        let frame_cycles = emulator.cpu.update();
        emulator.timers.update(frame_cycles);
        std::thread::sleep(refresh);
    }
}
