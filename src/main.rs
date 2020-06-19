use gba::constants::*;
use gba::cpu::Cpu;
use gba::debugger::steps;
use gba::gpu::Gpu;
use gba::interrupts::Interrupts;
use gba::memory::Memory;
use gba::timers::Timers;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

struct Emulator {
    cpu: Cpu,
    timers: Timers,
    memory: Rc<RefCell<Memory>>,
    interrupts: Rc<RefCell<Interrupts>>,
    gpu: Gpu,
}

impl Emulator {
    fn new() -> Self {
        let memory = Rc::new(RefCell::new(Memory::default()));
        let interrupts = Rc::new(RefCell::new(Interrupts::new(Rc::clone(&memory))));
        Self {
            cpu: Cpu::new(Rc::clone(&memory), Rc::clone(&interrupts)),
            timers: Timers::new(Rc::clone(&memory), Rc::clone(&interrupts)),
            gpu: Gpu::new(Rc::clone(&memory), Rc::clone(&interrupts)),
            interrupts,
            memory,
        }
    }
}

fn main() {
    let mut emulator = Emulator::new();
    let mut args: Vec<String> = std::env::args().collect();
    let mut rom = File::open(args.pop().unwrap()).unwrap();
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    emulator.memory.borrow_mut().load_rom(buffer);

    let refresh = std::time::Duration::from_secs_f64(1.0 / FPS as f64);

    loop {
        let mut frame_cycles = 0;
        while frame_cycles < MAXCYCLES {
            let opcode_cycles = emulator.cpu.update();
            frame_cycles += opcode_cycles;
            emulator.timers.update(opcode_cycles);
            emulator.gpu.update(opcode_cycles);
            emulator.interrupts.borrow_mut().update();
            steps();
        }
        std::thread::sleep(refresh);
    }
}
