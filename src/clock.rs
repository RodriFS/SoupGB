pub struct Clock {
  pub clock_frequency: u32,
  pub divider_frequency: u32,
  pub timer_counter: u32,
  pub divider_counter: u32,
  pub scan_line_counter: u32,
  pub master_enabled: bool,
  pub is_halted: bool,
}

impl Clock {
  pub fn default() -> Self {
    let clock_frequency = 4096;
    let divider_frequency = 16384;
    Self {
      scan_line_counter: 0,
      clock_frequency,
      divider_frequency,
      timer_counter: 0,
      divider_counter: 0,
      master_enabled: false,
      is_halted: false,
    }
  }

  pub fn set_master_enabled_on(&mut self) {
    self.master_enabled = true;
  }
  pub fn clear_master_enabled(&mut self) {
    self.master_enabled = false;
  }
}
