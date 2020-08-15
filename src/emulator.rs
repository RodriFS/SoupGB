use super::constants::*;
use super::gpu;
use super::interrupts;
use super::interrupts::request_interrupt;
use super::memory::{LcdMode, Memory};
use super::registers::Registers;
use super::timers;
use super::timers::Timers;
use std::iter::FromIterator;

#[allow(non_camel_case_types)]
pub enum Action {
  new_mode(LcdMode),
  interrupt_request(u8),
}

#[derive(Default)]
pub struct Dispatcher {
  actions_queue: Vec<Action>,
}

impl Dispatcher {
  fn run(emu: &mut Emulator) {
    let actions_queue = Vec::from_iter(emu.dispatcher.actions_queue.drain(..));
    for action in actions_queue {
      match action {
        Action::new_mode(mode) => emu.memory.set_lcd_status(mode),
        Action::interrupt_request(bit) => request_interrupt(emu, bit),
      }
    }
  }

  pub fn dispatch(&mut self, action: Action) {
    self.actions_queue.push(action);
  }
}

pub struct Emulator {
  pub registers: Registers,
  pub memory: Memory,
  pub timers: Timers,
  pub frame_buffer: Vec<u32>,
  pub dispatcher: Dispatcher,
}

impl Emulator {
  pub fn default() -> Self {
    Self {
      registers: Registers::default(),
      memory: Memory::default(),
      timers: Timers::default(),
      frame_buffer: Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT),
      dispatcher: Dispatcher::default(),
    }
  }

  pub fn take_cycle(&mut self) {
    Dispatcher::run(self);
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
    let lo = self.memory.read(self.memory.get_program_counter()) as u16;
    self.take_cycle();
    self.memory.increment_program_counter(1);
    let hi = self.memory.read(self.memory.get_program_counter()) as u16;
    self.take_cycle();
    self.memory.increment_program_counter(1);
    (hi << 8) | lo
  }

  pub fn get_byte(&mut self) -> u8 {
    let byte = self.memory.read(self.memory.get_program_counter());
    self.take_cycle();
    self.memory.increment_program_counter(1);
    byte
  }

  pub fn fetch_opcode(&mut self) -> u8 {
    let byte = self.memory.read(self.memory.get_program_counter());
    self.memory.increment_program_counter(1);
    byte
  }

  pub fn push_to_stack(&mut self, data: u16) {
    let bytes = data.to_be_bytes();
    self.memory.decrement_stack_pointer(1);
    self.memory.write(self.memory.stack_pointer, bytes[0]);
    self.take_cycle();
    self.memory.decrement_stack_pointer(1);
    self.memory.write(self.memory.stack_pointer, bytes[1]);
    self.take_cycle();
  }

  pub fn pop_from_stack(&mut self) -> u16 {
    let byte1 = self.memory.read(self.memory.stack_pointer);
    self.memory.increment_stack_pointer(1);
    self.take_cycle(); // check if before or after increment
    let byte2 = self.memory.read(self.memory.stack_pointer);
    self.memory.increment_stack_pointer(1);
    self.take_cycle(); // check if before or after increment
    (byte2 as u16) << 8 | byte1 as u16
  }
}
