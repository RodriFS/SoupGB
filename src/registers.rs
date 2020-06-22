pub struct Registers {
  pub a: u8,
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
}

impl Registers {
  pub fn default() -> Self {
    Self {
      a: 0x11,
      f: 0x80,
      b: 0x00,
      c: 0x00,
      d: 0x00,
      e: 0x08,
      h: 0x00,
      l: 0x7C,
    }
  }
}
