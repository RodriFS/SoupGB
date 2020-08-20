use super::emulator::Emulator;
use super::registers::{Flags, Registers};
use super::utils::*;

pub fn add_a_n(data: u8, a: u8, reg: &mut Registers) -> u8 {
  reg.set_flag(Flags::Z, test_flag_add(a, data, Flags::Z));
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, test_flag_add(a, data, Flags::H));
  reg.set_flag(Flags::C, test_flag_add(a, data, Flags::C));
  a.wrapping_add(data)
}

pub fn addc_a_n(data: u8, a: u8, reg: &mut Registers) -> u8 {
  let carry = reg.get_flag(Flags::C);
  reg.set_flag(Flags::Z, test_flag_add_carry(a, data, carry, Flags::Z));
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, test_flag_add_carry(a, data, carry, Flags::H));
  reg.set_flag(Flags::C, test_flag_add_carry(a, data, carry, Flags::C));
  a.wrapping_add(data).wrapping_add(carry)
}

pub fn sub_a_n(data: u8, a: u8, reg: &mut Registers) -> u8 {
  reg.set_flag(Flags::Z, test_flag_sub(a, data, Flags::Z));
  reg.set_flag(Flags::N, true);
  reg.set_flag(Flags::H, test_flag_sub(a, data, Flags::H));
  reg.set_flag(Flags::C, test_flag_sub(a, data, Flags::C));
  a.wrapping_sub(data)
}

pub fn subc_a_n(data: u8, a: u8, reg: &mut Registers) -> u8 {
  let carry = reg.get_flag(Flags::C);
  reg.set_flag(Flags::Z, test_flag_sub_carry(a, data, carry, Flags::Z));
  reg.set_flag(Flags::N, true);
  reg.set_flag(Flags::H, test_flag_sub_carry(a, data, carry, Flags::H));
  reg.set_flag(Flags::C, test_flag_sub_carry(a, data, carry, Flags::C));
  a.wrapping_sub(data).wrapping_sub(carry)
}

pub fn and_n(data: u8, a: u8, reg: &mut Registers) -> u8 {
  let result = data & a;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, true);
  reg.set_flag(Flags::C, false);
  result
}

pub fn or_n(data: u8, a: u8, reg: &mut Registers) -> u8 {
  let result = data | a;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, false);
  result
}

pub fn xor_n(data: u8, a: u8, reg: &mut Registers) -> u8 {
  let result = data ^ a;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, false);
  result
}

pub fn cp_n(data: u8, a: u8, reg: &mut Registers) {
  reg.set_flag(Flags::Z, test_flag_sub(a, data, Flags::Z));
  reg.set_flag(Flags::N, true);
  reg.set_flag(Flags::H, test_flag_sub(a, data, Flags::H));
  reg.set_flag(Flags::C, test_flag_sub(a, data, Flags::C));
}

pub fn inc_n(data: u8, reg: &mut Registers) -> u8 {
  reg.set_flag(Flags::Z, test_flag_add(data, 1, Flags::Z));
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, test_flag_add(data, 1, Flags::H));
  data.wrapping_add(1)
}

pub fn dec_n(data: u8, reg: &mut Registers) -> u8 {
  reg.set_flag(Flags::Z, test_flag_sub(data, 1, Flags::Z));
  reg.set_flag(Flags::N, true);
  reg.set_flag(Flags::H, test_flag_sub(data, 1, Flags::H));
  data.wrapping_sub(1)
}

pub fn add_hl_n(hl: u16, data: u16, reg: &mut Registers) -> u16 {
  let result = hl.wrapping_add(data);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, test_flag_add_16(hl, data, Flags::H));
  reg.set_flag(Flags::C, test_flag_add_16(hl, data, Flags::C));
  result
}

pub fn swap_n(data: u8, reg: &mut Registers) -> u8 {
  let result = swap_nibbles(data);
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::C, false);
  reg.set_flag(Flags::H, false);
  result
}

pub fn jr_cc_n(condition: bool, ctx: &mut Emulator) {
  let address = ctx.get_byte() as i8;
  if condition {
    let result = ctx
      .memory
      .get_program_counter()
      .wrapping_add(address as u16);
    ctx.take_cycle();
    ctx.memory.set_program_counter(result);
  }
}

