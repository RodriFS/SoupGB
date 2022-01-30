use crate::interrupts::GeneralInterrupts;
use crate::ppu::RenderProps;

use super::apu;
use super::constants::*;
use super::gpu;
use super::memory::Memory;
use super::registers::Registers;
use super::timers;
use super::timers::Timers;

pub struct Emulator {
  pub background_debug: bool,
  pub sprites_debug: bool,
  pub window_debug: bool,
  pub debug: bool,
  pub registers: Registers,
  pub memory: Memory,
  pub timers: Timers,
  pub line_buffer: [(u8, u8); SCREEN_WIDTH],
  pub frame_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
  pub interrupts: GeneralInterrupts,
  pub render_props: RenderProps,
}

impl Emulator {
  pub fn default() -> Self {
    let memory = Memory::default();
    Self {
      background_debug: true,
      sprites_debug: true,
      window_debug: true,
      debug: false,
      registers: Registers::default(),
      memory: Memory::default(),
      timers: Timers::default(),
      line_buffer: [(0, 0); SCREEN_WIDTH],
      frame_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
      interrupts: GeneralInterrupts::default(),
      render_props: RenderProps::new(&memory)
    }
  }

  pub fn debug(&mut self) {
    self.debug = !self.debug;
  }

  pub fn toggle_background(&mut self) {
    self.background_debug = !self.background_debug;
  }

  pub fn toggle_sprites(&mut self) {
    self.sprites_debug = !self.sprites_debug;
  }

  pub fn toggle_window(&mut self) {
    self.window_debug = !self.window_debug;
  }

  pub fn take_cycle(&mut self) {
    GeneralInterrupts::run(self);
    gpu::update(self);
    timers::update(self);
    apu::update(self);
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
    let lo = self.memory.read(self.memory.get_pc()) as u16;
    self.take_cycle();
    self.memory.inc_pc(1);
    let hi = self.memory.read(self.memory.get_pc()) as u16;
    self.take_cycle();
    self.memory.inc_pc(1);
    (hi << 8) | lo
  }

  pub fn get_byte(&mut self) -> u8 {
    let byte = self.memory.read(self.memory.get_pc());
    self.take_cycle();
    self.memory.inc_pc(1);
    byte
  }

  pub fn fetch_opcode(&mut self) -> u8 {
    let byte = self.memory.read(self.memory.get_pc());
    if self.timers.halt_bug {
      self.timers.halt_bug = false
    } else {
      self.memory.inc_pc(1);
    }
    byte
  }

  pub fn s_push(&mut self, data: u16) {
    let bytes = data.to_be_bytes();
    self.memory.dec_sp(1);
    self.memory.write(self.memory.stack_pointer, bytes[0]);
    self.take_cycle();
    self.memory.dec_sp(1);
    self.memory.write(self.memory.stack_pointer, bytes[1]);
    self.take_cycle();
  }

  pub fn s_push_hi(&mut self, data: u16) {
    let bytes = data.to_be_bytes();
    self.memory.dec_sp(1);
    self.memory.write(self.memory.stack_pointer, bytes[0]);
    self.take_cycle();
  }

  pub fn s_push_lo(&mut self, data: u16) {
    let bytes = data.to_be_bytes();
    self.memory.dec_sp(1);
    self.memory.write(self.memory.stack_pointer, bytes[1]);
    self.take_cycle();
  }

  pub fn s_pop(&mut self) -> u16 {
    let byte1 = self.memory.read(self.memory.stack_pointer);
    self.memory.inc_sp(1);
    self.take_cycle(); // check if before or after increment
    let byte2 = self.memory.read(self.memory.stack_pointer);
    self.memory.inc_sp(1);
    self.take_cycle(); // check if before or after increment
    (byte2 as u16) << 8 | byte1 as u16
  }
}
