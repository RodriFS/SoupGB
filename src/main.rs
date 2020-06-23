use gba::constants::*;
use gba::cpu;
use gba::debugger;
use gba::debugger::steps;
use gba::gpu;
use gba::interrupts;
use gba::memory::Memory;
use gba::registers::Registers;
use gba::timers;
use gba::timers::Timers;
use std::fs::File;
use std::io::Read;

struct Emulator {
    registers: Registers,
    memory: Memory,
    timers: Timers,
}

impl Emulator {
    fn new() -> Self {
        Self {
            registers: Registers::default(),
            memory: Memory::default(),
            timers: Timers::default(),
        }
    }
}

fn main() {
    let mut emulator = Emulator::new();
    let mut args: Vec<String> = std::env::args().collect();
    let mut rom = File::open(args.pop().unwrap()).unwrap();
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    emulator.memory.load_rom(buffer);

    let refresh = std::time::Duration::from_secs_f64(1.0 / FPS as f64);

    loop {
        let mut frame_cycles = 0;
        while frame_cycles < MAXCYCLES {
            debugger::print_debug_registers_info(&emulator.registers);
            debugger::print_debug_memory_info(&emulator.memory);
            let opcode_cycles = cpu::update(
                &mut emulator.memory,
                &mut emulator.timers,
                &mut emulator.registers,
            );
            frame_cycles += opcode_cycles;
            timers::update(&mut emulator.timers, &mut emulator.memory, opcode_cycles);
            gpu::update(&mut emulator.timers, &mut emulator.memory, opcode_cycles);
            interrupts::update(&mut emulator.timers, &mut emulator.memory);
            steps();
        }
        std::thread::sleep(refresh);
    }
}