pub fn ret(ctx: &mut Emulator) {
  let address = ctx.pop_from_stack();
  ctx.take_cycle();
  ctx.memory.set_program_counter(address);
}

pub fn ret_cc(condition: bool, ctx: &mut Emulator) {
  ctx.take_cycle();
  if condition {
    let address = ctx.pop_from_stack();
    ctx.take_cycle();
    ctx.memory.set_program_counter(address);
  }
}

pub fn reti(ctx: &mut Emulator) {
  let address = ctx.pop_from_stack();
  ctx.memory.set_program_counter(address);
  ctx.timers.master_enabled = true; // reti enables IME without delay
  ctx.take_cycle();
}

pub fn jp_cc_nn(condition: bool, ctx: &mut Emulator) {
  let address = ctx.get_word();
  if condition {
    ctx.take_cycle();
    ctx.memory.set_program_counter(address);
  }
}

pub fn call_cc_nn(condition: bool, ctx: &mut Emulator) {
  let address = ctx.get_word();
  if condition {
    ctx.take_cycle();
    let next_pc = ctx.memory.get_program_counter();
    ctx.push_to_stack(next_pc);
    ctx.memory.set_program_counter(address);
  }
}

pub fn rst_n(new_address: u16, ctx: &mut Emulator) {
  ctx.take_cycle();
  let current_address = ctx.memory.get_program_counter();
  ctx.push_to_stack(current_address);
  ctx.memory.set_program_counter(new_address);
}

pub fn rlc_n(data: u8, reg: &mut Registers) -> u8 {
  let to_carry = data >> 7;
  let result = data << 1 | to_carry;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, to_carry == 1);
  result
}

pub fn rl_n(data: u8, reg: &mut Registers) -> u8 {
  let result = reg.get_flag(Flags::C) | (data << 1);
  let to_carry = data >> 7;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, to_carry == 1);
  result
}

pub fn rrc_n(data: u8, reg: &mut Registers) -> u8 {
  let to_carry = data & 0x1;
  let result = to_carry << 7 | data >> 1;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, to_carry == 1);
  result
}

pub fn rr_n(data: u8, reg: &mut Registers) -> u8 {
  let result = reg.get_flag(Flags::C) << 7 | data >> 1;
  let to_carry = data & 0x1;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, to_carry == 1);
  result
}

pub fn sla_n(data: u8, reg: &mut Registers) -> u8 {
  let result = data << 1;
  let to_carry = data >> 7;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, to_carry == 1);
  result
}

pub fn sra_n(data: u8, reg: &mut Registers) -> u8 {
  let result = (data >> 1) | (data & 0x80);
  let to_carry = data & 0x01;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, to_carry == 1);
  result
}

pub fn srl_n(data: u8, reg: &mut Registers) -> u8 {
  let result = data >> 1;
  let to_carry = data & 0x01;
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, to_carry == 1);
  result
}

pub fn bit_b_r(data: u8, bit: u8, reg: &mut Registers) {
  let result = data & (1 << bit);
  reg.set_flag(Flags::Z, result == 0);
  reg.set_flag(Flags::N, false);
  reg.set_flag(Flags::H, true);
}

pub fn set_b_r(data: u8, bit: u8) -> u8 {
  data | (1 << bit)
}

pub fn res_b_r(data: u8, bit: u8) -> u8 {
  data & !(1 << bit)
}

pub fn daa(reg: &mut Registers) {
  let mut carry = false;
  let a = reg.get_a();
  if reg.get_flag(Flags::N) == 0 {
    if reg.get_flag(Flags::C) == 1 || a > 0x99 {
      reg.set_a(a.wrapping_add(0x60));
      carry = true;
    }
    let a = reg.get_a();
    if reg.get_flag(Flags::H) == 1 || (a & 0x0f) > 0x09 {
      reg.set_a(a.wrapping_add(0x06));
    }
  } else if reg.get_flag(Flags::C) == 1 {
    carry = true;
    let h = reg.get_flag(Flags::H);
    reg.set_a(a.wrapping_add(if h == 1 { 0x9a } else { 0xa0 }));
  } else if reg.get_flag(Flags::H) == 1 {
    reg.set_a(a.wrapping_add(0xfa));
  }
  let a = reg.get_a();
  reg.set_flag(Flags::Z, a == 0);
  reg.set_flag(Flags::H, false);
  reg.set_flag(Flags::C, carry);
}
