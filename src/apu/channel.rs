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
  lc: u16,
  sweep: Sweep,
  timer: ApuTimer,
  pub len_ctr_mask: u16,
}

impl Channel {
  pub fn new(len_ctr_mask: u16, frame_seq: Rc<RefCell<FrameSequencer>>) -> Self {
    Self {
      nrx0: 0,
      nrx1: 0,
      nrx2: 0,
      nrx3: 0,
      nrx4: 0,
      lc: 0,
      sweep: Sweep::default(),
      timer: ApuTimer::default(),
      len_ctr_mask,
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

  pub fn update(&mut self) -> bool {
    let counter = self.frame_seq.borrow().counter_256;
    self.dec_len_ctr(counter);
    self.lc == 0 || !self.get_len_enabled()
  }

  pub fn dec_len_ctr(&mut self, counter: u16) {
    if self.get_len_enabled() && counter == 0 && self.lc != 0 {
      self.lc = self.lc.wrapping_sub(1)
    }
  }

  pub fn get_len_enabled(&self) -> bool {
    get_bit_at(self.nrx4, 6)
  }

  pub fn set_nrx1(&mut self, data: u8) {
    self.nrx1 = data;
    self.lc = self.len_ctr_mask - (data & 0b0011_1111) as u16
  }

  pub fn set_nrx4(&mut self, data: u8) {
    if get_bit_at(data, 7) {
      if self.lc == 0 {
        self.lc = self.len_ctr_mask;
      }
      self.trigger();
    }
    self.nrx4 = data;
  }
}
