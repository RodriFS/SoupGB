use super::clock::Clock;
use super::constants::*;
use super::cpu;
use super::gpu;
use super::interrupts;
use super::memory::Memory;
use super::registers::Registers;
use super::timers;
use super::timers::Timers;

pub struct Emulator {
  pub registers: Registers,
  pub memory: Memory,
  pub timers: Timers,
  pub clock: Clock,
  pub frame_buffer: Vec<u32>,
}

impl Emulator {
  pub fn default() -> Self {
    Self {
      registers: Registers::default(),
      memory: Memory::default(),
      timers: Timers::default(),
      clock: Clock::default(),
      frame_buffer: Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT),
    }
  }

  pub fn load_rom(&mut self, buffer: Vec<u8>) {
    self.memory.load_rom(buffer);
  }
}

pub fn next(emulator: &mut Emulator, run_next_instr: bool) {
  if run_next_instr {
    cpu::update(emulator);
  }
  let step = emulator.clock.next();
  gpu::update(emulator, step);
  timers::update(&mut emulator.timers, &mut emulator.memory, step);
  interrupts::update(&mut emulator.timers, &mut emulator.memory);
}
