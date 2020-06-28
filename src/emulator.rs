use super::constants::*;
use super::cpu;
use super::debugger;
use super::debugger::steps;
use super::gpu;
use super::interrupts;
use super::memory::Memory;
use super::registers::Registers;
use super::timers;
use super::timers::Timers;
use std::sync::mpsc::{Receiver, Sender};

pub struct Emulator {
  registers: Registers,
  memory: Memory,
  timers: Timers,
}

impl Emulator {
  pub fn default() -> Self {
    Self {
      registers: Registers::default(),
      memory: Memory::default(),
      timers: Timers::default(),
    }
  }

  pub fn load_rom(&mut self, buffer: Vec<u8>) {
    self.memory.load_rom(buffer);
  }

  pub fn run(&mut self, provider: Sender<Vec<u32>>, receiver: Receiver<&str>) {
    let refresh = std::time::Duration::from_secs_f64(1.0 / FPS as f64);
    loop {
      let mut frame_cycles = 0;
      while frame_cycles < MAXCYCLES {
        debugger::print_debug_registers_info(&self.registers);
        debugger::print_debug_memory_info(&self.memory);
        let opcode_cycles = cpu::update(&mut self.memory, &mut self.timers, &mut self.registers);
        frame_cycles += opcode_cycles;
        timers::update(&mut self.timers, &mut self.memory, opcode_cycles);
        gpu::update(&provider, &mut self.timers, &mut self.memory, opcode_cycles);
        interrupts::update(&mut self.timers, &mut self.memory);
        steps();

        //provider.send("hello!").unwrap();
        if let Ok(message) = receiver.try_recv() {
          if message == "close" {
            std::process::exit(0);
          }
        }
      }
      std::thread::sleep(refresh);
    }
  }
}
