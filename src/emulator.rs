use super::constants::*;
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
  pub frame_buffer: Vec<u32>,
}

pub fn take_cycle(emu: &mut Emulator) {
  gpu::update(emu, 4);
  timers::update(&mut emu.timers, &mut emu.memory, 4);
  interrupts::update(&mut emu.timers, &mut emu.memory);
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

  pub fn get_word(&mut self) -> u16 {
    let lo = self.memory.get_byte() as u16;
    take_cycle(self);
    let hi = self.memory.get_byte() as u16;
    take_cycle(self);
    (hi << 8) | lo
  }

  pub fn get_byte(&mut self) -> u8 {
    let byte = self.memory.get_byte();
    take_cycle(self);
    byte
  }

  pub fn push_to_stack(&mut self, data: u16) {
    let bytes = data.to_be_bytes();
    self.memory.decrement_stack_pointer(2);
    self.memory.write(self.memory.stack_pointer, bytes[1]);
    take_cycle(self);
    self
      .memory
      .write(self.memory.stack_pointer.wrapping_add(1), bytes[0]);
    take_cycle(self);
  }

  pub fn pop_from_stack(&mut self) -> u16 {
    let byte1 = self.memory.read(self.memory.stack_pointer);
    take_cycle(self);
    let byte2 = self.memory.read(self.memory.stack_pointer.wrapping_add(1));
    take_cycle(self);
    self.memory.increment_stack_pointer(2);
    (byte2 as u16) << 8 | byte1 as u16
  }
}
