#[derive(Default)]
pub struct Clock {
  step: u32,
}

impl Clock {
  pub fn set_step(&mut self, step: u32) {
    self.step = step;
  }

  pub fn next(&self) -> u32 {
    self.step
  }
}
