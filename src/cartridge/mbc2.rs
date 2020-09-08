use super::Cartridge;
use std::fmt;

pub struct MBC2 {
  rom: Vec<u8>,
  ram: Vec<u8>,
  memory_bank: u8,
  rom_size: u8,
  ram_size: u16,
  is_ram_enabled: bool,
}

impl MBC2 {
  pub fn new(data: Vec<u8>) -> Self {
    let mut rom_size = ((32 << data[0x148]) as f32 / 16.0) as u8;
    if rom_size > 16 {
      rom_size = 16;
    }
    Self {
      rom: data,
      ram: vec![0; 0x200],
      memory_bank: 1,
      rom_size,
      ram_size: 0x200,
      is_ram_enabled: false,
    }
  }

  fn read_rom(&self, address: u16, bank: u8) -> u8 {
    let mut rom_address = address;
    if rom_address > 0x3fff {
      rom_address -= 0x4000;
    }
    self
      .rom
      .get((rom_address as u32 + (bank as u32 * 0x4000)) as usize)
      .unwrap_or(&0xff)
      .to_owned()
  }

  fn read_ram(&self, address: u16) -> u8 {
    let ram_address = (address - 0xa000) % 0x200;
    self
      .ram
      .get(ram_address as usize)
      .unwrap_or(&0xff)
      .to_owned()
      | 0xf0
  }

  fn write_ram(&mut self, address: u16, data: u8) {
    let ram_address = (address - 0xa000) % 0x200;
    self.ram[ram_address as usize] = data | 0xf0;
  }

  fn set_bank1(&mut self, data: u8) {
    let lower_bits = data & 0xf;
    if lower_bits == 0 {
      self.memory_bank = lower_bits + 1
    } else {
      self.memory_bank = lower_bits
    }
  }
}

impl Cartridge for MBC2 {
  fn read(&self, address: u16) -> u8 {
    match address {
      0x0000..=0x3fff => self.read_rom(address, 0),
      0x4000..=0x7fff => self.read_rom(address, self.memory_bank % self.rom_size),
      0xa000..=0xbfff => self.read_ram(address),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, data: u8) {
    match address {
      0x0000..=0x3fff if address >> 8 & 0b1 == 0 => match data & 0xf {
        0b1010 => self.is_ram_enabled = true,
        _ => self.is_ram_enabled = false,
      },
      0x0000..=0x3fff if address >> 8 & 0b1 == 1 => self.set_bank1(data),
      0x4000..=0x5fff => {}
      0x6000..=0x7fff => {}
      0xa000..=0xbfff => self.write_ram(address, data),
      _ => unreachable!(),
    };
  }

  fn ram_enabled(&self) -> bool {
    self.is_ram_enabled
  }

  fn debug(&self) {
    println!("{:?}", self);
  }
}

impl fmt::Debug for MBC2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "CARTRIDGE ------------------------\n\
        type: MBC2\n\
        Bank: {}\n\
        ROM Size: {}\n\
        RAM Size: {:X}\n\
        RAM Enabled: {}\n",
      self.memory_bank, self.rom_size, self.ram_size, self.is_ram_enabled
    )
  }
}
