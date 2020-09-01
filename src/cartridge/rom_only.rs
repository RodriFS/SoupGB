use super::Cartridge;

#[derive(Default)]
pub struct RomOnly {
  rom: Vec<u8>,
}

impl RomOnly {
  fn read_rom(&self, address: u16) -> u8 {
    self.rom.get(address as usize).unwrap_or(&0xff).to_owned()
  }
}

impl RomOnly {
  pub fn new(data: Vec<u8>) -> Self {
    Self { rom: data }
  }
}

impl Cartridge for RomOnly {
  fn read(&self, address: u16) -> u8 {
    self.read_rom(address)
  }

  fn write(&mut self, _address: u16, _data: u8) {}
}
