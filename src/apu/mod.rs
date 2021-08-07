pub mod apu_timer;
pub mod channel;
pub mod frame_seq;
pub mod sweep;

use super::apu::channel::{Channel, ChannelNr};
use super::constants::*;
use super::emulator::Emulator;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};
use apu_timer::ApuTimer;
use frame_seq::FrameSequencer;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use sweep::Sweep;

pub struct Apu {
  sender: Option<Sender<f32>>,
  frame_seq: Rc<RefCell<FrameSequencer>>,
  pub channel1: Channel,
  pub channel2: Channel,
  pub channel3: Channel,
  pub channel4: Channel,
  nr50: u8,
  nr51: u8,
  nr52: u8,
  wave_table: [u8; 16],
}

impl Apu {
  pub fn default() -> Self {
    let frame_seq = Rc::new(RefCell::new(FrameSequencer::new()));
    Self {
      sender: None,
      channel1: Channel::new(ChannelNr::Ch1, frame_seq.clone()),
      channel2: Channel::new(ChannelNr::Ch2, frame_seq.clone()),
      channel3: Channel::new(ChannelNr::Ch3, frame_seq.clone()),
      channel4: Channel::new(ChannelNr::Ch4, frame_seq.clone()),
      frame_seq,
      nr50: 0,
      nr51: 0,
      nr52: 0,
      wave_table: [0; 16],
    }
  }

  pub fn read(&self, address: u16) -> u8 {
    match address {
      NR10 => self.channel1.nrx0 | 0x80,
      NR11 => self.channel1.nrx1 | 0x3f,
      NR12 => self.channel1.nrx2,
      NR13 => self.channel1.nrx3 | 0xff,
      NR14 => self.channel1.nrx4 | 0xbf,
      NR20 => self.channel2.nrx0 | 0xff,
      NR21 => self.channel2.nrx1 | 0x3f,
      NR22 => self.channel2.nrx2,
      NR23 => self.channel2.nrx3 | 0xff,
      NR24 => self.channel2.nrx4 | 0xbf,
      NR30 => self.channel3.nrx0 | 0x7f,
      NR31 => self.channel3.nrx1 | 0xff,
      NR32 => self.channel3.nrx2 | 0x9f,
      NR33 => self.channel3.nrx3 | 0xff,
      NR34 => self.channel3.nrx4 | 0xbf,
      NR40 => self.channel4.nrx0 | 0xff,
      NR41 => self.channel4.nrx1 | 0xff,
      NR42 => self.channel4.nrx2,
      NR43 => self.channel4.nrx3,
      NR44 => self.channel4.nrx4 | 0xbf,
      NR50 => self.nr50,
      NR51 => self.nr51,
      NR52 => self.nr52 | 0x70,
      0xff30..=0xff3f => self.wave_table[(address - 0xff30) as usize],
      _ => 0xff,
    }
  }

  pub fn write(&mut self, address: u16, data: u8) {
    if self.is_apu_off(address) {
      return;
    }
    println!("write: {:x} - {:b}", address, data);
    println!("NR52: {:b}", self.nr52);
    match address {
      NR10 => self.channel1.nrx0 = data,
      NR11 => self.channel1.set_nrx1(data),
      NR12 => {
        self.channel1.nrx2 = data;
        if !self.channel1.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch1);
        }
      }
      NR13 => self.channel1.nrx3 = data,
      NR14 => {
        if get_bit_at(data, 6) {
          self.enable_channel(ChannelNr::Ch1);
        }
        self.channel1.set_nrx4(data);
        if !self.channel1.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch1);
        }
      }
      NR20 => self.channel2.nrx0 = data,
      NR21 => self.channel2.set_nrx1(data),
      NR22 => {
        self.channel2.nrx2 = data;
        if !self.channel2.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch2);
        }
      }
      NR23 => self.channel2.nrx3 = data,
      NR24 => {
        if get_bit_at(data, 6) {
          self.enable_channel(ChannelNr::Ch2);
        }
        self.channel2.set_nrx4(data);
        if !self.channel2.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch2);
        }
      }
      NR30 => {
        self.channel3.nrx0 = data;
        if !self.channel3.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch3);
        }
      }
      NR31 => self.channel3.set_nrx1(data),
      NR32 => self.channel3.nrx2 = data,
      NR33 => self.channel3.nrx3 = data,
      NR34 => {
        if get_bit_at(data, 6) {
          self.enable_channel(ChannelNr::Ch3);
        }
        self.channel3.set_nrx4(data);
        if !self.channel3.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch3);
        }
      }
      NR40 => self.channel4.nrx0 = data,
      NR41 => self.channel4.set_nrx1(data),
      NR42 => {
        self.channel4.nrx2 = data;
        if !self.channel4.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch4);
        }
      }
      NR43 => self.channel4.nrx3 = data,
      NR44 => {
        if get_bit_at(data, 6) {
          self.enable_channel(ChannelNr::Ch4);
        }
        self.channel4.set_nrx4(data);
        if !self.channel4.is_dac_power_enabled() {
          self.disable_channel(ChannelNr::Ch4);
        }
      }
      NR50 => self.nr50 = data,
      NR51 => self.nr51 = data,
      NR52 => self.write_power_control(data),
      0xff30..=0xff3f => self.wave_table[(address - 0xff30) as usize] = data,
      _ => {}
    };
  }

  fn is_apu_off(&self, address: u16) -> bool {
    address != NR52 && address < 0xff30 && self.nr52 >> 7 == 0
  }

  fn enable_channel(&mut self, channel: ChannelNr) {
    self.nr52 = set_bit_at(self.nr52, channel as u8);
  }

  fn disable_channel(&mut self, channel: ChannelNr) {
    self.nr52 = clear_bit_at(self.nr52, channel as u8);
  }

  fn write_power_control(&mut self, data: u8) {
    let power = data & 0b1000_0000;
    if power == 0 {
      for reg in NR10..=NR51 {
        self.write(reg, 0);
      }
    }
    self.nr52 = power | 0b0000_1111
  }

  fn update(&mut self) {
    self.frame_seq.borrow_mut().update();
    if self.channel1.update() {
      self.disable_channel(ChannelNr::Ch1);
    }
    if self.channel2.update() {
      self.disable_channel(ChannelNr::Ch2);
    }
    if self.channel3.update() {
      self.disable_channel(ChannelNr::Ch3);
    }
    if self.channel4.update() {
      self.disable_channel(ChannelNr::Ch4);
    }
  }

  pub fn load_sender(&mut self, sender: Sender<f32>) {
    self.sender = Some(sender);
  }
}

pub fn update(ctx: &mut Emulator) {
  ctx.memory.apu.update();
  let channel1 = &mut ctx.memory.apu.channel1;
  let output = channel1.get_output();
  ctx
    .memory
    .apu
    .sender
    .as_ref()
    .unwrap()
    .send(output as f32)
    .unwrap();
}
