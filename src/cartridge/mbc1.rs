use super::Bmode;
use super::Cartridge;
use std::fmt;

pub struct MBC1 {
  rom: Vec<u8>,
  ram: Vec<u8>,
  memory_bank: u8,
  rom_size: u8,
  ram_size: u16,
  banking_mode: Bmode,
  is_ram_enabled: bool,
}

impl MBC1 {
  pub fn new(data: Vec<u8>) -> Self {
    let rom_size = 32 << data[0x148];
    let ram_size = match data[0x149] {
      0 => 0,
      1 => 0x800,
      2 => 0x2000,
      3 => 0x8000,
      _ => panic!("Unsupported ram size"),
    };
    Self {
      rom: data,
      ram: vec![0; ram_size],
      memory_bank: 1,
      rom_size: (rom_size as f32 / 16.0) as u8,
      ram_size: ram_size as u16,
      banking_mode: Bmode::ROM,
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

  fn read_ram(&self, address: u16, bank: u8) -> u8 {
    let ram_address = if self.ram_size > 0x2000 {
      let ram_bank = bank % 4;
      (address - 0xa000) + (ram_bank as u16 * 0x2000)
    } else {
      address - 0xa000
    };
    self
      .ram
      .get(ram_address as usize)
      .unwrap_or(&0xff)
      .to_owned()
  }

  fn write_ram(&mut self, address: u16, bank: u8, data: u8) {
    let ram_address = if self.ram_size > 0x2000 {
      let ram_bank = bank % 4;
      (address - 0xa000) + (ram_bank as u16 * 0x2000)
    } else {
      address - 0xa000
    };
    if ram_address >= self.ram.len() as u16 {
      return;
    }
    self.ram[ram_address as usize] = data;
  }

  fn get_bank2_as_low(&self) -> u8 {
    self.memory_bank >> 5
  }

  fn get_bank2_as_hi(&self) -> u8 {
    self.memory_bank & 0b0110_0000
  }

  fn set_bank1(&mut self, data: u8) {
    let lower_bits = data & 0b0001_1111;
    let upper_bits = self.memory_bank & 0b0110_0000;
    let rom_bank = lower_bits | upper_bits;
    if lower_bits == 0 {
      self.memory_bank = rom_bank + 1;
    } else {
      self.memory_bank = rom_bank;
    }
  }

  fn set_bank2(&mut self, data: u8) {
    let lower_bits = self.memory_bank & 0b0001_1111;
    let upper_bits = (data & 0b0000_0011) << 5;
    let next_rom_bank = upper_bits | lower_bits;
    self.memory_bank = next_rom_bank;
  }
}

impl Cartridge for MBC1 {
  fn read(&self, address: u16) -> u8 {
    match address {
      0x0000..=0x3fff if self.banking_mode == Bmode::ROM => self.read_rom(address, 0),
      0x0000..=0x3fff if self.banking_mode == Bmode::RAM => {
        self.read_rom(address, self.get_bank2_as_hi() % self.rom_size)
      }
      0x4000..=0x7fff => self.read_rom(address, self.memory_bank % self.rom_size),
      0xa000..=0xbfff if self.banking_mode == Bmode::ROM => self.read_ram(address, 0),
      0xa000..=0xbfff if self.banking_mode == Bmode::RAM => {
        self.read_ram(address, self.get_bank2_as_low())
      }
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, data: u8) {
    match address {
      0x0000..=0x1fff => match data & 0xf {
        0b1010 => self.is_ram_enabled = true,
        _ => self.is_ram_enabled = false,
      },
      0x2000..=0x3fff => self.set_bank1(data),
      0x4000..=0x5fff => self.set_bank2(data),
      0x6000..=0x7fff => match data & 0b1 {
        0x0 => self.banking_mode = Bmode::ROM,
        0x1 => self.banking_mode = Bmode::RAM,
        _ => unreachable!(),
      },
      0xa000..=0xbfff if self.banking_mode == Bmode::ROM => self.write_ram(address, 0, data),
      0xa000..=0xbfff if self.banking_mode == Bmode::RAM => {
        self.write_ram(address, self.get_bank2_as_low(), data);
      }
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

impl fmt::Debug for MBC1 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "CARTRIDGE ------------------------\n\
      type: MBC1\n\
      Bank: {}\n\
      ROM Size: {}\n\
      RAM Size: {:X}\n\
      Banking Mode: {:?}\n\
      RAM Enabled: {}\n",
      self.memory_bank, self.rom_size, self.ram_size, self.banking_mode, self.is_ram_enabled,
    )
  }
}
