#[derive(Default)]
pub struct FrameSequencer {
  pub counter_256: u16,
  pub value_256: u16,
}

impl FrameSequencer {
  pub fn new() -> Self {
    Self {
      counter_256: 16384,
      value_256: 16384,
    }
  }
  pub fn update(&mut self) {
    let (count, overflow) = self.counter_256.overflowing_sub(4);
    self.counter_256 = if overflow { self.value_256 } else { count };
    // println!("counter: {}", self.counter_256);
  }
}
