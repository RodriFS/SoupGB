pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod rom_only;

pub trait Cartridge {
  fn write(&mut self, address: u16, data: u8);
  fn read(&self, address: u16) -> u8;
  fn ram_enabled(&self) -> bool {
    false
  }
  fn debug(&self);
}

#[derive(PartialEq, Debug, Clone)]
pub enum Bmode {
  RAM,
  ROM,
}
