use super::alu::*;
use super::emulator::{next, Emulator};
use super::registers::{Flags, Reg};
use super::utils::*;

pub struct Cpu<'a> {
    emu: &'a mut Emulator,
}

impl<'a> Cpu<'a> {
    fn new(emu: &'a mut Emulator) -> Self {
        Self { emu }
    }

    fn set_step(&mut self, s: u8) {
        self.emu.clock.set_step(s as u32 * 4);
    }

    fn exec_mid_instruction_steps(&mut self, s: u8) {
        self.set_step(s);
        next(self.emu, false);
    }

    fn get_hl_address_data(&mut self) -> u8 {
        let hl = self.emu.registers.get_hl();
        self.emu.memory.read(hl)
    }

    fn write_in_hl_address(&mut self, data: u8) {
        let hl = self.emu.registers.get_hl();
        self.emu.memory.write(hl, data)
    }
    //// INSTRUCTIONS
    fn ld_nn_n(&mut self, reg: Reg) -> u8 {
        let next_8 = self.emu.memory.get_byte();
        let _ = match reg {
            Reg::B => self.emu.registers.set_b(next_8),
            Reg::C => self.emu.registers.set_c(next_8),
            Reg::D => self.emu.registers.set_d(next_8),
            Reg::E => self.emu.registers.set_e(next_8),
            Reg::H => self.emu.registers.set_h(next_8),
            Reg::L => self.emu.registers.set_l(next_8),
            _ => panic!("Unsupported fn ld_nn_n"),
        };
        2
    }
    fn ld_n_nn(&mut self, n: Reg) -> u8 {
        let data = self.emu.memory.get_word();
        match n {
            Reg::BC => self.emu.registers.set_bc(data),
            Reg::DE => self.emu.registers.set_de(data),
            Reg::HL => self.emu.registers.set_hl(data),
            Reg::SP => self.emu.memory.set_stack_pointer(data),
            _ => panic!("Unsupported fn ld_n_nn"),
        }
        3
    }
    fn ld_r1_r2(&mut self, r1: Reg, r2: u8) -> u8 {
        match r1 {
            Reg::A => self.emu.registers.set_a(r2),
            Reg::B => self.emu.registers.set_b(r2),
            Reg::C => self.emu.registers.set_c(r2),
            Reg::D => self.emu.registers.set_d(r2),
            Reg::E => self.emu.registers.set_e(r2),
            Reg::H => self.emu.registers.set_h(r2),
            Reg::L => self.emu.registers.set_l(r2),
            _ => panic!("Unsupported fn ld_r1_r2"),
        };
        1
    }
    fn ld_r1_hl(&mut self, r1: Reg) -> u8 {
        let hl = self.emu.registers.get_hl();
        let data = self.emu.memory.read(hl);
        self.ld_r1_r2(r1, data);
        2
    }
    fn ld_hl_r2(&mut self, r2: Reg) -> u8 {
        let data = self.emu.registers.get_reg_u8(&r2);
        let hl = self.emu.registers.get_hl();
        self.emu.memory.write(hl, data);
        2
    }
    fn ld_a_n(&mut self, reg: Reg) -> u8 {
        let address = match reg {
            Reg::BC => self.emu.registers.get_bc(),
            Reg::DE => self.emu.registers.get_de(),
            Reg::HL => self.emu.registers.get_hl(),
            Reg::N16 => self.emu.memory.get_word(),
            _ => panic!("Unsupported fn ld_a_n"),
        };
        let data = self.emu.memory.read(address);
        self.emu.registers.set_a(data);
        2
    }
    fn ld_n_a(&mut self, reg: Reg) -> u8 {
        let address = match reg {
            Reg::BC => self.emu.registers.get_bc(),
            Reg::DE => self.emu.registers.get_de(),
            Reg::HL => self.emu.registers.get_hl(),
            Reg::N16 => self.emu.memory.get_word(),
            _ => panic!("Unsupported fn ld_n_a"),
        };
        let a = self.emu.registers.get_a();
        self.emu.memory.write(address, a);
        2
    }
    fn push_nn(&mut self, reg: Reg) -> u8 {
        let address = match reg {
            Reg::AF => self.emu.registers.get_af(),
            Reg::BC => self.emu.registers.get_bc(),
            Reg::DE => self.emu.registers.get_de(),
            Reg::HL => self.emu.registers.get_hl(),
            _ => panic!("Unsupported fn push_nn"),
        };
        self.emu.memory.push_to_stack(address);
        4
    }
    fn pop_nn(&mut self, reg: Reg) -> u8 {
        let data = self.emu.memory.pop_from_stack();
        match reg {
            Reg::AF => self.emu.registers.set_af(data),
            Reg::BC => self.emu.registers.set_bc(data),
            Reg::DE => self.emu.registers.set_de(data),
            Reg::HL => self.emu.registers.set_hl(data),
            _ => panic!("Unsupported fn pop_nn"),
        }
        3
    }
    fn inc_nn(&mut self, reg: Reg) -> u8 {
        let address = self.emu.registers.get_reg_u16(&reg);
        self.emu
            .registers
            .set_reg_u16(&reg, address.wrapping_add(1));
        2
    }
    fn dec_nn(&mut self, reg: Reg) -> u8 {
        let address = self.emu.registers.get_reg_u16(&reg);
        self.emu
            .registers
            .set_reg_u16(&reg, address.wrapping_sub(1));
        2
    }
    fn di(&mut self) -> u8 {
        self.emu.timers.clear_master_enabled();
        1
    }
    fn ei(&mut self) -> u8 {
        self.emu.timers.set_master_enabled_on();
        1
    }
    fn bit_b_r(&mut self, data: u8, bit: u8) -> u8 {
        let result = data & (1 << bit);
        self.emu.registers.set_flag(Flags::Z, result == 0);
        self.emu.registers.set_flag(Flags::N, false);
        self.emu.registers.set_flag(Flags::H, true);
        2
    }
    fn set_b_r(&mut self, reg: Reg, bit: u8) -> u8 {
        let data = self.emu.registers.get_reg_u8(&reg);
        self.emu.registers.set_reg_u8(&reg, data | (1 << bit));
        2
    }
    fn res_b_r(&mut self, reg: Reg, bit: u8) -> u8 {
        let data = self.emu.registers.get_reg_u8(&reg);
        self.emu.registers.set_reg_u8(&reg, data & !(1 << bit));
        2
    }
    fn daa(&mut self) -> u8 {
        let mut carry = false;
        let a = self.emu.registers.get_a();
        if self.emu.registers.get_flag(Flags::N) == 0 {
            if self.emu.registers.get_flag(Flags::C) == 1 || a > 0x99 {
                self.emu.registers.set_a(a.wrapping_add(0x60));
                carry = true;
            }
            let a = self.emu.registers.get_a();
            if self.emu.registers.get_flag(Flags::H) == 1 || (a & 0x0f) > 0x09 {
                self.emu.registers.set_a(a.wrapping_add(0x06));
            }
        } else if self.emu.registers.get_flag(Flags::C) == 1 {
            carry = true;
            let h = self.emu.registers.get_flag(Flags::H);
            self.emu
                .registers
                .set_a(a.wrapping_add(if h == 1 { 0x9a } else { 0xa0 }));
        } else if self.emu.registers.get_flag(Flags::H) == 1 {
            self.emu.registers.set_a(a.wrapping_add(0xfa));
        }
        let a = self.emu.registers.get_a();
        self.emu.registers.set_flag(Flags::Z, a == 0);
        self.emu.registers.set_flag(Flags::H, false);
        self.emu.registers.set_flag(Flags::C, carry);
        1
    }
}

