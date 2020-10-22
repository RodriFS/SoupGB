#[derive(Default)]
pub struct Sweep {
  internal_counter: usize,
  enabled: bool,
  shadow_freq: u16,
  sweep_period: u8,
  shift: u8,
  negate: u8,
}

impl Sweep {
  fn copy_freq(&mut self, nrx3: u8, nrx4: u8) {
    self.shadow_freq = ((nrx4 & 0b111) as u16) << 8 & nrx3 as u16;
  }

  fn reload_timer(&mut self) {
    self.internal_counter = 32768;
  }

  fn set_enabled(&mut self) {
    if self.shift == 0 && self.sweep_period == 0 {
      self.enabled = false;
    } else {
      self.enabled = true;
    }
  }

  fn freq_calc(&mut self) -> u16 {
    let mut new_freq = self.shadow_freq >> self.shift;
    if self.negate == 1 {
      new_freq = !new_freq;
    }
    new_freq.wrapping_add(self.shadow_freq)
  }

  fn overflow_check(&mut self, new_freq: u16) -> bool {
    new_freq > 2047 && self.shift != 0
  }

  pub fn perform_checks(&mut self) -> u16 {
    if self.enabled && self.sweep_period != 0 {
      let new_freq = self.freq_calc();
      let overflow = self.overflow_check(new_freq);
      if !overflow && self.shift != 0 {
        self.shadow_freq = new_freq;
      }
      return 0;
    }
    self.shadow_freq
  }

  pub fn trigger(&mut self, nrx0: u8, nrx3: u8, nrx4: u8) {
    self.sweep_period = (nrx0 >> 4) & 0b111;
    self.shift = nrx0 & 0b111;
    self.negate = (nrx0 >> 3) & 0b1;
    self.copy_freq(nrx3, nrx4);
    self.reload_timer();
    self.set_enabled();
    self.perform_checks();
  }

  pub fn update(&mut self) -> u16 {
    let (counter, overflow) = self.internal_counter.overflowing_sub(4);
    if overflow {
      self.reload_timer();
      self.perform_checks()
    } else {
      self.internal_counter = counter;
      self.shadow_freq
    }
  }
}
