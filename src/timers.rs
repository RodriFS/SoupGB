use super::constants::*;
use super::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Timers {
  memory: Rc<RefCell<Memory>>,
  clock_frequency: u32,
  timer_counter: u32,
}

impl Timers {
  pub fn new(memory: Rc<RefCell<Memory>>) -> Self {
    let clock_frequency = 4096;
    Self {
      memory,
      clock_frequency,
      timer_counter: CLOCKSPEED / clock_frequency,
    }
  }

  fn get_is_clock_enabled(&self) -> bool {
    let clock = self.memory.borrow().read(TIMER_CONTROLLER_ADDRESS);
    clock & 0x3 == 1
  }

  fn update_devider_register(&self, _: u32) {}
  fn set_clock_frequency(&self) {}
  fn request_interrupt(&self, _: u8) {}

  pub fn update(&mut self, frame_cycles: u32) {
    self.update_devider_register(frame_cycles);

    if !self.get_is_clock_enabled() {
      return;
    }

    self.timer_counter -= frame_cycles;

    if self.timer_counter > 0 {
      return;
    }

    self.set_clock_frequency();

    if self.memory.borrow().read(TIMER_ADDRESS) == 0xff {
      self.memory.borrow_mut().write(
        TIMER_ADDRESS,
        self.memory.borrow().read(TIMER_MODULATOR_ADDRESS),
      );
      self.request_interrupt(2);
    } else {
      self
        .memory
        .borrow_mut()
        .write(TIMER_ADDRESS, self.memory.borrow().read(TIMER_ADDRESS) + 1);
    }
  }
}
