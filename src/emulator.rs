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

impl Emulator {
  pub fn default() -> Self {
    Self {
      registers: Registers::default(),
      memory: Memory::default(),
      timers: Timers::default(),
      frame_buffer: Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT),
    }
  }

  pub fn take_cycle(&mut self) {
    gpu::update(self, 4);
    timers::update(self, 4);
    interrupts::update(self);
    self.memory.dma_copy_byte();
  }

  pub fn load_rom(&mut self, buffer: Vec<u8>) {
    self.memory.load_rom(buffer);
  }

  pub fn mem_read(&mut self, address: u16) -> u8 {
    let r = self.memory.read(address);
    self.take_cycle();
    r
  }

  pub fn mem_write(&mut self, address: u16, data: u8) {
    self.memory.write(address, data);
    self.take_cycle();
  }

  pub fn write_word(&mut self, address: u16, data: u16) {
    let bytes = data.to_be_bytes();
    self.memory.write(address, bytes[1]);
    self.take_cycle();
    self.memory.write(address.wrapping_add(1), bytes[0]);
    self.take_cycle();
  }

  pub fn get_word(&mut self) -> u16 {
    let lo = self.memory.get_byte() as u16;
    self.take_cycle();
    let hi = self.memory.get_byte() as u16;
    self.take_cycle();
    (hi << 8) | lo
  }

  pub fn get_byte(&mut self) -> u8 {
    let byte = self.memory.get_byte();
    self.take_cycle();
    byte
  }

  pub fn push_to_stack(&mut self, data: u16) {
    let bytes = data.to_be_bytes();
    self.memory.decrement_stack_pointer(2);
    self.memory.write(self.memory.stack_pointer, bytes[1]);
    self.take_cycle();
    self
      .memory
      .write(self.memory.stack_pointer.wrapping_add(1), bytes[0]);
    self.take_cycle();
  }

  pub fn pop_from_stack(&mut self) -> u16 {
    let byte1 = self.memory.read(self.memory.stack_pointer);
    self.take_cycle();
    let byte2 = self.memory.read(self.memory.stack_pointer.wrapping_add(1));
    self.take_cycle();
    self.memory.increment_stack_pointer(2);
    (byte2 as u16) << 8 | byte1 as u16
  }
}
