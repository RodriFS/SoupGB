use super::utils::get_bit_at;
use byteorder::{BigEndian, ByteOrder};

#[derive(PartialEq)]
pub enum Reg {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  AF,
  BC,
  DE,
  HL,
  N8,
  N16,
  SP,
}

pub enum Flags {
  Z,
  N,
  H,
  C,
}

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

  pub fn get_flag(&self, flag: Flags) -> u8 {
    match flag {
      Flags::Z => get_bit_at(self.get_f(), 7) as u8,
      Flags::N => get_bit_at(self.get_f(), 6) as u8,
      Flags::H => get_bit_at(self.get_f(), 5) as u8,
      Flags::C => get_bit_at(self.get_f(), 4) as u8,
    }
  }

  pub fn set_flag(&mut self, flag: Flags, value: bool) {
    let mask = match flag {
      Flags::Z => 0x80,
      Flags::N => 0x40,
      Flags::H => 0x20,
      Flags::C => 0x10,
    };
    if value {
      let new_value = self.get_f() | mask;
      self.set_f(new_value);
    } else {
      let new_value = self.get_f() & !(mask);
      self.set_f(new_value);
    };
  }

  pub fn get_reg_u8(&mut self, reg: &Reg) -> u8 {
    match reg {
      Reg::A => self.get_a(),
      Reg::B => self.get_b(),
      Reg::C => self.get_c(),
      Reg::D => self.get_d(),
      Reg::E => self.get_e(),
      Reg::H => self.get_h(),
      Reg::L => self.get_l(),
      _ => panic!("Unsupported fn get_reg_u8"),
    }
  }
  pub fn set_reg_u8(&mut self, reg: &Reg, data: u8) {
    match reg {
      Reg::A => {
        self.set_a(data);
      }
      Reg::B => {
        self.set_b(data);
      }
      Reg::C => {
        self.set_c(data);
      }
      Reg::D => {
        self.set_d(data);
      }
      Reg::E => {
        self.set_e(data);
      }
      Reg::H => {
        self.set_h(data);
      }
      Reg::L => {
        self.set_l(data);
      }
      _ => panic!("Unsupported fn set_reg_u8"),
    };
  }
  pub fn set_reg_u16(&mut self, reg: &Reg, data: u16) {
    match reg {
      Reg::AF => self.set_af(data),
      Reg::BC => self.set_bc(data),
      Reg::DE => self.set_de(data),
      Reg::HL => self.set_hl(data),
      _ => panic!("Unsupported fn set_reg_u16"),
    };
  }
  pub fn get_af(&self) -> u16 {
    BigEndian::read_u16(&[self.get_a(), self.get_f()])
  }
  pub fn get_bc(&self) -> u16 {
    BigEndian::read_u16(&[self.get_b(), self.get_c()])
  }
  pub fn get_de(&self) -> u16 {
    BigEndian::read_u16(&[self.get_d(), self.get_e()])
  }
  pub fn get_hl(&self) -> u16 {
    BigEndian::read_u16(&[self.get_h(), self.get_l()])
  }
  pub fn set_af(&mut self, data: u16) {
    let split = data.to_be_bytes();
    self.set_a(split[0]);
    self.set_f(0xf0 & split[1]);
  }
  pub fn set_bc(&mut self, data: u16) {
    let split = data.to_be_bytes();
    self.set_b(split[0]);
    self.set_c(split[1]);
  }
  pub fn set_de(&mut self, data: u16) {
    let split = data.to_be_bytes();
    self.set_d(split[0]);
    self.set_e(split[1]);
  }
  pub fn set_hl(&mut self, data: u16) {
    let split = data.to_be_bytes();
    self.set_h(split[0]);
    self.set_l(split[1]);
  }
  pub fn set_a(&mut self, data: u8) -> u8 {
    self.a = data;
    self.a
  }
  pub fn set_f(&mut self, data: u8) -> u8 {
    self.f = 0xf0 & data;
    self.f
  }
  pub fn set_b(&mut self, data: u8) -> u8 {
    self.b = data;
    self.b
  }
  pub fn set_c(&mut self, data: u8) -> u8 {
    self.c = data;
    self.c
  }
  pub fn set_d(&mut self, data: u8) -> u8 {
    self.d = data;
    self.d
  }
  pub fn set_e(&mut self, data: u8) -> u8 {
    self.e = data;
    self.e
  }
  pub fn set_h(&mut self, data: u8) -> u8 {
    self.h = data;
    self.h
  }
  pub fn set_l(&mut self, data: u8) -> u8 {
    self.l = data;
    self.l
  }
  pub fn get_a(&self) -> u8 {
    self.a
  }
  pub fn get_f(&self) -> u8 {
    self.f
  }
  pub fn get_b(&self) -> u8 {
    self.b
  }
  pub fn get_c(&self) -> u8 {
    self.c
  }
  pub fn get_d(&self) -> u8 {
    self.d
  }
  pub fn get_e(&self) -> u8 {
    self.e
  }
  pub fn get_h(&self) -> u8 {
    self.h
  }
  pub fn get_l(&self) -> u8 {
    self.l
  }
  pub fn get_reg_u16(&mut self, reg: &Reg) -> u16 {
    match reg {
      Reg::AF => self.get_af(),
      Reg::BC => self.get_bc(),
      Reg::DE => self.get_de(),
      Reg::HL => self.get_hl(),
      _ => panic!("Unsupported fn get_reg_u16"),
    }
  }
}
