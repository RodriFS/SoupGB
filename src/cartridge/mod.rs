pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod rom_only;
use std::path::PathBuf;

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

pub trait Save {
  fn load(file_path: &PathBuf, ram_size: usize) -> Vec<u8> {
    match std::fs::read(file_path) {
      Ok(data) => {
        println!("loaded save");
        data
      }
      Err(_) => vec![0; ram_size],
    }
  }

  fn save(file_path: &PathBuf, data: &[u8]) {
    match std::fs::write(file_path, data) {
      Ok(_) => println!("Game Saved"),
      Err(e) => println!("Unable to save game: {}", e),
    };
  }
}
