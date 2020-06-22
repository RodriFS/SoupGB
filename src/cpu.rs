use super::clock::Clock;
use super::constants::*;
use super::memory::Memory;
use super::registers::{Flags, Reg, Registers};
use super::utils::*;

pub struct Cpu<'a> {
    memory: &'a mut Memory,
    clock: &'a mut Clock,
    registers: &'a mut Registers,
}

impl<'a> Cpu<'a> {
    fn new(memory: &'a mut Memory, clock: &'a mut Clock, registers: &'a mut Registers) -> Self {
        Self {
            memory,
            clock,
            registers,
        }
    }

    fn get_hl_address_data(&self) -> u8 {
        let hl = self.registers.get_hl();
        self.memory.read(hl)
    }

    fn write_in_hl_address(&mut self, data: u8) {
        let hl = self.registers.get_hl();
        self.memory.write(hl, data)
    }
    //// INSTRUCTIONS
    fn ld_nn_n(&mut self, reg: Reg) -> bool {
        let next_8 = self.memory.get_next_8();
        let _ = match reg {
            Reg::B => self.registers.set_b(next_8),
            Reg::C => self.registers.set_c(next_8),
            Reg::D => self.registers.set_d(next_8),
            Reg::E => self.registers.set_e(next_8),
            Reg::H => self.registers.set_h(next_8),
            Reg::L => self.registers.set_l(next_8),
            _ => panic!("Unsupported fn ld_nn_n"),
        };
        true
    }
    fn ld_n_nn(&mut self, n: Reg) -> bool {
        let data = self.memory.get_next_16();
        match n {
            Reg::BC => self.registers.set_bc(data),
            Reg::DE => self.registers.set_de(data),
            Reg::HL => self.registers.set_hl(data),
            Reg::SP => self.memory.set_stack_pointer(data),
            _ => panic!("Unsupported fn ld_n_nn"),
        }
        true
    }
    fn ld_r1_r2(&mut self, r1: Reg, r2: u8) -> bool {
        match r1 {
            Reg::A => self.registers.set_a(r2),
            Reg::B => self.registers.set_b(r2),
            Reg::C => self.registers.set_c(r2),
            Reg::D => self.registers.set_d(r2),
            Reg::E => self.registers.set_e(r2),
            Reg::H => self.registers.set_h(r2),
            Reg::L => self.registers.set_l(r2),
            _ => panic!("Unsupported fn ld_r1_r2"),
        };
        true
    }
    fn ld_r1_hl(&mut self, r1: Reg) -> bool {
        let hl = self.registers.get_hl();
        let data = self.memory.read(hl);
        self.ld_r1_r2(r1, data);
        true
    }
    fn ld_hl_r2(&mut self, r2: Reg) -> bool {
        let data = self.registers.get_reg_u8(&r2);
        let hl = self.registers.get_hl();
        self.memory.write(hl, data);
        true
    }
    fn ld_a_n(&mut self, reg: Reg) -> bool {
        let address = match reg {
            Reg::BC => self.registers.get_bc(),
            Reg::DE => self.registers.get_de(),
            Reg::HL => self.registers.get_hl(),
            Reg::N16 => self.memory.get_next_16(),
            _ => panic!("Unsupported fn ld_a_n"),
        };
        let data = self.memory.read(address);
        self.registers.set_a(data);
        true
    }
    fn ld_n_a(&mut self, reg: Reg) -> bool {
        let address = match reg {
            Reg::BC => self.registers.get_bc(),
            Reg::DE => self.registers.get_de(),
            Reg::HL => self.registers.get_hl(),
            Reg::N16 => self.memory.get_next_16(),
            _ => panic!("Unsupported fn ld_n_a"),
        };
        let a = self.registers.get_a();
        self.memory.write(address, a);
        true
    }
    fn push_nn(&mut self, reg: Reg) -> bool {
        let address = match reg {
            Reg::AF => self.registers.get_af(),
            Reg::BC => self.registers.get_bc(),
            Reg::DE => self.registers.get_de(),
            Reg::HL => self.registers.get_hl(),
            _ => panic!("Unsupported fn push_nn"),
        };
        self.memory.push_to_stack(address);
        true
    }
    fn pop_nn(&mut self, reg: Reg) -> bool {
        let data = self.memory.pop_from_stack();
        match reg {
            Reg::AF => self.registers.set_af(data),
            Reg::BC => self.registers.set_bc(data),
            Reg::DE => self.registers.set_de(data),
            Reg::HL => self.registers.set_hl(data),
            _ => panic!("Unsupported fn pop_nn"),
        }
        true
    }
    fn add_a_n(&mut self, data: u8) -> bool {
        let a = self.registers.get_a();
        self.registers
            .set_flag(Flags::Z, test_flag_add(a, data, Flags::Z));
        self.registers.set_flag(Flags::N, false);
        self.registers
            .set_flag(Flags::H, test_flag_add(a, data, Flags::H));
        self.registers
            .set_flag(Flags::C, test_flag_add(a, data, Flags::C));
        self.registers.set_a(a.wrapping_add(data));
        true
    }
    fn addc_a_n(&mut self, data: u8) -> bool {
        let carry = self.registers.get_flag(Flags::C);
        let a = self.registers.get_a();
        self.registers
            .set_flag(Flags::Z, test_flag_add_carry(a, data, carry, Flags::Z));
        self.registers.set_flag(Flags::N, false);
        self.registers
            .set_flag(Flags::H, test_flag_add_carry(a, data, carry, Flags::H));
        self.registers
            .set_flag(Flags::C, test_flag_add_carry(a, data, carry, Flags::C));
        self.registers
            .set_a(a.wrapping_add(data).wrapping_add(carry));
        true
    }
    fn sub_a_n(&mut self, data: u8) -> bool {
        let a = self.registers.get_a();
        self.registers
            .set_flag(Flags::Z, test_flag_sub(a, data, Flags::Z));
        self.registers.set_flag(Flags::N, true);
        self.registers
            .set_flag(Flags::H, test_flag_sub(a, data, Flags::H));
        self.registers
            .set_flag(Flags::C, test_flag_sub(a, data, Flags::C));
        self.registers.set_a(a.wrapping_sub(data));
        true
    }
    fn subc_a_n(&mut self, data: u8) -> bool {
        let carry = self.registers.get_flag(Flags::C);
        let a = self.registers.get_a();
        self.registers
            .set_flag(Flags::Z, test_flag_sub_carry(a, data, carry, Flags::Z));
        self.registers.set_flag(Flags::N, true);
        self.registers
            .set_flag(Flags::H, test_flag_sub_carry(a, data, carry, Flags::H));
        self.registers
            .set_flag(Flags::C, test_flag_sub_carry(a, data, carry, Flags::C));
        self.registers
            .set_a(a.wrapping_sub(data).wrapping_sub(carry));
        true
    }
    fn and_n(&mut self, data: u8) -> bool {
        let result = data & self.registers.get_a();
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, true);
        self.registers.set_flag(Flags::C, false);
        self.registers.set_a(result);
        true
    }
    fn or_n(&mut self, data: u8) -> bool {
        let result = data | self.registers.get_a();
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, false);
        self.registers.set_a(result);
        true
    }
    fn xor_n(&mut self, data: u8) -> bool {
        let result = data ^ self.registers.get_a();
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, false);
        self.registers.set_a(result);
        true
    }
    fn cp_n(&mut self, data: u8) -> bool {
        let a = self.registers.get_a();
        self.registers
            .set_flag(Flags::Z, test_flag_sub(a, data, Flags::Z));
        self.registers.set_flag(Flags::N, true);
        self.registers
            .set_flag(Flags::H, test_flag_sub(a, data, Flags::H));
        self.registers
            .set_flag(Flags::C, test_flag_sub(a, data, Flags::C));
        true
    }
    fn inc_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        self.registers
            .set_flag(Flags::Z, test_flag_add(data, 1, Flags::Z));
        self.registers.set_flag(Flags::N, false);
        self.registers
            .set_flag(Flags::H, test_flag_add(data, 1, Flags::H));
        self.registers.set_reg_u8(&reg, data.wrapping_add(1));
        true
    }
    fn dec_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        self.registers
            .set_flag(Flags::Z, test_flag_sub(data, 1, Flags::Z));
        self.registers.set_flag(Flags::N, true);
        self.registers
            .set_flag(Flags::H, test_flag_sub(data, 1, Flags::H));
        self.registers.set_reg_u8(&reg, data.wrapping_sub(1));
        true
    }
    fn inc_nn(&mut self, reg: Reg) -> bool {
        let address = self.registers.get_reg_u16(&reg);
        self.registers.set_reg_u16(&reg, address.wrapping_add(1));
        true
    }
    fn dec_nn(&mut self, reg: Reg) -> bool {
        let address = self.registers.get_reg_u16(&reg);
        self.registers.set_reg_u16(&reg, address.wrapping_sub(1));
        true
    }
    fn add_hl_n(&mut self, reg: Reg) -> bool {
        let hl = self.registers.get_hl();
        let data = match reg {
            Reg::BC => self.registers.get_bc(),
            Reg::DE => self.registers.get_de(),
            Reg::HL => hl,
            Reg::SP => self.memory.get_stack_pointer(),
            _ => panic!("Unsupported fn add_hl_n"),
        };
        let result = hl.wrapping_add(data);
        self.registers.set_flag(Flags::N, false);
        self.registers
            .set_flag(Flags::H, test_flag_add_16(hl, data, Flags::H));
        self.registers
            .set_flag(Flags::C, test_flag_add_16(hl, data, Flags::C));
        self.registers.set_hl(result);
        true
    }
    fn swap_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let result = swap_nibbles(data);
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::C, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn jr_cc_n(&mut self, condition: bool) -> bool {
        let address = self.memory.get_next_8() as i8;
        if condition {
            self.memory.set_program_counter(
                self.memory
                    .get_program_counter()
                    .wrapping_add(address as u16),
            );
            return true;
        }
        false
    }
    fn ret_cc(&mut self, condition: bool) -> bool {
        if condition {
            let address = self.memory.pop_from_stack();
            self.memory.set_program_counter(address);
            return true;
        }
        false
    }
    fn jp_cc_nn(&mut self, condition: bool) -> bool {
        let address = self.memory.get_next_16();
        if condition {
            self.memory.set_program_counter(address);
            return true;
        }
        false
    }
    fn call_cc_nn(&mut self, condition: bool) -> bool {
        let address = self.memory.get_next_16();
        if condition {
            let next_pc = self.memory.get_program_counter();
            self.memory.push_to_stack(next_pc);
            self.memory.set_program_counter(address);
            return true;
        }
        false
    }
    fn rst_n(&mut self, new_address: u16) -> bool {
        let current_address = self.memory.get_program_counter();
        self.memory.push_to_stack(current_address);
        self.memory.set_program_counter(new_address);
        true
    }
    fn di(&mut self) -> bool {
        self.clock.clear_master_enabled();
        true
    }
    fn ei(&mut self) -> bool {
        self.clock.set_master_enabled_on();
        true
    }
    fn cb(&mut self) -> u32 {
        let address = self.memory.get_next_8();
        self.execute_opcode(address, true)
    }
    fn rlc_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let to_carry = data >> 7;
        let result = data << 1 | to_carry;
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, to_carry == 1);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn rl_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let result = self.registers.get_flag(Flags::C) | (data << 1);
        let to_carry = data >> 7;
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, to_carry == 1);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn rrc_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let to_carry = data & 0x1;
        let result = to_carry << 7 | data >> 1;
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, to_carry == 1);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn rr_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let result = self.registers.get_flag(Flags::C) << 7 | data >> 1;
        let to_carry = data & 0x1;
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, to_carry == 1);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn sla_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let result = data << 1;
        let to_carry = data >> 7;
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, to_carry == 1);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn sra_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let result = (data >> 1) | (data & 0x80);
        let to_carry = data & 0x01;
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, to_carry == 1);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn srl_n(&mut self, reg: Reg) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        let result = data >> 1;
        let to_carry = data & 0x01;
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, to_carry == 1);
        self.registers.set_reg_u8(&reg, result);
        true
    }
    fn bit_b_r(&mut self, data: u8, bit: u8) -> bool {
        let result = data & (1 << bit);
        self.registers.set_flag(Flags::Z, result == 0);
        self.registers.set_flag(Flags::N, false);
        self.registers.set_flag(Flags::H, true);
        true
    }
    fn set_b_r(&mut self, reg: Reg, bit: u8) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        self.registers.set_reg_u8(&reg, data | (1 << bit));
        true
    }
    fn res_b_r(&mut self, reg: Reg, bit: u8) -> bool {
        let data = self.registers.get_reg_u8(&reg);
        self.registers.set_reg_u8(&reg, data & !(1 << bit));
        true
    }
    fn daa(&mut self) -> bool {
        let mut carry = false;
        let a = self.registers.get_a();
        if self.registers.get_flag(Flags::N) == 0 {
            if self.registers.get_flag(Flags::C) == 1 || a > 0x99 {
                self.registers.set_a(a.wrapping_add(0x60));
                carry = true;
            }
            let a = self.registers.get_a();
            if self.registers.get_flag(Flags::H) == 1 || (a & 0x0f) > 0x09 {
                self.registers.set_a(a.wrapping_add(0x06));
            }
        } else if self.registers.get_flag(Flags::C) == 1 {
            carry = true;
            let h = self.registers.get_flag(Flags::H);
            self.registers
                .set_a(a.wrapping_add(if h == 1 { 0x9a } else { 0xa0 }));
        } else if self.registers.get_flag(Flags::H) == 1 {
            self.registers.set_a(a.wrapping_add(0xfa));
        }
        let a = self.registers.get_a();
        self.registers.set_flag(Flags::Z, a == 0);
        self.registers.set_flag(Flags::H, false);
        self.registers.set_flag(Flags::C, carry);
        true
    }
    fn execute_opcode(&mut self, opcode: u8, is_callback: bool) -> u32 {
        if is_callback {
            match opcode {
                0x00 => self.rlc_n(Reg::B),
                0x01 => self.rlc_n(Reg::C),
                0x02 => self.rlc_n(Reg::D),
                0x03 => self.rlc_n(Reg::E),
                0x04 => self.rlc_n(Reg::H),
                0x05 => self.rlc_n(Reg::L),
                0x06 => {
                    let data = self.get_hl_address_data();
                    let to_carry = data >> 7;
                    let result = data << 1 | to_carry;
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::H, false);
                    self.registers.set_flag(Flags::C, to_carry == 1);
                    self.write_in_hl_address(result);
                    true
                }
                0x07 => self.rlc_n(Reg::A),
                0x08 => self.rrc_n(Reg::B),
                0x09 => self.rrc_n(Reg::C),
                0x0a => self.rrc_n(Reg::D),
                0x0b => self.rrc_n(Reg::E),
                0x0c => self.rrc_n(Reg::H),
                0x0d => self.rrc_n(Reg::L),
                0x0e => {
                    let data = self.get_hl_address_data();
                    let to_carry = data & 0x1;
                    let result = to_carry << 7 | data >> 1;
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::H, false);
                    self.registers.set_flag(Flags::C, to_carry == 1);
                    self.write_in_hl_address(result);
                    true
                }
                0x0f => self.rrc_n(Reg::A),
                0x10 => self.rl_n(Reg::B),
                0x11 => self.rl_n(Reg::C),
                0x12 => self.rl_n(Reg::D),
                0x13 => self.rl_n(Reg::E),
                0x14 => self.rl_n(Reg::H),
                0x15 => self.rl_n(Reg::L),
                0x16 => {
                    let data = self.get_hl_address_data();
                    let result = self.registers.get_flag(Flags::C) | (data << 1);
                    let to_carry = data >> 7;
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::H, false);
                    self.registers.set_flag(Flags::C, to_carry == 1);
                    self.write_in_hl_address(result);
                    true
                }
                0x17 => self.rl_n(Reg::A),
                0x18 => self.rr_n(Reg::B),
                0x19 => self.rr_n(Reg::C),
                0x1a => self.rr_n(Reg::D),
                0x1b => self.rr_n(Reg::E),
                0x1c => self.rr_n(Reg::H),
                0x1d => self.rr_n(Reg::L),
                0x1e => {
                    let data = self.get_hl_address_data();
                    let result = self.registers.get_flag(Flags::C) << 7 | data >> 1;
                    let to_carry = data & 0x1;
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::H, false);
                    self.registers.set_flag(Flags::C, to_carry == 1);
                    self.write_in_hl_address(result);
                    true
                }
                0x1f => self.rr_n(Reg::A),
                0x20 => self.sla_n(Reg::B),
                0x21 => self.sla_n(Reg::C),
                0x22 => self.sla_n(Reg::D),
                0x23 => self.sla_n(Reg::E),
                0x24 => self.sla_n(Reg::H),
                0x25 => self.sla_n(Reg::L),
                0x26 => {
                    let data = self.get_hl_address_data();
                    let result = data << 1;
                    let to_carry = data >> 7;
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::H, false);
                    self.registers.set_flag(Flags::C, to_carry == 1);
                    self.write_in_hl_address(result);
                    true
                }
                0x27 => self.sla_n(Reg::A),
                0x28 => self.sra_n(Reg::B),
                0x29 => self.sra_n(Reg::C),
                0x2a => self.sra_n(Reg::D),
                0x2b => self.sra_n(Reg::E),
                0x2c => self.sra_n(Reg::H),
                0x2d => self.sra_n(Reg::L),
                0x2e => {
                    let data = self.get_hl_address_data();
                    let result = (data >> 1) | (data & 0x80);
                    let to_carry = data & 0x01;
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::H, false);
                    self.registers.set_flag(Flags::C, to_carry == 1);
                    self.write_in_hl_address(result);
                    true
                }
                0x2f => self.sra_n(Reg::A),
                0x30 => self.swap_n(Reg::B),
                0x31 => self.swap_n(Reg::C),
                0x32 => self.swap_n(Reg::D),
                0x33 => self.swap_n(Reg::E),
                0x34 => self.swap_n(Reg::H),
                0x35 => self.swap_n(Reg::L),
                0x36 => {
                    let data = self.get_hl_address_data();
                    let result = swap_nibbles(data);
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::C, false);
                    self.registers.set_flag(Flags::H, false);
                    self.write_in_hl_address(result);
                    true
                }
                0x37 => self.swap_n(Reg::A),
                0x38 => self.srl_n(Reg::B),
                0x39 => self.srl_n(Reg::C),
                0x3a => self.srl_n(Reg::D),
                0x3b => self.srl_n(Reg::E),
                0x3c => self.srl_n(Reg::H),
                0x3d => self.srl_n(Reg::L),
                0x3e => {
                    let data = self.get_hl_address_data();
                    let result = data >> 1;
                    let to_carry = data & 0x01;
                    self.registers.set_flag(Flags::Z, result == 0);
                    self.registers.set_flag(Flags::N, false);
                    self.registers.set_flag(Flags::H, false);
                    self.registers.set_flag(Flags::C, to_carry == 1);
                    self.write_in_hl_address(result);
                    true
                }
                0x3f => self.srl_n(Reg::A),
                0x40 => self.bit_b_r(self.registers.b, 0),
                0x41 => self.bit_b_r(self.registers.c, 0),
                0x42 => self.bit_b_r(self.registers.d, 0),
                0x43 => self.bit_b_r(self.registers.e, 0),
                0x44 => self.bit_b_r(self.registers.h, 0),
                0x45 => self.bit_b_r(self.registers.l, 0),
                0x46 => self.bit_b_r(self.get_hl_address_data(), 0),
                0x47 => self.bit_b_r(self.registers.a, 0),
                0x48 => self.bit_b_r(self.registers.b, 1),
                0x49 => self.bit_b_r(self.registers.c, 1),
                0x4a => self.bit_b_r(self.registers.d, 1),
                0x4b => self.bit_b_r(self.registers.e, 1),
                0x4c => self.bit_b_r(self.registers.h, 1),
                0x4d => self.bit_b_r(self.registers.l, 1),
                0x4e => self.bit_b_r(self.get_hl_address_data(), 1),
                0x4f => self.bit_b_r(self.registers.a, 1),
                0x50 => self.bit_b_r(self.registers.b, 2),
                0x51 => self.bit_b_r(self.registers.c, 2),
                0x52 => self.bit_b_r(self.registers.d, 2),
                0x53 => self.bit_b_r(self.registers.e, 2),
                0x54 => self.bit_b_r(self.registers.h, 2),
                0x55 => self.bit_b_r(self.registers.l, 2),
                0x56 => self.bit_b_r(self.get_hl_address_data(), 2),
                0x57 => self.bit_b_r(self.registers.a, 2),
                0x58 => self.bit_b_r(self.registers.b, 3),
                0x59 => self.bit_b_r(self.registers.c, 3),
                0x5a => self.bit_b_r(self.registers.d, 3),
                0x5b => self.bit_b_r(self.registers.e, 3),
                0x5c => self.bit_b_r(self.registers.h, 3),
                0x5d => self.bit_b_r(self.registers.l, 3),
                0x5e => self.bit_b_r(self.get_hl_address_data(), 3),
                0x5f => self.bit_b_r(self.registers.a, 3),
                0x60 => self.bit_b_r(self.registers.b, 4),
                0x61 => self.bit_b_r(self.registers.c, 4),
                0x62 => self.bit_b_r(self.registers.d, 4),
                0x63 => self.bit_b_r(self.registers.e, 4),
                0x64 => self.bit_b_r(self.registers.h, 4),
                0x65 => self.bit_b_r(self.registers.l, 4),
                0x66 => self.bit_b_r(self.get_hl_address_data(), 4),
                0x67 => self.bit_b_r(self.registers.a, 4),
                0x68 => self.bit_b_r(self.registers.b, 5),
                0x69 => self.bit_b_r(self.registers.c, 5),
                0x6a => self.bit_b_r(self.registers.d, 5),
                0x6b => self.bit_b_r(self.registers.e, 5),
                0x6c => self.bit_b_r(self.registers.h, 5),
                0x6d => self.bit_b_r(self.registers.l, 5),
                0x6e => self.bit_b_r(self.get_hl_address_data(), 5),
                0x6f => self.bit_b_r(self.registers.a, 5),
                0x70 => self.bit_b_r(self.registers.b, 6),
                0x71 => self.bit_b_r(self.registers.c, 6),
                0x72 => self.bit_b_r(self.registers.d, 6),
                0x73 => self.bit_b_r(self.registers.e, 6),
                0x74 => self.bit_b_r(self.registers.h, 6),
                0x75 => self.bit_b_r(self.registers.l, 6),
                0x76 => self.bit_b_r(self.get_hl_address_data(), 6),
                0x77 => self.bit_b_r(self.registers.a, 6),
                0x78 => self.bit_b_r(self.registers.b, 7),
                0x79 => self.bit_b_r(self.registers.c, 7),
                0x7a => self.bit_b_r(self.registers.d, 7),
                0x7b => self.bit_b_r(self.registers.e, 7),
                0x7c => self.bit_b_r(self.registers.h, 7),
                0x7d => self.bit_b_r(self.registers.l, 7),
                0x7e => self.bit_b_r(self.get_hl_address_data(), 7),
                0x7f => self.bit_b_r(self.registers.a, 7),
                0x80 => self.res_b_r(Reg::B, 0),
                0x81 => self.res_b_r(Reg::C, 0),
                0x82 => self.res_b_r(Reg::D, 0),
                0x83 => self.res_b_r(Reg::E, 0),
                0x84 => self.res_b_r(Reg::H, 0),
                0x85 => self.res_b_r(Reg::L, 0),
                0x86 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !1);
                    true
                }
                0x87 => self.res_b_r(Reg::A, 0),
                0x88 => self.res_b_r(Reg::B, 1),
                0x89 => self.res_b_r(Reg::C, 1),
                0x8a => self.res_b_r(Reg::D, 1),
                0x8b => self.res_b_r(Reg::E, 1),
                0x8c => self.res_b_r(Reg::H, 1),
                0x8d => self.res_b_r(Reg::L, 1),
                0x8e => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !(1 << 1));
                    true
                }
                0x8f => self.res_b_r(Reg::A, 1),
                0x90 => self.res_b_r(Reg::B, 2),
                0x91 => self.res_b_r(Reg::C, 2),
                0x92 => self.res_b_r(Reg::D, 2),
                0x93 => self.res_b_r(Reg::E, 2),
                0x94 => self.res_b_r(Reg::H, 2),
                0x95 => self.res_b_r(Reg::L, 2),
                0x96 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !(1 << 2));
                    true
                }
                0x97 => self.res_b_r(Reg::A, 2),
                0x98 => self.res_b_r(Reg::B, 3),
                0x99 => self.res_b_r(Reg::C, 3),
                0x9a => self.res_b_r(Reg::D, 3),
                0x9b => self.res_b_r(Reg::E, 3),
                0x9c => self.res_b_r(Reg::H, 3),
                0x9d => self.res_b_r(Reg::L, 3),
                0x9e => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !(1 << 3));
                    true
                }
                0x9f => self.res_b_r(Reg::A, 3),
                0xa0 => self.res_b_r(Reg::B, 4),
                0xa1 => self.res_b_r(Reg::C, 4),
                0xa2 => self.res_b_r(Reg::D, 4),
                0xa3 => self.res_b_r(Reg::E, 4),
                0xa4 => self.res_b_r(Reg::H, 4),
                0xa5 => self.res_b_r(Reg::L, 4),
                0xa6 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !(1 << 4));
                    true
                }
                0xa7 => self.res_b_r(Reg::A, 4),
                0xa8 => self.res_b_r(Reg::B, 5),
                0xa9 => self.res_b_r(Reg::C, 5),
                0xaa => self.res_b_r(Reg::D, 5),
                0xab => self.res_b_r(Reg::E, 5),
                0xac => self.res_b_r(Reg::H, 5),
                0xad => self.res_b_r(Reg::L, 5),
                0xae => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !(1 << 5));
                    true
                }
                0xaf => self.res_b_r(Reg::A, 5),
                0xb0 => self.res_b_r(Reg::B, 6),
                0xb1 => self.res_b_r(Reg::C, 6),
                0xb2 => self.res_b_r(Reg::D, 6),
                0xb3 => self.res_b_r(Reg::E, 6),
                0xb4 => self.res_b_r(Reg::H, 6),
                0xb5 => self.res_b_r(Reg::L, 6),
                0xb6 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !(1 << 6));
                    true
                }
                0xb7 => self.res_b_r(Reg::A, 6),
                0xb8 => self.res_b_r(Reg::B, 7),
                0xb9 => self.res_b_r(Reg::C, 7),
                0xba => self.res_b_r(Reg::D, 7),
                0xbb => self.res_b_r(Reg::E, 7),
                0xbc => self.res_b_r(Reg::H, 7),
                0xbd => self.res_b_r(Reg::L, 7),
                0xbe => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data & !(1 << 7));
                    true
                }
                0xbf => self.res_b_r(Reg::A, 7),
                0xc0 => self.set_b_r(Reg::B, 0),
                0xc1 => self.set_b_r(Reg::C, 0),
                0xc2 => self.set_b_r(Reg::D, 0),
                0xc3 => self.set_b_r(Reg::E, 0),
                0xc4 => self.set_b_r(Reg::H, 0),
                0xc5 => self.set_b_r(Reg::L, 0),
                0xc6 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | 1);
                    true
                }
                0xc7 => self.set_b_r(Reg::A, 0),
                0xc8 => self.set_b_r(Reg::B, 1),
                0xc9 => self.set_b_r(Reg::C, 1),
                0xca => self.set_b_r(Reg::D, 1),
                0xcb => self.set_b_r(Reg::E, 1),
                0xcc => self.set_b_r(Reg::H, 1),
                0xcd => self.set_b_r(Reg::L, 1),
                0xce => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | (1 << 1));
                    true
                }
                0xcf => self.set_b_r(Reg::A, 1),
                0xd0 => self.set_b_r(Reg::B, 2),
                0xd1 => self.set_b_r(Reg::C, 2),
                0xd2 => self.set_b_r(Reg::D, 2),
                0xd3 => self.set_b_r(Reg::E, 2),
                0xd4 => self.set_b_r(Reg::H, 2),
                0xd5 => self.set_b_r(Reg::L, 2),
                0xd6 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | (1 << 2));
                    true
                }
                0xd7 => self.set_b_r(Reg::A, 2),
                0xd8 => self.set_b_r(Reg::B, 3),
                0xd9 => self.set_b_r(Reg::C, 3),
                0xda => self.set_b_r(Reg::D, 3),
                0xdb => self.set_b_r(Reg::E, 3),
                0xdc => self.set_b_r(Reg::H, 3),
                0xdd => self.set_b_r(Reg::L, 3),
                0xde => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | (1 << 3));
                    true
                }
                0xdf => self.set_b_r(Reg::A, 3),
                0xe0 => self.set_b_r(Reg::B, 4),
                0xe1 => self.set_b_r(Reg::C, 4),
                0xe2 => self.set_b_r(Reg::D, 4),
                0xe3 => self.set_b_r(Reg::E, 4),
                0xe4 => self.set_b_r(Reg::H, 4),
                0xe5 => self.set_b_r(Reg::L, 4),
                0xe6 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | (1 << 4));
                    true
                }
                0xe7 => self.set_b_r(Reg::A, 4),
                0xe8 => self.set_b_r(Reg::B, 5),
                0xe9 => self.set_b_r(Reg::C, 5),
                0xea => self.set_b_r(Reg::D, 5),
                0xeb => self.set_b_r(Reg::E, 5),
                0xec => self.set_b_r(Reg::H, 5),
                0xed => self.set_b_r(Reg::L, 5),
                0xee => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | (1 << 5));
                    true
                }
                0xef => self.set_b_r(Reg::A, 5),
                0xf0 => self.set_b_r(Reg::B, 6),
                0xf1 => self.set_b_r(Reg::C, 6),
                0xf2 => self.set_b_r(Reg::D, 6),
                0xf3 => self.set_b_r(Reg::E, 6),
                0xf4 => self.set_b_r(Reg::H, 6),
                0xf5 => self.set_b_r(Reg::L, 6),
                0xf6 => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | (1 << 6));
                    true
                }
                0xf7 => self.set_b_r(Reg::A, 6),
                0xf8 => self.set_b_r(Reg::B, 7),
                0xf9 => self.set_b_r(Reg::C, 7),
                0xfa => self.set_b_r(Reg::D, 7),
                0xfb => self.set_b_r(Reg::E, 7),
                0xfc => self.set_b_r(Reg::H, 7),
                0xfd => self.set_b_r(Reg::L, 7),
                0xfe => {
                    let data = self.get_hl_address_data();
                    self.write_in_hl_address(data | (1 << 7));
                    true
                }
                0xff => self.set_b_r(Reg::A, 7),
            };
            return CB_OP_TIMES[opcode as usize] as u32;
        }
        let mut cb_timing = 0;
        let taken = match opcode {
            0x00 => true,
            0x01 => self.ld_n_nn(Reg::BC),
            0x02 => self.ld_n_a(Reg::BC),
            0x03 => self.inc_nn(Reg::BC),
            0x04 => self.inc_n(Reg::B),
            0x05 => self.dec_n(Reg::B),
            0x06 => self.ld_nn_n(Reg::B),
            0x07 => {
                let r = self.rlc_n(Reg::A);
                self.registers.set_flag(Flags::Z, false);
                r
            }
            0x08 => {
                let address = self.memory.get_next_16();
                let stack_pointer = self.memory.get_stack_pointer();
                self.memory.write_u16(address, stack_pointer);
                true
            }
            0x09 => self.add_hl_n(Reg::BC),
            0x0a => self.ld_a_n(Reg::BC),
            0x0b => self.dec_nn(Reg::BC),
            0x0c => self.inc_n(Reg::C),
            0x0d => self.dec_n(Reg::C),
            0x0e => self.ld_nn_n(Reg::C),
            0x0f => {
                let r = self.rrc_n(Reg::A);
                self.registers.set_flag(Flags::Z, false);
                r
            }
            0x10 => true,
            0x11 => self.ld_n_nn(Reg::DE),
            0x12 => self.ld_n_a(Reg::DE),
            0x13 => self.inc_nn(Reg::DE),
            0x14 => self.inc_n(Reg::D),
            0x15 => self.dec_n(Reg::D),
            0x16 => self.ld_nn_n(Reg::D),
            0x17 => {
                let r = self.rl_n(Reg::A);
                self.registers.set_flag(Flags::Z, false);
                r
            }
            0x18 => self.jr_cc_n(true),
            0x19 => self.add_hl_n(Reg::DE),
            0x1a => self.ld_a_n(Reg::DE),
            0x1b => self.dec_nn(Reg::DE),
            0x1c => self.inc_n(Reg::E),
            0x1d => self.dec_n(Reg::E),
            0x1e => self.ld_nn_n(Reg::E),
            0x1f => {
                let r = self.rr_n(Reg::A);
                self.registers.set_flag(Flags::Z, false);
                r
            }
            0x20 => {
                let z = self.registers.get_flag(Flags::Z);
                self.jr_cc_n(z == 0)
            }
            0x21 => self.ld_n_nn(Reg::HL),
            0x22 => {
                let address = self.registers.get_hl();
                let a = self.registers.get_a();
                self.memory.write(address, a);
                self.registers.set_hl(address.wrapping_add(1));
                true
            }
            0x23 => self.inc_nn(Reg::HL),
            0x24 => self.inc_n(Reg::H),
            0x25 => self.dec_n(Reg::H),
            0x26 => self.ld_nn_n(Reg::H),
            0x27 => self.daa(),
            0x28 => {
                let z = self.registers.get_flag(Flags::Z);
                self.jr_cc_n(z == 1)
            }
            0x29 => self.add_hl_n(Reg::HL),
            0x2a => {
                let address = self.registers.get_hl();
                let data = self.memory.read(address);
                self.registers.set_a(data);
                self.registers.set_hl(address.wrapping_add(1));
                true
            }
            0x2b => self.dec_nn(Reg::HL),
            0x2c => self.inc_n(Reg::L),
            0x2d => self.dec_n(Reg::L),
            0x2e => self.ld_nn_n(Reg::L),
            0x2f => {
                let a = self.registers.get_a();
                self.registers.set_a(!a);
                self.registers.set_flag(Flags::H, true);
                self.registers.set_flag(Flags::N, true);
                true
            }
            0x30 => {
                let c = self.registers.get_flag(Flags::C);
                self.jr_cc_n(c == 0)
            }
            0x31 => self.ld_n_nn(Reg::SP),
            0x32 => {
                let address = self.registers.get_hl();
                let a = self.registers.get_a();
                self.memory.write(address, a);
                self.registers.set_hl(address.wrapping_sub(1));
                true
            }
            0x33 => {
                self.memory.increment_stack_pointer(1);
                true
            }
            0x34 => {
                let data = self.get_hl_address_data();
                self.registers
                    .set_flag(Flags::Z, test_flag_add(data, 1, Flags::Z));
                self.registers.set_flag(Flags::N, false);
                self.registers
                    .set_flag(Flags::H, test_flag_add(data, 1, Flags::H));
                self.write_in_hl_address(data.wrapping_add(1));
                true
            }
            0x35 => {
                let data = self.get_hl_address_data();
                self.registers
                    .set_flag(Flags::Z, test_flag_sub(data, 1, Flags::Z));
                self.registers.set_flag(Flags::N, true);
                self.registers
                    .set_flag(Flags::H, test_flag_sub(data, 1, Flags::H));
                self.write_in_hl_address(data.wrapping_sub(1));
                true
            }
            0x36 => {
                let data = self.memory.get_next_8();
                let hl = self.registers.get_hl();
                self.memory.write(hl, data);
                true
            }
            0x37 => {
                self.registers.set_flag(Flags::C, true);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
                true
            }
            0x38 => {
                let c = self.registers.get_flag(Flags::C);
                self.jr_cc_n(c == 1)
            }
            0x39 => self.add_hl_n(Reg::SP),
            0x3a => {
                let address = self.registers.get_hl();
                let data = self.memory.read(address);
                self.registers.set_a(data);
                self.registers.set_hl(address.wrapping_sub(1));
                true
            }
            0x3b => {
                self.memory.decrement_stack_pointer(1);
                true
            }
            0x3c => self.inc_n(Reg::A),
            0x3d => self.dec_n(Reg::A),
            0x3e => {
                let n = self.memory.get_next_8();
                self.ld_r1_r2(Reg::A, n)
            }
            0x3f => {
                let c = self.registers.get_flag(Flags::C);
                self.registers.set_flag(Flags::C, c == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
                true
            }
            0x40 => true,
            0x41 => {
                let c = self.registers.get_c();
                self.ld_r1_r2(Reg::B, c)
            }
            0x42 => {
                let d = self.registers.get_d();
                self.ld_r1_r2(Reg::B, d)
            }
            0x43 => {
                let e = self.registers.get_e();
                self.ld_r1_r2(Reg::B, e)
            }
            0x44 => {
                let h = self.registers.get_h();
                self.ld_r1_r2(Reg::B, h)
            }
            0x45 => {
                let l = self.registers.get_l();
                self.ld_r1_r2(Reg::B, l)
            }
            0x46 => self.ld_r1_hl(Reg::B),
            0x47 => {
                let a = self.registers.get_a();
                self.ld_r1_r2(Reg::B, a)
            }
            0x48 => {
                let b = self.registers.get_b();
                self.ld_r1_r2(Reg::C, b)
            }
            0x49 => true,
            0x4a => {
                let d = self.registers.get_d();
                self.ld_r1_r2(Reg::C, d)
            }
            0x4b => {
                let e = self.registers.get_e();
                self.ld_r1_r2(Reg::C, e)
            }
            0x4c => {
                let h = self.registers.get_h();
                self.ld_r1_r2(Reg::C, h)
            }
            0x4d => {
                let l = self.registers.get_l();
                self.ld_r1_r2(Reg::C, l)
            }
            0x4e => self.ld_r1_hl(Reg::C),
            0x4f => {
                let a = self.registers.get_a();
                self.ld_r1_r2(Reg::C, a)
            }
            0x50 => {
                let b = self.registers.get_b();
                self.ld_r1_r2(Reg::D, b)
            }
            0x51 => {
                let c = self.registers.get_c();
                self.ld_r1_r2(Reg::D, c)
            }
            0x52 => true,
            0x53 => {
                let e = self.registers.get_e();
                self.ld_r1_r2(Reg::D, e)
            }
            0x54 => {
                let h = self.registers.get_h();
                self.ld_r1_r2(Reg::D, h)
            }
            0x55 => {
                let l = self.registers.get_l();
                self.ld_r1_r2(Reg::D, l)
            }
            0x56 => self.ld_r1_hl(Reg::D),
            0x57 => {
                let a = self.registers.get_a();
                self.ld_r1_r2(Reg::D, a)
            }
            0x58 => {
                let b = self.registers.get_b();
                self.ld_r1_r2(Reg::E, b)
            }
            0x59 => {
                let c = self.registers.get_c();
                self.ld_r1_r2(Reg::E, c)
            }
            0x5a => {
                let d = self.registers.get_d();
                self.ld_r1_r2(Reg::E, d)
            }
            0x5b => true,
            0x5c => {
                let h = self.registers.get_h();
                self.ld_r1_r2(Reg::E, h)
            }
            0x5d => {
                let l = self.registers.get_l();
                self.ld_r1_r2(Reg::E, l)
            }
            0x5e => self.ld_r1_hl(Reg::E),
            0x5f => {
                let a = self.registers.get_a();
                self.ld_r1_r2(Reg::E, a)
            }
            0x60 => {
                let b = self.registers.get_b();
                self.ld_r1_r2(Reg::H, b)
            }
            0x61 => {
                let c = self.registers.get_c();
                self.ld_r1_r2(Reg::H, c)
            }
            0x62 => {
                let d = self.registers.get_d();
                self.ld_r1_r2(Reg::H, d)
            }
            0x63 => {
                let e = self.registers.get_e();
                self.ld_r1_r2(Reg::H, e)
            }
            0x64 => true,
            0x65 => {
                let l = self.registers.get_l();
                self.ld_r1_r2(Reg::H, l)
            }
            0x66 => self.ld_r1_hl(Reg::H),
            0x67 => {
                let a = self.registers.get_a();
                self.ld_r1_r2(Reg::H, a)
            }
            0x68 => {
                let b = self.registers.get_b();
                self.ld_r1_r2(Reg::L, b)
            }
            0x69 => {
                let c = self.registers.get_c();
                self.ld_r1_r2(Reg::L, c)
            }
            0x6a => {
                let d = self.registers.get_d();
                self.ld_r1_r2(Reg::L, d)
            }
            0x6b => {
                let e = self.registers.get_e();
                self.ld_r1_r2(Reg::L, e)
            }
            0x6c => {
                let h = self.registers.get_h();
                self.ld_r1_r2(Reg::L, h)
            }
            0x6d => true,
            0x6e => self.ld_r1_hl(Reg::L),
            0x6f => {
                let a = self.registers.get_a();
                self.ld_r1_r2(Reg::L, a)
            }
            0x70 => self.ld_hl_r2(Reg::B),
            0x71 => self.ld_hl_r2(Reg::C),
            0x72 => self.ld_hl_r2(Reg::D),
            0x73 => self.ld_hl_r2(Reg::E),
            0x74 => self.ld_hl_r2(Reg::H),
            0x75 => self.ld_hl_r2(Reg::L),
            0x76 => {
                self.clock.is_halted = true;
                true
            }
            0x77 => self.ld_hl_r2(Reg::A),
            0x78 => {
                let b = self.registers.get_b();
                self.ld_r1_r2(Reg::A, b)
            }
            0x79 => {
                let c = self.registers.get_c();
                self.ld_r1_r2(Reg::A, c)
            }
            0x7a => {
                let d = self.registers.get_d();
                self.ld_r1_r2(Reg::A, d)
            }
            0x7b => {
                let e = self.registers.get_e();
                self.ld_r1_r2(Reg::A, e)
            }
            0x7c => {
                let h = self.registers.get_h();
                self.ld_r1_r2(Reg::A, h)
            }
            0x7d => {
                let l = self.registers.get_l();
                self.ld_r1_r2(Reg::A, l)
            }
            0x7e => self.ld_r1_hl(Reg::A),
            0x7f => true,
            0x80 => self.add_a_n(self.registers.b),
            0x81 => self.add_a_n(self.registers.c),
            0x82 => self.add_a_n(self.registers.d),
            0x83 => self.add_a_n(self.registers.e),
            0x84 => self.add_a_n(self.registers.h),
            0x85 => self.add_a_n(self.registers.l),
            0x86 => self.add_a_n(self.get_hl_address_data()),
            0x87 => self.add_a_n(self.registers.a),
            0x88 => self.addc_a_n(self.registers.b),
            0x89 => self.addc_a_n(self.registers.c),
            0x8a => self.addc_a_n(self.registers.d),
            0x8b => self.addc_a_n(self.registers.e),
            0x8c => self.addc_a_n(self.registers.h),
            0x8d => self.addc_a_n(self.registers.l),
            0x8e => self.addc_a_n(self.get_hl_address_data()),
            0x8f => self.addc_a_n(self.registers.a),
            0x90 => self.sub_a_n(self.registers.b),
            0x91 => self.sub_a_n(self.registers.c),
            0x92 => self.sub_a_n(self.registers.d),
            0x93 => self.sub_a_n(self.registers.e),
            0x94 => self.sub_a_n(self.registers.h),
            0x95 => self.sub_a_n(self.registers.l),
            0x96 => self.sub_a_n(self.get_hl_address_data()),
            0x97 => self.sub_a_n(self.registers.a),
            0x98 => self.subc_a_n(self.registers.b),
            0x99 => self.subc_a_n(self.registers.c),
            0x9a => self.subc_a_n(self.registers.d),
            0x9b => self.subc_a_n(self.registers.e),
            0x9c => self.subc_a_n(self.registers.h),
            0x9d => self.subc_a_n(self.registers.l),
            0x9e => self.subc_a_n(self.get_hl_address_data()),
            0x9f => self.subc_a_n(self.registers.a),
            0xa0 => self.and_n(self.registers.b),
            0xa1 => self.and_n(self.registers.c),
            0xa2 => self.and_n(self.registers.d),
            0xa3 => self.and_n(self.registers.e),
            0xa4 => self.and_n(self.registers.h),
            0xa5 => self.and_n(self.registers.l),
            0xa6 => self.and_n(self.get_hl_address_data()),
            0xa7 => self.and_n(self.registers.a),
            0xa8 => self.xor_n(self.registers.b),
            0xa9 => self.xor_n(self.registers.c),
            0xaa => self.xor_n(self.registers.d),
            0xab => self.xor_n(self.registers.e),
            0xac => self.xor_n(self.registers.h),
            0xad => self.xor_n(self.registers.l),
            0xae => self.xor_n(self.get_hl_address_data()),
            0xaf => self.xor_n(self.registers.a),
            0xb0 => self.or_n(self.registers.b),
            0xb1 => self.or_n(self.registers.c),
            0xb2 => self.or_n(self.registers.d),
            0xb3 => self.or_n(self.registers.e),
            0xb4 => self.or_n(self.registers.h),
            0xb5 => self.or_n(self.registers.l),
            0xb6 => self.or_n(self.get_hl_address_data()),
            0xb7 => self.or_n(self.registers.a),
            0xb8 => self.cp_n(self.registers.b),
            0xb9 => self.cp_n(self.registers.c),
            0xba => self.cp_n(self.registers.d),
            0xbb => self.cp_n(self.registers.e),
            0xbc => self.cp_n(self.registers.h),
            0xbd => self.cp_n(self.registers.l),
            0xbe => self.cp_n(self.get_hl_address_data()),
            0xbf => self.cp_n(self.registers.a),
            0xc0 => {
                let z = self.registers.get_flag(Flags::Z);
                self.ret_cc(z == 0)
            }
            0xc1 => self.pop_nn(Reg::BC),
            0xc2 => {
                let z = self.registers.get_flag(Flags::Z);
                self.jp_cc_nn(z == 0)
            }
            0xc3 => self.jp_cc_nn(true),
            0xc4 => {
                let z = self.registers.get_flag(Flags::Z);
                self.call_cc_nn(z == 0)
            }
            0xc5 => self.push_nn(Reg::BC),
            0xc6 => {
                let n = self.memory.get_next_8();
                self.add_a_n(n)
            }
            0xc7 => self.rst_n(0x0000),
            0xc8 => {
                let z = self.registers.get_flag(Flags::Z);
                self.ret_cc(z == 1)
            }
            0xc9 => self.ret_cc(true),
            0xca => {
                let z = self.registers.get_flag(Flags::Z);
                self.jp_cc_nn(z == 1)
            }
            0xcb => {
                cb_timing = self.cb();
                true
            }
            0xcc => {
                let z = self.registers.get_flag(Flags::Z);
                self.call_cc_nn(z == 1)
            }
            0xcd => self.call_cc_nn(true),
            0xce => {
                let n = self.memory.get_next_8();
                self.addc_a_n(n)
            }
            0xcf => self.rst_n(0x0008),
            0xd0 => {
                let c = self.registers.get_flag(Flags::C);
                self.ret_cc(c == 0)
            }
            0xd1 => self.pop_nn(Reg::DE),
            0xd2 => {
                let c = self.registers.get_flag(Flags::C);
                self.jp_cc_nn(c == 0)
            }
            0xd4 => {
                let c = self.registers.get_flag(Flags::C);
                self.call_cc_nn(c == 0)
            }
            0xd5 => self.push_nn(Reg::DE),
            0xd6 => {
                let n = self.memory.get_next_8();
                self.sub_a_n(n)
            }
            0xd7 => self.rst_n(0x0010),
            0xd8 => {
                let c = self.registers.get_flag(Flags::C);
                self.ret_cc(c == 1)
            }
            0xd9 => {
                let address = self.memory.pop_from_stack();
                self.memory.set_program_counter(address);
                self.clock.set_master_enabled_on();
                true
            }
            0xda => {
                let c = self.registers.get_flag(Flags::C);
                self.jp_cc_nn(c == 1)
            }
            0xdc => {
                let c = self.registers.get_flag(Flags::C);
                self.call_cc_nn(c == 1)
            }
            0xde => {
                let n = self.memory.get_next_8();
                self.subc_a_n(n)
            }
            0xdf => self.rst_n(0x0018),
            0xe0 => {
                let address = 0xff00 | self.memory.get_next_8() as u16;
                let a = self.registers.get_a();
                self.memory.write(address, a);
                true
            }
            0xe1 => self.pop_nn(Reg::HL),
            0xe2 => {
                let a = self.registers.get_a();
                let c = self.registers.get_c();
                self.memory.write(0xff00 | (c as u16), a);
                true
            }
            0xe5 => self.push_nn(Reg::HL),
            0xe6 => {
                let n = self.memory.get_next_8();
                self.and_n(n)
            }
            0xe7 => self.rst_n(0x0020),
            0xe8 => {
                let data = self.memory.get_next_8() as i8 as u16;
                let address = self.memory.get_stack_pointer();
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers
                    .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
                self.registers
                    .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
                self.memory
                    .set_stack_pointer(address.wrapping_add(data as u16));
                true
            }
            0xe9 => {
                let address = self.registers.get_hl();
                self.memory.set_program_counter(address);
                true
            }
            0xea => self.ld_n_a(Reg::N16),
            0xee => {
                let n = self.memory.get_next_8();
                self.xor_n(n)
            }
            0xef => self.rst_n(0x0028),
            0xf0 => {
                let address = 0xff00 | self.memory.get_next_8() as u16;
                self.registers.set_a(self.memory.read(address));
                true
            }
            0xf1 => self.pop_nn(Reg::AF),
            0xf2 => {
                let c = self.registers.get_c();
                let data = self.memory.read(0xff00 | c as u16);
                self.registers.set_a(data);
                true
            }
            0xf3 => self.di(),
            0xf5 => self.push_nn(Reg::AF),
            0xf6 => {
                let n = self.memory.get_next_8();
                self.or_n(n)
            }
            0xf7 => self.rst_n(0x0030),
            0xf8 => {
                let data = self.memory.get_next_8() as i8 as u16;
                let address = self.memory.get_stack_pointer();
                self.registers
                    .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
                self.registers
                    .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_hl(address.wrapping_add(data));
                true
            }
            0xf9 => {
                let address = self.registers.get_hl();
                self.memory.set_stack_pointer(address);
                true
            }
            0xfa => self.ld_a_n(Reg::N16),
            0xfb => self.ei(),
            0xfe => {
                let n = self.memory.get_next_8();
                self.cp_n(n)
            }
            0xff => self.rst_n(0x0038),
            0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {
                panic!("Unexisting code {:X}", opcode)
            }
        };
        if taken {
            return cb_timing + OP_TIMES_TAKEN[opcode as usize] as u32;
        }
        cb_timing + OP_TIMES[opcode as usize] as u32
    }
}

pub fn update(memory: &mut Memory, clock: &mut Clock, registers: &mut Registers) -> u32 {
    let mut cpu = Cpu::new(memory, clock, registers);
    if !cpu.clock.is_halted {
        let opcode = cpu.memory.get_next_8();
        return cpu.execute_opcode(opcode, false) * 4;
    }
    4
}
