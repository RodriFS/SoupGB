use super::Cartridge;
use chrono::{Datelike, Timelike, Utc};
use std::fmt;

pub struct MBC3 {
  rom: Vec<u8>,
  ram: Vec<u8>,
  rom_bank: u8,
  ram_bank: u8,
  rom_size: u8,
  ram_size: u16,
  is_ram_enabled: bool,
  prev_bit: u8,
  sec_reg: u8,
  min_reg: u8,
  hrs_reg: u8,
  dayl_reg: u8,
  dayh_reg: u8,
}

impl MBC3 {
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
      rom_bank: 1,
      ram_bank: 0,
      rom_size: (rom_size as f32 / 16.0) as u8,
      ram_size: ram_size as u16,
      is_ram_enabled: false,
      prev_bit: 0,
      sec_reg: 0,
      min_reg: 0,
      hrs_reg: 0,
      dayl_reg: 0,
      dayh_reg: 0,
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
    match bank {
      0x0..=0x3 => {
        let ram_address = if self.ram_size > 0x1fff {
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
      0x8 => self.sec_reg,
      0x9 => self.min_reg,
      0xa => self.hrs_reg,
      0xb => self.dayl_reg,
      0xc => self.dayh_reg,
      _ => unreachable!(),
    }
  }

  fn write_ram(&mut self, address: u16, bank: u8, data: u8) {
    match bank {
      0x0..=0x3 => {
        let ram_address = if self.ram_size > 0x1fff {
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
      0x8 => self.sec_reg = data,
      0x9 => self.min_reg = data,
      0xa => self.hrs_reg = data,
      0xb => self.dayl_reg = data,
      0xc => self.dayh_reg = data,
      _ => unreachable!(),
    }
  }

  fn get_bank2_as_low(&self) -> u8 {
    self.rom_bank >> 5
  }

  fn set_bank1(&mut self, data: u8) {
    let bank = data & 0b0111_1111;
    if bank == 0 {
      self.rom_bank = bank + 1;
    } else {
      self.rom_bank = bank;
    }
  }

  fn set_bank2(&mut self, data: u8) {
    self.ram_bank = data;
  }
}

impl Cartridge for MBC3 {
  fn read(&self, address: u16) -> u8 {
    match address {
      0x0000..=0x3fff => self.read_rom(address, 0),
      0x4000..=0x7fff => self.read_rom(address, self.rom_bank % self.rom_size),
      0xa000..=0xbfff => self.read_ram(address, self.ram_bank),
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
        0x0 => self.prev_bit = 0,
        0x1 => {
          if self.prev_bit == 0 {
            let now = Utc::now();
            self.sec_reg = now.second() as u8;
            self.min_reg = now.minute() as u8;
            self.hrs_reg = now.hour() as u8;
            self.dayl_reg = now.day() as u8;
            self.dayh_reg = (now.day() >> 8 & 0b1) as u8;
          }
          self.prev_bit = 1;
        }
        _ => unreachable!(),
      },
      0xa000..=0xbfff => self.write_ram(address, self.get_bank2_as_low(), data),
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

impl fmt::Debug for MBC3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "CARTRIDGE ------------------------\n\
      type: MBC3\n\
      ROM Bank: {}\n\
      RAM Bank: {}\n\
      ROM Size: {}\n\
      RAM Size: {:X}\n\
      RAM Enabled: {}\n",
      self.rom_bank, self.ram_bank, self.rom_size, self.ram_size, self.is_ram_enabled
    )
  }
}
