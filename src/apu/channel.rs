use super::super::utils::{clear_bit_at, get_bit_at};
use super::frame_seq::FrameSequencer;
use super::ApuTimer;
use super::Sweep;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum ChannelNr {
  Ch1 = 0,
  Ch2 = 1,
  Ch3 = 2,
  Ch4 = 3,
}

pub struct Channel {
  tag: ChannelNr,
  frame_seq: Rc<RefCell<FrameSequencer>>,
  pub nrx0: u8,
  pub nrx1: u8,
  pub nrx2: u8,
  pub nrx3: u8,
  pub nrx4: u8,
  lc: u16,
  sweep: Sweep,
  timer: ApuTimer,
  len_ctr_mask: u16,
  dac_power_mask: u8,
}

impl Channel {
  pub fn new(tag: ChannelNr, frame_seq: Rc<RefCell<FrameSequencer>>) -> Self {
    let len_ctr_mask = if tag == ChannelNr::Ch3 { 256 } else { 64 };
    let dac_power_mask = if tag == ChannelNr::Ch3 {
      0b1000_0000
    } else {
      0b1111_1000
    };
    Self {
      tag,
      nrx0: 0,
      nrx1: 0,
      nrx2: 0,
      nrx3: 0,
      nrx4: 0,
      lc: 0,
      sweep: Sweep::default(),
      timer: ApuTimer::default(),
      len_ctr_mask,
      dac_power_mask,
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
    let disable = self.lc == 0 || !self.is_len_enabled() || !self.is_dac_power_enabled();
    if self.lc == 0 {
      self.lc = self.len_ctr_mask;
    }
    disable
  }

  pub fn dec_len_ctr(&mut self, counter: u16) {
    if self.is_len_enabled() && counter == 0 && self.lc != 0 {
      self.lc = self.lc.wrapping_sub(1)
    }
  }

  pub fn disable_len_enable(&self) {
    clear_bit_at(self.nrx4, 6);
  }

  pub fn is_len_enabled(&self) -> bool {
    get_bit_at(self.nrx4, 6)
  }

  pub fn set_nrx1(&mut self, data: u8) {
    if self.is_len_enabled() {
      self.nrx1 = data;
      self.lc = self.len_ctr_mask - (data & 0b0011_1111) as u16
    }
  }

  pub fn set_nrx4(&mut self, data: u8) {
    if get_bit_at(data, 7) {
      self.trigger();
    }
    self.nrx4 = data;
  }

  pub fn is_dac_power_enabled(&self) -> bool {
    if self.tag == ChannelNr::Ch3 {
      self.nrx0 & self.dac_power_mask != 0
    } else {
      self.nrx2 & self.dac_power_mask != 0
    }
  }
}
