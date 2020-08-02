use super::constants::*;
use super::memory::Memory;
use super::registers::Registers;
use super::timers::Timers;

pub struct Emulator {
  pub registers: Registers,
  pub memory: Memory,
  pub timers: Timers,
  pub frame_buffer: Vec<u32>,
}

impl Emulator {
  pub fn default() -> Self {
    Self {
      registers: Registers::default(),
      memory: Memory::default(),
      timers: Timers::default(),
      frame_buffer: Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT),
    }
  }

  pub fn load_rom(&mut self, buffer: Vec<u8>) {
    self.memory.load_rom(buffer);
  }
}