// Instruction set
impl<'a> Cpu<'a> {
    fn execute_opcode(&mut self, opcode: u8, is_callback: bool) {
        if is_callback {
            let cb_timing = match opcode {
                0x00 => {
                    self.emu.registers.b = rlc_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x01 => {
                    self.emu.registers.c = rlc_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x02 => {
                    self.emu.registers.d = rlc_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x03 => {
                    self.emu.registers.e = rlc_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x04 => {
                    self.emu.registers.h = rlc_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x05 => {
                    self.emu.registers.l = rlc_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x06 => {
                    self.exec_mid_instruction_steps(1);
                    let hl = self.get_hl_address_data();
                    let result = rlc_n(hl, &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x07 => {
                    self.emu.registers.a = rlc_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x08 => {
                    self.emu.registers.b = rrc_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x09 => {
                    self.emu.registers.c = rrc_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x0a => {
                    self.emu.registers.d = rrc_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x0b => {
                    self.emu.registers.e = rrc_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x0c => {
                    self.emu.registers.h = rrc_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x0d => {
                    self.emu.registers.l = rrc_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x0e => {
                    self.exec_mid_instruction_steps(1);
                    let hl = self.get_hl_address_data();
                    let result = rrc_n(hl, &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x0f => {
                    self.emu.registers.a = rrc_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x10 => {
                    self.emu.registers.b = rl_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x11 => {
                    self.emu.registers.c = rl_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x12 => {
                    self.emu.registers.d = rl_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x13 => {
                    self.emu.registers.e = rl_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x14 => {
                    self.emu.registers.h = rl_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x15 => {
                    self.emu.registers.l = rl_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x16 => {
                    self.exec_mid_instruction_steps(1);
                    let hl = self.get_hl_address_data();
                    let result = rl_n(hl, &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x17 => {
                    self.emu.registers.a = rl_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x18 => {
                    self.emu.registers.b = rr_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x19 => {
                    self.emu.registers.c = rr_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x1a => {
                    self.emu.registers.d = rr_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x1b => {
                    self.emu.registers.e = rr_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x1c => {
                    self.emu.registers.h = rr_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x1d => {
                    self.emu.registers.l = rr_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x1e => {
                    self.exec_mid_instruction_steps(1);
                    let hl = self.get_hl_address_data();
                    let result = rr_n(hl, &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x1f => {
                    self.emu.registers.a = rr_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x20 => {
                    self.emu.registers.b = sla_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x21 => {
                    self.emu.registers.c = sla_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x22 => {
                    self.emu.registers.d = sla_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x23 => {
                    self.emu.registers.e = sla_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x24 => {
                    self.emu.registers.h = sla_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x25 => {
                    self.emu.registers.l = sla_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x26 => {
                    self.exec_mid_instruction_steps(1);
                    let hl = self.get_hl_address_data();
                    let result = sla_n(hl, &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x27 => {
                    self.emu.registers.a = sla_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x28 => {
                    self.emu.registers.b = sra_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x29 => {
                    self.emu.registers.c = sra_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x2a => {
                    self.emu.registers.d = sra_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x2b => {
                    self.emu.registers.e = sra_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x2c => {
                    self.emu.registers.h = sra_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x2d => {
                    self.emu.registers.l = sra_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x2e => {
                    self.exec_mid_instruction_steps(1);
                    let hl = self.get_hl_address_data();
                    let result = sra_n(hl, &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x2f => {
                    self.emu.registers.a = sra_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x30 => {
                    self.emu.registers.b = swap_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x31 => {
                    self.emu.registers.c = swap_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x32 => {
                    self.emu.registers.d = swap_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x33 => {
                    self.emu.registers.e = swap_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x34 => {
                    self.emu.registers.h = swap_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x35 => {
                    self.emu.registers.l = swap_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x36 => {
                    self.exec_mid_instruction_steps(1);
                    let result = swap_n(self.get_hl_address_data(), &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x37 => {
                    self.emu.registers.a = swap_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x38 => {
                    self.emu.registers.b = srl_n(self.emu.registers.b, &mut self.emu.registers);
                    2
                }
                0x39 => {
                    self.emu.registers.c = srl_n(self.emu.registers.c, &mut self.emu.registers);
                    2
                }
                0x3a => {
                    self.emu.registers.d = srl_n(self.emu.registers.d, &mut self.emu.registers);
                    2
                }
                0x3b => {
                    self.emu.registers.e = srl_n(self.emu.registers.e, &mut self.emu.registers);
                    2
                }
                0x3c => {
                    self.emu.registers.h = srl_n(self.emu.registers.h, &mut self.emu.registers);
                    2
                }
                0x3d => {
                    self.emu.registers.l = srl_n(self.emu.registers.l, &mut self.emu.registers);
                    2
                }
                0x3e => {
                    self.exec_mid_instruction_steps(1);
                    let hl = self.get_hl_address_data();
                    let result = srl_n(hl, &mut self.emu.registers);
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(result);
                    4
                }
                0x3f => {
                    self.emu.registers.a = srl_n(self.emu.registers.a, &mut self.emu.registers);
                    2
                }
                0x40 => self.bit_b_r(self.emu.registers.b, 0),
                0x41 => self.bit_b_r(self.emu.registers.c, 0),
                0x42 => self.bit_b_r(self.emu.registers.d, 0),
                0x43 => self.bit_b_r(self.emu.registers.e, 0),
                0x44 => self.bit_b_r(self.emu.registers.h, 0),
                0x45 => self.bit_b_r(self.emu.registers.l, 0),
                0x46 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 0);
                    3
                }
                0x47 => self.bit_b_r(self.emu.registers.a, 0),
                0x48 => self.bit_b_r(self.emu.registers.b, 1),
                0x49 => self.bit_b_r(self.emu.registers.c, 1),
                0x4a => self.bit_b_r(self.emu.registers.d, 1),
                0x4b => self.bit_b_r(self.emu.registers.e, 1),
                0x4c => self.bit_b_r(self.emu.registers.h, 1),
                0x4d => self.bit_b_r(self.emu.registers.l, 1),
                0x4e => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 1);
                    3
                }
                0x4f => self.bit_b_r(self.emu.registers.a, 1),
                0x50 => self.bit_b_r(self.emu.registers.b, 2),
                0x51 => self.bit_b_r(self.emu.registers.c, 2),
                0x52 => self.bit_b_r(self.emu.registers.d, 2),
                0x53 => self.bit_b_r(self.emu.registers.e, 2),
                0x54 => self.bit_b_r(self.emu.registers.h, 2),
                0x55 => self.bit_b_r(self.emu.registers.l, 2),
                0x56 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 2);
                    3
                }
                0x57 => self.bit_b_r(self.emu.registers.a, 2),
                0x58 => self.bit_b_r(self.emu.registers.b, 3),
                0x59 => self.bit_b_r(self.emu.registers.c, 3),
                0x5a => self.bit_b_r(self.emu.registers.d, 3),
                0x5b => self.bit_b_r(self.emu.registers.e, 3),
                0x5c => self.bit_b_r(self.emu.registers.h, 3),
                0x5d => self.bit_b_r(self.emu.registers.l, 3),
                0x5e => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 3);
                    3
                }
                0x5f => self.bit_b_r(self.emu.registers.a, 3),
                0x60 => self.bit_b_r(self.emu.registers.b, 4),
                0x61 => self.bit_b_r(self.emu.registers.c, 4),
                0x62 => self.bit_b_r(self.emu.registers.d, 4),
                0x63 => self.bit_b_r(self.emu.registers.e, 4),
                0x64 => self.bit_b_r(self.emu.registers.h, 4),
                0x65 => self.bit_b_r(self.emu.registers.l, 4),
                0x66 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 4);
                    3
                }
                0x67 => self.bit_b_r(self.emu.registers.a, 4),
                0x68 => self.bit_b_r(self.emu.registers.b, 5),
                0x69 => self.bit_b_r(self.emu.registers.c, 5),
                0x6a => self.bit_b_r(self.emu.registers.d, 5),
                0x6b => self.bit_b_r(self.emu.registers.e, 5),
                0x6c => self.bit_b_r(self.emu.registers.h, 5),
                0x6d => self.bit_b_r(self.emu.registers.l, 5),
                0x6e => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 5);
                    3
                }
                0x6f => self.bit_b_r(self.emu.registers.a, 5),
                0x70 => self.bit_b_r(self.emu.registers.b, 6),
                0x71 => self.bit_b_r(self.emu.registers.c, 6),
                0x72 => self.bit_b_r(self.emu.registers.d, 6),
                0x73 => self.bit_b_r(self.emu.registers.e, 6),
                0x74 => self.bit_b_r(self.emu.registers.h, 6),
                0x75 => self.bit_b_r(self.emu.registers.l, 6),
                0x76 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 6);
                    3
                }
                0x77 => self.bit_b_r(self.emu.registers.a, 6),
                0x78 => self.bit_b_r(self.emu.registers.b, 7),
                0x79 => self.bit_b_r(self.emu.registers.c, 7),
                0x7a => self.bit_b_r(self.emu.registers.d, 7),
                0x7b => self.bit_b_r(self.emu.registers.e, 7),
                0x7c => self.bit_b_r(self.emu.registers.h, 7),
                0x7d => self.bit_b_r(self.emu.registers.l, 7),
                0x7e => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.bit_b_r(data, 7);
                    3
                }
                0x7f => self.bit_b_r(self.emu.registers.a, 7),
                0x80 => self.res_b_r(Reg::B, 0),
                0x81 => self.res_b_r(Reg::C, 0),
                0x82 => self.res_b_r(Reg::D, 0),
                0x83 => self.res_b_r(Reg::E, 0),
                0x84 => self.res_b_r(Reg::H, 0),
                0x85 => self.res_b_r(Reg::L, 0),
                0x86 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !1);
                    4
                }
                0x87 => self.res_b_r(Reg::A, 0),
                0x88 => self.res_b_r(Reg::B, 1),
                0x89 => self.res_b_r(Reg::C, 1),
                0x8a => self.res_b_r(Reg::D, 1),
                0x8b => self.res_b_r(Reg::E, 1),
                0x8c => self.res_b_r(Reg::H, 1),
                0x8d => self.res_b_r(Reg::L, 1),
                0x8e => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !(1 << 1));
                    4
                }
                0x8f => self.res_b_r(Reg::A, 1),
                0x90 => self.res_b_r(Reg::B, 2),
                0x91 => self.res_b_r(Reg::C, 2),
                0x92 => self.res_b_r(Reg::D, 2),
                0x93 => self.res_b_r(Reg::E, 2),
                0x94 => self.res_b_r(Reg::H, 2),
                0x95 => self.res_b_r(Reg::L, 2),
                0x96 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !(1 << 2));
                    4
                }
                0x97 => self.res_b_r(Reg::A, 2),
                0x98 => self.res_b_r(Reg::B, 3),
                0x99 => self.res_b_r(Reg::C, 3),
                0x9a => self.res_b_r(Reg::D, 3),
                0x9b => self.res_b_r(Reg::E, 3),
                0x9c => self.res_b_r(Reg::H, 3),
                0x9d => self.res_b_r(Reg::L, 3),
                0x9e => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !(1 << 3));
                    4
                }
                0x9f => self.res_b_r(Reg::A, 3),
                0xa0 => self.res_b_r(Reg::B, 4),
                0xa1 => self.res_b_r(Reg::C, 4),
                0xa2 => self.res_b_r(Reg::D, 4),
                0xa3 => self.res_b_r(Reg::E, 4),
                0xa4 => self.res_b_r(Reg::H, 4),
                0xa5 => self.res_b_r(Reg::L, 4),
                0xa6 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !(1 << 4));
                    4
                }
                0xa7 => self.res_b_r(Reg::A, 4),
                0xa8 => self.res_b_r(Reg::B, 5),
                0xa9 => self.res_b_r(Reg::C, 5),
                0xaa => self.res_b_r(Reg::D, 5),
                0xab => self.res_b_r(Reg::E, 5),
                0xac => self.res_b_r(Reg::H, 5),
                0xad => self.res_b_r(Reg::L, 5),
                0xae => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !(1 << 5));
                    4
                }
                0xaf => self.res_b_r(Reg::A, 5),
                0xb0 => self.res_b_r(Reg::B, 6),
                0xb1 => self.res_b_r(Reg::C, 6),
                0xb2 => self.res_b_r(Reg::D, 6),
                0xb3 => self.res_b_r(Reg::E, 6),
                0xb4 => self.res_b_r(Reg::H, 6),
                0xb5 => self.res_b_r(Reg::L, 6),
                0xb6 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !(1 << 6));
                    4
                }
                0xb7 => self.res_b_r(Reg::A, 6),
                0xb8 => self.res_b_r(Reg::B, 7),
                0xb9 => self.res_b_r(Reg::C, 7),
                0xba => self.res_b_r(Reg::D, 7),
                0xbb => self.res_b_r(Reg::E, 7),
                0xbc => self.res_b_r(Reg::H, 7),
                0xbd => self.res_b_r(Reg::L, 7),
                0xbe => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data & !(1 << 7));
                    4
                }
                0xbf => self.res_b_r(Reg::A, 7),
                0xc0 => self.set_b_r(Reg::B, 0),
                0xc1 => self.set_b_r(Reg::C, 0),
                0xc2 => self.set_b_r(Reg::D, 0),
                0xc3 => self.set_b_r(Reg::E, 0),
                0xc4 => self.set_b_r(Reg::H, 0),
                0xc5 => self.set_b_r(Reg::L, 0),
                0xc6 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | 1);
                    4
                }
                0xc7 => self.set_b_r(Reg::A, 0),
                0xc8 => self.set_b_r(Reg::B, 1),
                0xc9 => self.set_b_r(Reg::C, 1),
                0xca => self.set_b_r(Reg::D, 1),
                0xcb => self.set_b_r(Reg::E, 1),
                0xcc => self.set_b_r(Reg::H, 1),
                0xcd => self.set_b_r(Reg::L, 1),
                0xce => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | (1 << 1));
                    4
                }
                0xcf => self.set_b_r(Reg::A, 1),
                0xd0 => self.set_b_r(Reg::B, 2),
                0xd1 => self.set_b_r(Reg::C, 2),
                0xd2 => self.set_b_r(Reg::D, 2),
                0xd3 => self.set_b_r(Reg::E, 2),
                0xd4 => self.set_b_r(Reg::H, 2),
                0xd5 => self.set_b_r(Reg::L, 2),
                0xd6 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | (1 << 2));
                    4
                }
                0xd7 => self.set_b_r(Reg::A, 2),
                0xd8 => self.set_b_r(Reg::B, 3),
                0xd9 => self.set_b_r(Reg::C, 3),
                0xda => self.set_b_r(Reg::D, 3),
                0xdb => self.set_b_r(Reg::E, 3),
                0xdc => self.set_b_r(Reg::H, 3),
                0xdd => self.set_b_r(Reg::L, 3),
                0xde => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | (1 << 3));
                    4
                }
                0xdf => self.set_b_r(Reg::A, 3),
                0xe0 => self.set_b_r(Reg::B, 4),
                0xe1 => self.set_b_r(Reg::C, 4),
                0xe2 => self.set_b_r(Reg::D, 4),
                0xe3 => self.set_b_r(Reg::E, 4),
                0xe4 => self.set_b_r(Reg::H, 4),
                0xe5 => self.set_b_r(Reg::L, 4),
                0xe6 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | (1 << 4));
                    4
                }
                0xe7 => self.set_b_r(Reg::A, 4),
                0xe8 => self.set_b_r(Reg::B, 5),
                0xe9 => self.set_b_r(Reg::C, 5),
                0xea => self.set_b_r(Reg::D, 5),
                0xeb => self.set_b_r(Reg::E, 5),
                0xec => self.set_b_r(Reg::H, 5),
                0xed => self.set_b_r(Reg::L, 5),
                0xee => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | (1 << 5));
                    4
                }
                0xef => self.set_b_r(Reg::A, 5),
                0xf0 => self.set_b_r(Reg::B, 6),
                0xf1 => self.set_b_r(Reg::C, 6),
                0xf2 => self.set_b_r(Reg::D, 6),
                0xf3 => self.set_b_r(Reg::E, 6),
                0xf4 => self.set_b_r(Reg::H, 6),
                0xf5 => self.set_b_r(Reg::L, 6),
                0xf6 => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | (1 << 6));
                    4
                }
                0xf7 => self.set_b_r(Reg::A, 6),
                0xf8 => self.set_b_r(Reg::B, 7),
                0xf9 => self.set_b_r(Reg::C, 7),
                0xfa => self.set_b_r(Reg::D, 7),
                0xfb => self.set_b_r(Reg::E, 7),
                0xfc => self.set_b_r(Reg::H, 7),
                0xfd => self.set_b_r(Reg::L, 7),
                0xfe => {
                    self.exec_mid_instruction_steps(1);
                    let data = self.get_hl_address_data();
                    self.exec_mid_instruction_steps(1);
                    self.write_in_hl_address(data | (1 << 7));
                    4
                }
                0xff => self.set_b_r(Reg::A, 7),
            };
            return self.set_step(cb_timing);
        }
        let timing = match opcode {
            0x00 => 1,
            0x01 => self.ld_n_nn(Reg::BC),
            0x02 => self.ld_n_a(Reg::BC),
            0x03 => self.inc_nn(Reg::BC),
            0x04 => {
                self.emu.registers.b = inc_n(self.emu.registers.b, &mut self.emu.registers);
                1
            }
            0x05 => {
                self.emu.registers.b = dec_n(self.emu.registers.b, &mut self.emu.registers);
                1
            }
            0x06 => self.ld_nn_n(Reg::B),
            0x07 => {
                self.emu.registers.a = rlc_n(self.emu.registers.a, &mut self.emu.registers);
                self.emu.registers.set_flag(Flags::Z, false);
                1
            }
            0x08 => {
                let address = self.emu.memory.get_word();
                let stack_pointer = self.emu.memory.get_stack_pointer();
                self.emu.memory.write_word(address, stack_pointer);
                5
            }
            0x09 => {
                let result = add_hl_n(
                    self.emu.registers.get_hl(),
                    self.emu.registers.get_bc(),
                    &mut self.emu.registers,
                );
                self.emu.registers.set_hl(result);
                2
            }
            0x0a => self.ld_a_n(Reg::BC),
            0x0b => self.dec_nn(Reg::BC),
            0x0c => {
                self.emu.registers.c = inc_n(self.emu.registers.c, &mut self.emu.registers);
                1
            }
            0x0d => {
                self.emu.registers.c = dec_n(self.emu.registers.c, &mut self.emu.registers);
                1
            }
            0x0e => self.ld_nn_n(Reg::C),
            0x0f => {
                self.emu.registers.a = rrc_n(self.emu.registers.a, &mut self.emu.registers);
                self.emu.registers.set_flag(Flags::Z, false);
                1
            }
            0x10 => 0,
            0x11 => self.ld_n_nn(Reg::DE),
            0x12 => self.ld_n_a(Reg::DE),
            0x13 => self.inc_nn(Reg::DE),
            0x14 => {
                self.emu.registers.d = inc_n(self.emu.registers.d, &mut self.emu.registers);
                1
            }
            0x15 => {
                self.emu.registers.d = dec_n(self.emu.registers.d, &mut self.emu.registers);
                1
            }
            0x16 => self.ld_nn_n(Reg::D),
            0x17 => {
                self.emu.registers.a = rl_n(self.emu.registers.a, &mut self.emu.registers);
                self.emu.registers.set_flag(Flags::Z, false);
                1
            }
            0x18 => {
                jr_cc_n(true, &mut self.emu.memory);
                3
            }
            0x19 => {
                let result = add_hl_n(
                    self.emu.registers.get_hl(),
                    self.emu.registers.get_de(),
                    &mut self.emu.registers,
                );
                self.emu.registers.set_hl(result);
                2
            }
            0x1a => self.ld_a_n(Reg::DE),
            0x1b => self.dec_nn(Reg::DE),
            0x1c => {
                self.emu.registers.e = inc_n(self.emu.registers.e, &mut self.emu.registers);
                1
            }
            0x1d => {
                self.emu.registers.e = dec_n(self.emu.registers.e, &mut self.emu.registers);
                1
            }
            0x1e => self.ld_nn_n(Reg::E),
            0x1f => {
                self.emu.registers.a = rr_n(self.emu.registers.a, &mut self.emu.registers);
                self.emu.registers.set_flag(Flags::Z, false);
                1
            }
            0x20 => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if jr_cc_n(z == 0, &mut self.emu.memory) {
                    3
                } else {
                    2
                }
            }
            0x21 => self.ld_n_nn(Reg::HL),
            0x22 => {
                let address = self.emu.registers.get_hl();
                let a = self.emu.registers.get_a();
                self.emu.memory.write(address, a);
                self.emu.registers.set_hl(address.wrapping_add(1));
                2
            }
            0x23 => self.inc_nn(Reg::HL),
            0x24 => {
                self.emu.registers.h = inc_n(self.emu.registers.h, &mut self.emu.registers);
                1
            }
            0x25 => {
                self.emu.registers.h = dec_n(self.emu.registers.h, &mut self.emu.registers);
                1
            }
            0x26 => self.ld_nn_n(Reg::H),
            0x27 => self.daa(),
            0x28 => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if jr_cc_n(z == 1, &mut self.emu.memory) {
                    3
                } else {
                    2
                }
            }
            0x29 => {
                let result = add_hl_n(
                    self.emu.registers.get_hl(),
                    self.emu.registers.get_hl(),
                    &mut self.emu.registers,
                );
                self.emu.registers.set_hl(result);
                2
            }
            0x2a => {
                let address = self.emu.registers.get_hl();
                let data = self.emu.memory.read(address);
                self.emu.registers.set_a(data);
                self.emu.registers.set_hl(address.wrapping_add(1));
                2
            }
            0x2b => self.dec_nn(Reg::HL),
            0x2c => {
                self.emu.registers.l = inc_n(self.emu.registers.l, &mut self.emu.registers);
                1
            }
            0x2d => {
                self.emu.registers.l = dec_n(self.emu.registers.l, &mut self.emu.registers);
                1
            }
            0x2e => self.ld_nn_n(Reg::L),
            0x2f => {
                let a = self.emu.registers.get_a();
                self.emu.registers.set_a(!a);
                self.emu.registers.set_flag(Flags::H, true);
                self.emu.registers.set_flag(Flags::N, true);
                1
            }
            0x30 => {
                let c = self.emu.registers.get_flag(Flags::C);
                if jr_cc_n(c == 0, &mut self.emu.memory) {
                    3
                } else {
                    2
                }
            }
            0x31 => self.ld_n_nn(Reg::SP),
            0x32 => {
                let address = self.emu.registers.get_hl();
                let a = self.emu.registers.get_a();
                self.emu.memory.write(address, a);
                self.emu.registers.set_hl(address.wrapping_sub(1));
                2
            }
            0x33 => {
                self.emu.memory.increment_stack_pointer(1);
                2
            }
            0x34 => {
                let data = self.get_hl_address_data();
                self.emu
                    .registers
                    .set_flag(Flags::Z, test_flag_add(data, 1, Flags::Z));
                self.emu.registers.set_flag(Flags::N, false);
                self.emu
                    .registers
                    .set_flag(Flags::H, test_flag_add(data, 1, Flags::H));
                self.exec_mid_instruction_steps(1);
                self.write_in_hl_address(data.wrapping_add(1));
                3
            }
            0x35 => {
                let data = self.get_hl_address_data();
                self.emu
                    .registers
                    .set_flag(Flags::Z, test_flag_sub(data, 1, Flags::Z));
                self.emu.registers.set_flag(Flags::N, true);
                self.emu
                    .registers
                    .set_flag(Flags::H, test_flag_sub(data, 1, Flags::H));
                self.exec_mid_instruction_steps(1);
                self.write_in_hl_address(data.wrapping_sub(1));
                3
            }
            0x36 => {
                let data = self.emu.memory.get_byte();
                let hl = self.emu.registers.get_hl();
                self.exec_mid_instruction_steps(1);
                self.emu.memory.write(hl, data);
                2
            }
            0x37 => {
                self.emu.registers.set_flag(Flags::C, true);
                self.emu.registers.set_flag(Flags::N, false);
                self.emu.registers.set_flag(Flags::H, false);
                1
            }
            0x38 => {
                let c = self.emu.registers.get_flag(Flags::C);
                if jr_cc_n(c == 1, &mut self.emu.memory) {
                    3
                } else {
                    2
                }
            }
            0x39 => {
                let result = add_hl_n(
                    self.emu.registers.get_hl(),
                    self.emu.memory.get_stack_pointer(),
                    &mut self.emu.registers,
                );
                self.emu.registers.set_hl(result);
                2
            }
            0x3a => {
                let address = self.emu.registers.get_hl();
                let data = self.emu.memory.read(address);
                self.emu.registers.set_a(data);
                self.emu.registers.set_hl(address.wrapping_sub(1));
                2
            }
            0x3b => {
                self.emu.memory.decrement_stack_pointer(1);
                2
            }
            0x3c => {
                self.emu.registers.a = inc_n(self.emu.registers.a, &mut self.emu.registers);
                1
            }
            0x3d => {
                self.emu.registers.a = dec_n(self.emu.registers.a, &mut self.emu.registers);
                1
            }
            0x3e => {
                let n = self.emu.memory.get_byte();
                self.ld_r1_r2(Reg::A, n);
                2
            }
            0x3f => {
                let c = self.emu.registers.get_flag(Flags::C);
                self.emu.registers.set_flag(Flags::C, c == 0);
                self.emu.registers.set_flag(Flags::N, false);
                self.emu.registers.set_flag(Flags::H, false);
                1
            }
            0x40 => 1,
            0x41 => {
                let c = self.emu.registers.get_c();
                self.ld_r1_r2(Reg::B, c)
            }
            0x42 => {
                let d = self.emu.registers.get_d();
                self.ld_r1_r2(Reg::B, d)
            }
            0x43 => {
                let e = self.emu.registers.get_e();
                self.ld_r1_r2(Reg::B, e)
            }
            0x44 => {
                let h = self.emu.registers.get_h();
                self.ld_r1_r2(Reg::B, h)
            }
            0x45 => {
                let l = self.emu.registers.get_l();
                self.ld_r1_r2(Reg::B, l)
            }
            0x46 => self.ld_r1_hl(Reg::B),
            0x47 => {
                let a = self.emu.registers.get_a();
                self.ld_r1_r2(Reg::B, a)
            }
            0x48 => {
                let b = self.emu.registers.get_b();
                self.ld_r1_r2(Reg::C, b)
            }
            0x49 => 1,
            0x4a => {
                let d = self.emu.registers.get_d();
                self.ld_r1_r2(Reg::C, d)
            }
            0x4b => {
                let e = self.emu.registers.get_e();
                self.ld_r1_r2(Reg::C, e)
            }
            0x4c => {
                let h = self.emu.registers.get_h();
                self.ld_r1_r2(Reg::C, h)
            }
            0x4d => {
                let l = self.emu.registers.get_l();
                self.ld_r1_r2(Reg::C, l)
            }
            0x4e => self.ld_r1_hl(Reg::C),
            0x4f => {
                let a = self.emu.registers.get_a();
                self.ld_r1_r2(Reg::C, a)
            }
            0x50 => {
                let b = self.emu.registers.get_b();
                self.ld_r1_r2(Reg::D, b)
            }
            0x51 => {
                let c = self.emu.registers.get_c();
                self.ld_r1_r2(Reg::D, c)
            }
            0x52 => 1,
            0x53 => {
                let e = self.emu.registers.get_e();
                self.ld_r1_r2(Reg::D, e)
            }
            0x54 => {
                let h = self.emu.registers.get_h();
                self.ld_r1_r2(Reg::D, h)
            }
            0x55 => {
                let l = self.emu.registers.get_l();
                self.ld_r1_r2(Reg::D, l)
            }
            0x56 => self.ld_r1_hl(Reg::D),
            0x57 => {
                let a = self.emu.registers.get_a();
                self.ld_r1_r2(Reg::D, a)
            }
            0x58 => {
                let b = self.emu.registers.get_b();
                self.ld_r1_r2(Reg::E, b)
            }
            0x59 => {
                let c = self.emu.registers.get_c();
                self.ld_r1_r2(Reg::E, c)
            }
            0x5a => {
                let d = self.emu.registers.get_d();
                self.ld_r1_r2(Reg::E, d)
            }
            0x5b => 1,
            0x5c => {
                let h = self.emu.registers.get_h();
                self.ld_r1_r2(Reg::E, h)
            }
            0x5d => {
                let l = self.emu.registers.get_l();
                self.ld_r1_r2(Reg::E, l)
            }
            0x5e => self.ld_r1_hl(Reg::E),
            0x5f => {
                let a = self.emu.registers.get_a();
                self.ld_r1_r2(Reg::E, a)
            }
            0x60 => {
                let b = self.emu.registers.get_b();
                self.ld_r1_r2(Reg::H, b)
            }
            0x61 => {
                let c = self.emu.registers.get_c();
                self.ld_r1_r2(Reg::H, c)
            }
            0x62 => {
                let d = self.emu.registers.get_d();
                self.ld_r1_r2(Reg::H, d)
            }
            0x63 => {
                let e = self.emu.registers.get_e();
                self.ld_r1_r2(Reg::H, e)
            }
            0x64 => 1,
            0x65 => {
                let l = self.emu.registers.get_l();
                self.ld_r1_r2(Reg::H, l)
            }
            0x66 => self.ld_r1_hl(Reg::H),
            0x67 => {
                let a = self.emu.registers.get_a();
                self.ld_r1_r2(Reg::H, a)
            }
            0x68 => {
                let b = self.emu.registers.get_b();
                self.ld_r1_r2(Reg::L, b)
            }
            0x69 => {
                let c = self.emu.registers.get_c();
                self.ld_r1_r2(Reg::L, c)
            }
            0x6a => {
                let d = self.emu.registers.get_d();
                self.ld_r1_r2(Reg::L, d)
            }
            0x6b => {
                let e = self.emu.registers.get_e();
                self.ld_r1_r2(Reg::L, e)
            }
            0x6c => {
                let h = self.emu.registers.get_h();
                self.ld_r1_r2(Reg::L, h)
            }
            0x6d => 1,
            0x6e => self.ld_r1_hl(Reg::L),
            0x6f => {
                let a = self.emu.registers.get_a();
                self.ld_r1_r2(Reg::L, a)
            }
            0x70 => self.ld_hl_r2(Reg::B),
            0x71 => self.ld_hl_r2(Reg::C),
            0x72 => self.ld_hl_r2(Reg::D),
            0x73 => self.ld_hl_r2(Reg::E),
            0x74 => self.ld_hl_r2(Reg::H),
            0x75 => self.ld_hl_r2(Reg::L),
            0x76 => {
                self.emu.timers.is_halted = true;
                0
            }
            0x77 => self.ld_hl_r2(Reg::A),
            0x78 => {
                let b = self.emu.registers.get_b();
                self.ld_r1_r2(Reg::A, b)
            }
            0x79 => {
                let c = self.emu.registers.get_c();
                self.ld_r1_r2(Reg::A, c)
            }
            0x7a => {
                let d = self.emu.registers.get_d();
                self.ld_r1_r2(Reg::A, d)
            }
            0x7b => {
                let e = self.emu.registers.get_e();
                self.ld_r1_r2(Reg::A, e)
            }
            0x7c => {
                let h = self.emu.registers.get_h();
                self.ld_r1_r2(Reg::A, h)
            }
            0x7d => {
                let l = self.emu.registers.get_l();
                self.ld_r1_r2(Reg::A, l)
            }
            0x7e => self.ld_r1_hl(Reg::A),
            0x7f => 1,
            0x80 => {
                self.emu.registers.a = add_a_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x81 => {
                self.emu.registers.a = add_a_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x82 => {
                self.emu.registers.a = add_a_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x83 => {
                self.emu.registers.a = add_a_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x84 => {
                self.emu.registers.a = add_a_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x85 => {
                self.emu.registers.a = add_a_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x86 => {
                let hl = self.get_hl_address_data();
                self.emu.registers.a = add_a_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0x87 => {
                self.emu.registers.a = add_a_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x88 => {
                self.emu.registers.a = addc_a_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x89 => {
                self.emu.registers.a = addc_a_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x8a => {
                self.emu.registers.a = addc_a_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x8b => {
                self.emu.registers.a = addc_a_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x8c => {
                self.emu.registers.a = addc_a_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x8d => {
                self.emu.registers.a = addc_a_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x8e => {
                let hl = self.get_hl_address_data();
                self.emu.registers.a = addc_a_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0x8f => {
                self.emu.registers.a = addc_a_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x90 => {
                self.emu.registers.a = sub_a_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x91 => {
                self.emu.registers.a = sub_a_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x92 => {
                self.emu.registers.a = sub_a_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x93 => {
                self.emu.registers.a = sub_a_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x94 => {
                self.emu.registers.a = sub_a_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x95 => {
                self.emu.registers.a = sub_a_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x96 => {
                let hl = self.get_hl_address_data();
                self.emu.registers.a = sub_a_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0x97 => {
                self.emu.registers.a = sub_a_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x98 => {
                self.emu.registers.a = subc_a_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x99 => {
                self.emu.registers.a = subc_a_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x9a => {
                self.emu.registers.a = subc_a_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x9b => {
                self.emu.registers.a = subc_a_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x9c => {
                self.emu.registers.a = subc_a_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x9d => {
                self.emu.registers.a = subc_a_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0x9e => {
                let hl = self.get_hl_address_data();
                self.emu.registers.a = subc_a_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0x9f => {
                self.emu.registers.a = subc_a_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa0 => {
                self.emu.registers.a = and_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa1 => {
                self.emu.registers.a = and_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa2 => {
                self.emu.registers.a = and_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa3 => {
                self.emu.registers.a = and_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa4 => {
                self.emu.registers.a = and_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa5 => {
                self.emu.registers.a = and_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa6 => {
                let hl = self.get_hl_address_data();
                self.emu.registers.a = and_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xa7 => {
                self.emu.registers.a = and_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa8 => {
                self.emu.registers.a = xor_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xa9 => {
                self.emu.registers.a = xor_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xaa => {
                self.emu.registers.a = xor_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xab => {
                self.emu.registers.a = xor_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xac => {
                self.emu.registers.a = xor_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xad => {
                self.emu.registers.a = xor_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xae => {
                let hl = self.get_hl_address_data();
                self.emu.registers.a = xor_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xaf => {
                self.emu.registers.a = xor_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb0 => {
                self.emu.registers.a = or_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb1 => {
                self.emu.registers.a = or_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb2 => {
                self.emu.registers.a = or_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb3 => {
                self.emu.registers.a = or_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb4 => {
                self.emu.registers.a = or_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb5 => {
                self.emu.registers.a = or_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb6 => {
                let hl = self.get_hl_address_data();
                self.emu.registers.a = or_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xb7 => {
                self.emu.registers.a = or_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb8 => {
                cp_n(
                    self.emu.registers.b,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xb9 => {
                cp_n(
                    self.emu.registers.c,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xba => {
                cp_n(
                    self.emu.registers.d,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xbb => {
                cp_n(
                    self.emu.registers.e,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xbc => {
                cp_n(
                    self.emu.registers.h,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xbd => {
                cp_n(
                    self.emu.registers.l,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xbe => {
                let hl = self.get_hl_address_data();
                cp_n(hl, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xbf => {
                cp_n(
                    self.emu.registers.a,
                    self.emu.registers.a,
                    &mut self.emu.registers,
                );
                1
            }
            0xc0 => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if ret_cc(z == 0, &mut self.emu.memory) {
                    5
                } else {
                    2
                }
            }
            0xc1 => self.pop_nn(Reg::BC),
            0xc2 => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if jp_cc_nn(z == 0, &mut self.emu.memory) {
                    4
                } else {
                    3
                }
            }
            0xc3 => {
                jp_cc_nn(true, &mut self.emu.memory);
                4
            }
            0xc4 => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if call_cc_nn(z == 0, &mut self.emu.memory) {
                    6
                } else {
                    3
                }
            }
            0xc5 => self.push_nn(Reg::BC),
            0xc6 => {
                let n = self.emu.memory.get_byte();
                self.emu.registers.a = add_a_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xc7 => {
                rst_n(0x0000, &mut self.emu.memory);
                4
            }
            0xc8 => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if ret_cc(z == 1, &mut self.emu.memory) {
                    5
                } else {
                    2
                }
            }
            0xc9 => {
                ret_cc(true, &mut self.emu.memory);
                4
            }
            0xca => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if jp_cc_nn(z == 1, &mut self.emu.memory) {
                    4
                } else {
                    3
                }
            }
            0xcb => {
                let address = self.emu.memory.get_byte();
                self.execute_opcode(address, true);
                return;
            }
            0xcc => {
                let z = self.emu.registers.get_flag(Flags::Z);
                if call_cc_nn(z == 1, &mut self.emu.memory) {
                    6
                } else {
                    3
                }
            }
            0xcd => {
                call_cc_nn(true, &mut self.emu.memory);
                6
            }
            0xce => {
                let n = self.emu.memory.get_byte();
                self.emu.registers.a = addc_a_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xcf => {
                rst_n(0x0008, &mut self.emu.memory);
                4
            }
            0xd0 => {
                let c = self.emu.registers.get_flag(Flags::C);
                if ret_cc(c == 0, &mut self.emu.memory) {
                    5
                } else {
                    2
                }
            }
            0xd1 => self.pop_nn(Reg::DE),
            0xd2 => {
                let c = self.emu.registers.get_flag(Flags::C);
                if jp_cc_nn(c == 0, &mut self.emu.memory) {
                    4
                } else {
                    3
                }
            }
            0xd4 => {
                let c = self.emu.registers.get_flag(Flags::C);
                if call_cc_nn(c == 0, &mut self.emu.memory) {
                    6
                } else {
                    3
                }
            }
            0xd5 => self.push_nn(Reg::DE),
            0xd6 => {
                let n = self.emu.memory.get_byte();
                self.emu.registers.a = sub_a_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xd7 => {
                rst_n(0x0010, &mut self.emu.memory);
                4
            }
            0xd8 => {
                let c = self.emu.registers.get_flag(Flags::C);
                if ret_cc(c == 1, &mut self.emu.memory) {
                    5
                } else {
                    2
                }
            }
            0xd9 => {
                let address = self.emu.memory.pop_from_stack();
                self.emu.memory.set_program_counter(address);
                self.emu.timers.set_master_enabled_on();
                4
            }
            0xda => {
                let c = self.emu.registers.get_flag(Flags::C);
                if jp_cc_nn(c == 1, &mut self.emu.memory) {
                    4
                } else {
                    3
                }
            }
            0xdc => {
                let c = self.emu.registers.get_flag(Flags::C);
                if call_cc_nn(c == 1, &mut self.emu.memory) {
                    6
                } else {
                    3
                }
            }
            0xde => {
                let n = self.emu.memory.get_byte();
                self.emu.registers.a = subc_a_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xdf => {
                rst_n(0x0018, &mut self.emu.memory);
                4
            }
            0xe0 => {
                let address = 0xff00 | self.emu.memory.get_byte() as u16;
                let a = self.emu.registers.get_a();
                self.exec_mid_instruction_steps(1);
                self.emu.memory.write(address, a);
                1
            }
            0xe1 => self.pop_nn(Reg::HL),
            0xe2 => {
                let a = self.emu.registers.get_a();
                let c = self.emu.registers.get_c();
                self.emu.memory.write(0xff00 | (c as u16), a);
                2
            }
            0xe5 => self.push_nn(Reg::HL),
            0xe6 => {
                let n = self.emu.memory.get_byte();
                self.emu.registers.a = and_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xe7 => {
                rst_n(0x0020, &mut self.emu.memory);
                4
            }
            0xe8 => {
                let data = self.emu.memory.get_byte() as i8 as u16;
                let address = self.emu.memory.get_stack_pointer();
                self.emu.registers.set_flag(Flags::Z, false);
                self.emu.registers.set_flag(Flags::N, false);
                self.emu
                    .registers
                    .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
                self.emu
                    .registers
                    .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
                self.emu
                    .memory
                    .set_stack_pointer(address.wrapping_add(data as u16));
                4
            }
            0xe9 => {
                let address = self.emu.registers.get_hl();
                self.emu.memory.set_program_counter(address);
                1
            }
            0xea => {
                self.exec_mid_instruction_steps(2);
                self.ld_n_a(Reg::N16);
                4
            }
            0xee => {
                let n = self.emu.memory.get_byte();
                self.emu.registers.a = xor_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xef => {
                rst_n(0x0028, &mut self.emu.memory);
                4
            }
            0xf0 => {
                self.exec_mid_instruction_steps(1);
                let address = 0xff00 | self.emu.memory.get_byte() as u16;
                self.emu.registers.set_a(self.emu.memory.read(address));
                1
            }
            0xf1 => self.pop_nn(Reg::AF),
            0xf2 => {
                let c = self.emu.registers.get_c();
                let data = self.emu.memory.read(0xff00 | c as u16);
                self.emu.registers.set_a(data);
                2
            }
            0xf3 => self.di(),
            0xf5 => self.push_nn(Reg::AF),
            0xf6 => {
                let n = self.emu.memory.get_byte();
                self.emu.registers.a = or_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xf7 => {
                rst_n(0x0030, &mut self.emu.memory);
                4
            }
            0xf8 => {
                let data = self.emu.memory.get_byte() as i8 as u16;
                let address = self.emu.memory.get_stack_pointer();
                self.emu
                    .registers
                    .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
                self.emu
                    .registers
                    .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
                self.emu.registers.set_flag(Flags::Z, false);
                self.emu.registers.set_flag(Flags::N, false);
                self.emu.registers.set_hl(address.wrapping_add(data));
                3
            }
            0xf9 => {
                let address = self.emu.registers.get_hl();
                self.emu.memory.set_stack_pointer(address);
                2
            }
            0xfa => {
                self.exec_mid_instruction_steps(2);
                self.ld_a_n(Reg::N16);
                4
            }
            0xfb => self.ei(),
            0xfe => {
                let n = self.emu.memory.get_byte();
                cp_n(n, self.emu.registers.a, &mut self.emu.registers);
                2
            }
            0xff => {
                rst_n(0x0038, &mut self.emu.memory);
                4
            }
            0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {
                panic!("Unexisting code {:X}", opcode)
            }
        };
        self.set_step(timing);
    }
}

pub fn update(emulator: &mut Emulator) {
    let mut cpu = Cpu::new(emulator);
    if !cpu.emu.timers.is_halted {
        let opcode = cpu.emu.memory.get_byte();
        return cpu.execute_opcode(opcode, false);
    }
    cpu.emu.clock.set_step(1);
}
