#[derive(Default)]
pub struct ApuTimer {
  internal_counter: u8,
  period: u8,
  mask: u16,
}

impl ApuTimer {
  pub fn trigger(&mut self, nrx2: u8) {
    let period = nrx2 & 0b111;
    self.period = period;
  }

  pub fn update(&mut self, input: u16) -> u16 {
    let (counter, overflow) = self.internal_counter.overflowing_sub(4);
    if overflow {
      self.internal_counter = 4;
      self.mask = !self.mask;
    } else {
      self.internal_counter = counter;
    }
    input & self.mask
  }
}
