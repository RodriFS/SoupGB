use super::super::utils::get_bit_at;
use super::frame_seq::FrameSequencer;
use super::ApuTimer;
use super::Sweep;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Channel {
  frame_seq: Rc<RefCell<FrameSequencer>>,
  pub nrx0: u8,
  pub nrx1: u8,
  pub nrx2: u8,
  pub nrx3: u8,
  pub nrx4: u8,
  sweep: Sweep,
  timer: ApuTimer,
  pub len_ctr_mask: u8,
  enable_bit: u8,
}

impl Channel {
  pub fn new(len_ctr_mask: u8, enable_bit: u8, frame_seq: Rc<RefCell<FrameSequencer>>) -> Self {
    Self {
      nrx0: 0,
      nrx1: 0,
      nrx2: 0,
      nrx3: 0,
      nrx4: 0,
      sweep: Sweep::default(),
      timer: ApuTimer::default(),
      len_ctr_mask,
      enable_bit,
      frame_seq,
    }
  }

  pub fn trigger(&mut self) {
    self.sweep.trigger(self.nrx0, self.nrx3, self.nrx4);
    self.timer.trigger(self.nrx2);
  }

  pub fn get_output(&mut self) -> u16 {
    let output = self.sweep.update();
    self.timer.update(output)
  }

  pub fn update(&mut self) -> u8 {
    let counter = self.frame_seq.borrow().counter_256;
    self.dec_len_ctr(counter);
    if self.get_len_ctr() == 0 || !self.get_len_enabled() {
      0 << self.enable_bit
    } else {
      1 << self.enable_bit
    }
  }

  fn get_len_ctr(&self) -> u8 {
    self.nrx1 & self.len_ctr_mask
  }

  fn set_len_ctr(&mut self, length_counter: u8) {
    let duty = self.nrx1 & !self.len_ctr_mask;
    self.nrx1 = duty | length_counter;
  }

  pub fn dec_len_ctr(&mut self, counter: u16) {
    let length_counter = self.get_len_ctr();
    if self.get_len_enabled() && counter == 0 && length_counter != 0 {
      self.set_len_ctr(length_counter.wrapping_sub(1))
    }
  }

  pub fn get_len_enabled(&self) -> bool {
    get_bit_at(self.nrx4, 6)
  }

  pub fn set_nrx1(&mut self, data: u8) {
    let length_counter = data & self.len_ctr_mask;
    let duty = data & !self.len_ctr_mask;
    println!(
      "data: {}, mask: {}, written: {} or {}",
      data,
      self.len_ctr_mask,
      (self.len_ctr_mask as u16)
        .wrapping_add(1)
        .wrapping_sub(length_counter as u16) as u8,
      (-(length_counter as i8)) as u8
    );
    self.nrx1 = duty | (-(length_counter as i8)) as u8
    // self.nrx1 = duty
    //   | (self.len_ctr_mask as u16)
    //     .wrapping_add(1)
    //     .wrapping_sub(length_counter as u16) as u8
  }

  pub fn set_nrx4(&mut self, data: u8) {
    if get_bit_at(data, 7) {
      if self.get_len_ctr() == 0 {
        self.set_len_ctr(self.len_ctr_mask);
      }
      self.trigger();
    }
    self.nrx4 = data;
  }
}
