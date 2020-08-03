use super::alu::*;
use super::emulator::Emulator;
use super::registers::Flags;
use super::utils::*;

fn read_hl_mem_address(emu: &mut Emulator) -> u8 {
    let hl = emu.registers.get_hl();
    emu.mem_read(hl)
}

fn write_hl_mem_address(data: u8, emu: &mut Emulator) {
    let hl = emu.registers.get_hl();
    emu.mem_write(hl, data)
}

fn execute_opcode(emu: &mut Emulator, opcode: u8, is_callback: bool) {
    if is_callback {
        match opcode {
            0x00 => {
                emu.registers.b = rlc_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x01 => {
                emu.registers.c = rlc_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x02 => {
                emu.registers.d = rlc_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x03 => {
                emu.registers.e = rlc_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x04 => {
                emu.registers.h = rlc_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x05 => {
                emu.registers.l = rlc_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x06 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                let data = rlc_n(hl, &mut emu.registers);
                let hl = emu.registers.get_hl();
                emu.mem_write(hl, data);
            }
            0x07 => {
                emu.registers.a = rlc_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x08 => {
                emu.registers.b = rrc_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x09 => {
                emu.registers.c = rrc_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x0a => {
                emu.registers.d = rrc_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x0b => {
                emu.registers.e = rrc_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x0c => {
                emu.registers.h = rrc_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x0d => {
                emu.registers.l = rrc_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x0e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                let result = rrc_n(hl, &mut emu.registers);
                write_hl_mem_address(result, emu);
            }
            0x0f => {
                emu.registers.a = rrc_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x10 => {
                emu.registers.b = rl_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x11 => {
                emu.registers.c = rl_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x12 => {
                emu.registers.d = rl_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x13 => {
                emu.registers.e = rl_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x14 => {
                emu.registers.h = rl_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x15 => {
                emu.registers.l = rl_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x16 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                let result = rl_n(hl, &mut emu.registers);
                write_hl_mem_address(result, emu);
            }
            0x17 => {
                emu.registers.a = rl_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x18 => {
                emu.registers.b = rr_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x19 => {
                emu.registers.c = rr_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x1a => {
                emu.registers.d = rr_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x1b => {
                emu.registers.e = rr_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x1c => {
                emu.registers.h = rr_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x1d => {
                emu.registers.l = rr_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x1e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                let result = rr_n(hl, &mut emu.registers);
                write_hl_mem_address(result, emu);
            }
            0x1f => {
                emu.registers.a = rr_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x20 => {
                emu.registers.b = sla_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x21 => {
                emu.registers.c = sla_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x22 => {
                emu.registers.d = sla_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x23 => {
                emu.registers.e = sla_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x24 => {
                emu.registers.h = sla_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x25 => {
                emu.registers.l = sla_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x26 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                let result = sla_n(hl, &mut emu.registers);
                write_hl_mem_address(result, emu);
            }
            0x27 => {
                emu.registers.a = sla_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x28 => {
                emu.registers.b = sra_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x29 => {
                emu.registers.c = sra_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x2a => {
                emu.registers.d = sra_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x2b => {
                emu.registers.e = sra_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x2c => {
                emu.registers.h = sra_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x2d => {
                emu.registers.l = sra_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x2e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                let result = sra_n(hl, &mut emu.registers);
                write_hl_mem_address(result, emu);
            }
            0x2f => {
                emu.registers.a = sra_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x30 => {
                emu.registers.b = swap_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x31 => {
                emu.registers.c = swap_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x32 => {
                emu.registers.d = swap_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x33 => {
                emu.registers.e = swap_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x34 => {
                emu.registers.h = swap_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x35 => {
                emu.registers.l = swap_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x36 => {
                emu.take_cycle();
                let result = swap_n(read_hl_mem_address(emu), &mut emu.registers);
                write_hl_mem_address(result, emu);
            }
            0x37 => {
                emu.registers.a = swap_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x38 => {
                emu.registers.b = srl_n(emu.registers.b, &mut emu.registers);
                emu.take_cycle()
            }
            0x39 => {
                emu.registers.c = srl_n(emu.registers.c, &mut emu.registers);
                emu.take_cycle()
            }
            0x3a => {
                emu.registers.d = srl_n(emu.registers.d, &mut emu.registers);
                emu.take_cycle()
            }
            0x3b => {
                emu.registers.e = srl_n(emu.registers.e, &mut emu.registers);
                emu.take_cycle()
            }
            0x3c => {
                emu.registers.h = srl_n(emu.registers.h, &mut emu.registers);
                emu.take_cycle()
            }
            0x3d => {
                emu.registers.l = srl_n(emu.registers.l, &mut emu.registers);
                emu.take_cycle()
            }
            0x3e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                let result = srl_n(hl, &mut emu.registers);
                write_hl_mem_address(result, emu);
            }
            0x3f => {
                emu.registers.a = srl_n(emu.registers.a, &mut emu.registers);
                emu.take_cycle()
            }
            0x40 => {
                bit_b_r(emu.registers.b, 0, &mut emu.registers);
                emu.take_cycle()
            }
            0x41 => {
                bit_b_r(emu.registers.c, 0, &mut emu.registers);
                emu.take_cycle()
            }
            0x42 => {
                bit_b_r(emu.registers.d, 0, &mut emu.registers);
                emu.take_cycle()
            }
            0x43 => {
                bit_b_r(emu.registers.e, 0, &mut emu.registers);
                emu.take_cycle()
            }
            0x44 => {
                bit_b_r(emu.registers.h, 0, &mut emu.registers);
                emu.take_cycle()
            }
            0x45 => {
                bit_b_r(emu.registers.l, 0, &mut emu.registers);
                emu.take_cycle()
            }
            0x46 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 0, &mut emu.registers);
            }
            0x47 => {
                bit_b_r(emu.registers.a, 0, &mut emu.registers);
                emu.take_cycle()
            }
            0x48 => {
                bit_b_r(emu.registers.b, 1, &mut emu.registers);
                emu.take_cycle()
            }
            0x49 => {
                bit_b_r(emu.registers.c, 1, &mut emu.registers);
                emu.take_cycle()
            }
            0x4a => {
                bit_b_r(emu.registers.d, 1, &mut emu.registers);
                emu.take_cycle()
            }
            0x4b => {
                bit_b_r(emu.registers.e, 1, &mut emu.registers);
                emu.take_cycle()
            }
            0x4c => {
                bit_b_r(emu.registers.h, 1, &mut emu.registers);
                emu.take_cycle()
            }
            0x4d => {
                bit_b_r(emu.registers.l, 1, &mut emu.registers);
                emu.take_cycle()
            }
            0x4e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 1, &mut emu.registers);
            }
            0x4f => {
                bit_b_r(emu.registers.a, 1, &mut emu.registers);
                emu.take_cycle()
            }
            0x50 => {
                bit_b_r(emu.registers.b, 2, &mut emu.registers);
                emu.take_cycle()
            }
            0x51 => {
                bit_b_r(emu.registers.c, 2, &mut emu.registers);
                emu.take_cycle()
            }
            0x52 => {
                bit_b_r(emu.registers.d, 2, &mut emu.registers);
                emu.take_cycle()
            }
            0x53 => {
                bit_b_r(emu.registers.e, 2, &mut emu.registers);
                emu.take_cycle()
            }
            0x54 => {
                bit_b_r(emu.registers.h, 2, &mut emu.registers);
                emu.take_cycle()
            }
            0x55 => {
                bit_b_r(emu.registers.l, 2, &mut emu.registers);
                emu.take_cycle()
            }
            0x56 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 2, &mut emu.registers);
            }
            0x57 => {
                bit_b_r(emu.registers.a, 2, &mut emu.registers);
                emu.take_cycle()
            }
            0x58 => {
                bit_b_r(emu.registers.b, 3, &mut emu.registers);
                emu.take_cycle()
            }
            0x59 => {
                bit_b_r(emu.registers.c, 3, &mut emu.registers);
                emu.take_cycle()
            }
            0x5a => {
                bit_b_r(emu.registers.d, 3, &mut emu.registers);
                emu.take_cycle()
            }
            0x5b => {
                bit_b_r(emu.registers.e, 3, &mut emu.registers);
                emu.take_cycle()
            }
            0x5c => {
                bit_b_r(emu.registers.h, 3, &mut emu.registers);
                emu.take_cycle()
            }
            0x5d => {
                bit_b_r(emu.registers.l, 3, &mut emu.registers);
                emu.take_cycle()
            }
            0x5e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 3, &mut emu.registers);
            }
            0x5f => {
                bit_b_r(emu.registers.a, 3, &mut emu.registers);
                emu.take_cycle()
            }
            0x60 => {
                bit_b_r(emu.registers.b, 4, &mut emu.registers);
                emu.take_cycle()
            }
            0x61 => {
                bit_b_r(emu.registers.c, 4, &mut emu.registers);
                emu.take_cycle()
            }
            0x62 => {
                bit_b_r(emu.registers.d, 4, &mut emu.registers);
                emu.take_cycle()
            }
            0x63 => {
                bit_b_r(emu.registers.e, 4, &mut emu.registers);
                emu.take_cycle()
            }
            0x64 => {
                bit_b_r(emu.registers.h, 4, &mut emu.registers);
                emu.take_cycle()
            }
            0x65 => {
                bit_b_r(emu.registers.l, 4, &mut emu.registers);
                emu.take_cycle()
            }
            0x66 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 4, &mut emu.registers);
            }
            0x67 => {
                bit_b_r(emu.registers.a, 4, &mut emu.registers);
                emu.take_cycle()
            }
            0x68 => {
                bit_b_r(emu.registers.b, 5, &mut emu.registers);
                emu.take_cycle()
            }
            0x69 => {
                bit_b_r(emu.registers.c, 5, &mut emu.registers);
                emu.take_cycle()
            }
            0x6a => {
                bit_b_r(emu.registers.d, 5, &mut emu.registers);
                emu.take_cycle()
            }
            0x6b => {
                bit_b_r(emu.registers.e, 5, &mut emu.registers);
                emu.take_cycle()
            }
            0x6c => {
                bit_b_r(emu.registers.h, 5, &mut emu.registers);
                emu.take_cycle()
            }
            0x6d => {
                bit_b_r(emu.registers.l, 5, &mut emu.registers);
                emu.take_cycle()
            }
            0x6e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 5, &mut emu.registers);
            }
            0x6f => {
                bit_b_r(emu.registers.a, 5, &mut emu.registers);
                emu.take_cycle()
            }
            0x70 => {
                bit_b_r(emu.registers.b, 6, &mut emu.registers);
                emu.take_cycle()
            }
            0x71 => {
                bit_b_r(emu.registers.c, 6, &mut emu.registers);
                emu.take_cycle()
            }
            0x72 => {
                bit_b_r(emu.registers.d, 6, &mut emu.registers);
                emu.take_cycle()
            }
            0x73 => {
                bit_b_r(emu.registers.e, 6, &mut emu.registers);
                emu.take_cycle()
            }
            0x74 => {
                bit_b_r(emu.registers.h, 6, &mut emu.registers);
                emu.take_cycle()
            }
            0x75 => {
                bit_b_r(emu.registers.l, 6, &mut emu.registers);
                emu.take_cycle()
            }
            0x76 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 6, &mut emu.registers);
            }
            0x77 => {
                bit_b_r(emu.registers.a, 6, &mut emu.registers);
                emu.take_cycle()
            }
            0x78 => {
                bit_b_r(emu.registers.b, 7, &mut emu.registers);
                emu.take_cycle()
            }
            0x79 => {
                bit_b_r(emu.registers.c, 7, &mut emu.registers);
                emu.take_cycle()
            }
            0x7a => {
                bit_b_r(emu.registers.d, 7, &mut emu.registers);
                emu.take_cycle()
            }
            0x7b => {
                bit_b_r(emu.registers.e, 7, &mut emu.registers);
                emu.take_cycle()
            }
            0x7c => {
                bit_b_r(emu.registers.h, 7, &mut emu.registers);
                emu.take_cycle()
            }
            0x7d => {
                bit_b_r(emu.registers.l, 7, &mut emu.registers);
                emu.take_cycle()
            }
            0x7e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                bit_b_r(hl, 7, &mut emu.registers);
            }
            0x7f => {
                bit_b_r(emu.registers.a, 7, &mut emu.registers);
                emu.take_cycle()
            }
            0x80 => {
                emu.registers.b = res_b_r(emu.registers.b, 0);
                emu.take_cycle()
            }
            0x81 => {
                emu.registers.c = res_b_r(emu.registers.c, 0);
                emu.take_cycle()
            }
            0x82 => {
                emu.registers.d = res_b_r(emu.registers.d, 0);
                emu.take_cycle()
            }
            0x83 => {
                emu.registers.e = res_b_r(emu.registers.e, 0);
                emu.take_cycle()
            }
            0x84 => {
                emu.registers.h = res_b_r(emu.registers.h, 0);
                emu.take_cycle()
            }
            0x85 => {
                emu.registers.l = res_b_r(emu.registers.l, 0);
                emu.take_cycle()
            }
            0x86 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 0), emu);
            }
            0x87 => {
                emu.registers.a = res_b_r(emu.registers.a, 0);
                emu.take_cycle()
            }
            0x88 => {
                emu.registers.b = res_b_r(emu.registers.b, 1);
                emu.take_cycle()
            }
            0x89 => {
                emu.registers.c = res_b_r(emu.registers.c, 1);
                emu.take_cycle()
            }
            0x8a => {
                emu.registers.d = res_b_r(emu.registers.d, 1);
                emu.take_cycle()
            }
            0x8b => {
                emu.registers.e = res_b_r(emu.registers.e, 1);
                emu.take_cycle()
            }
            0x8c => {
                emu.registers.h = res_b_r(emu.registers.h, 1);
                emu.take_cycle()
            }
            0x8d => {
                emu.registers.l = res_b_r(emu.registers.l, 1);
                emu.take_cycle()
            }
            0x8e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 1), emu);
            }
            0x8f => {
                emu.registers.a = res_b_r(emu.registers.a, 1);
                emu.take_cycle()
            }
            0x90 => {
                emu.registers.b = res_b_r(emu.registers.b, 2);
                emu.take_cycle()
            }
            0x91 => {
                emu.registers.c = res_b_r(emu.registers.c, 2);
                emu.take_cycle()
            }
            0x92 => {
                emu.registers.d = res_b_r(emu.registers.d, 2);
                emu.take_cycle()
            }
            0x93 => {
                emu.registers.e = res_b_r(emu.registers.e, 2);
                emu.take_cycle()
            }
            0x94 => {
                emu.registers.h = res_b_r(emu.registers.h, 2);
                emu.take_cycle()
            }
            0x95 => {
                emu.registers.l = res_b_r(emu.registers.l, 2);
                emu.take_cycle()
            }
            0x96 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 2), emu);
            }
            0x97 => {
                emu.registers.a = res_b_r(emu.registers.a, 2);
                emu.take_cycle()
            }
            0x98 => {
                emu.registers.b = res_b_r(emu.registers.b, 3);
                emu.take_cycle()
            }
            0x99 => {
                emu.registers.c = res_b_r(emu.registers.c, 3);
                emu.take_cycle()
            }
            0x9a => {
                emu.registers.d = res_b_r(emu.registers.d, 3);
                emu.take_cycle()
            }
            0x9b => {
                emu.registers.e = res_b_r(emu.registers.e, 3);
                emu.take_cycle()
            }
            0x9c => {
                emu.registers.h = res_b_r(emu.registers.h, 3);
                emu.take_cycle()
            }
            0x9d => {
                emu.registers.l = res_b_r(emu.registers.l, 3);
                emu.take_cycle()
            }
            0x9e => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 3), emu);
            }
            0x9f => {
                emu.registers.a = res_b_r(emu.registers.a, 3);
                emu.take_cycle()
            }
            0xa0 => {
                emu.registers.b = res_b_r(emu.registers.b, 4);
                emu.take_cycle()
            }
            0xa1 => {
                emu.registers.c = res_b_r(emu.registers.c, 4);
                emu.take_cycle()
            }
            0xa2 => {
                emu.registers.d = res_b_r(emu.registers.d, 4);
                emu.take_cycle()
            }
            0xa3 => {
                emu.registers.e = res_b_r(emu.registers.e, 4);
                emu.take_cycle()
            }
            0xa4 => {
                emu.registers.h = res_b_r(emu.registers.h, 4);
                emu.take_cycle()
            }
            0xa5 => {
                emu.registers.l = res_b_r(emu.registers.l, 4);
                emu.take_cycle()
            }
            0xa6 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 4), emu);
            }
            0xa7 => {
                emu.registers.a = res_b_r(emu.registers.a, 4);
                emu.take_cycle()
            }
            0xa8 => {
                emu.registers.b = res_b_r(emu.registers.b, 5);
                emu.take_cycle()
            }
            0xa9 => {
                emu.registers.c = res_b_r(emu.registers.c, 5);
                emu.take_cycle()
            }
            0xaa => {
                emu.registers.d = res_b_r(emu.registers.d, 5);
                emu.take_cycle()
            }
            0xab => {
                emu.registers.e = res_b_r(emu.registers.e, 5);
                emu.take_cycle()
            }
            0xac => {
                emu.registers.h = res_b_r(emu.registers.h, 5);
                emu.take_cycle()
            }
            0xad => {
                emu.registers.l = res_b_r(emu.registers.l, 5);
                emu.take_cycle()
            }
            0xae => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 5), emu);
            }
            0xaf => {
                emu.registers.a = res_b_r(emu.registers.a, 5);
                emu.take_cycle()
            }
            0xb0 => {
                emu.registers.b = res_b_r(emu.registers.b, 6);
                emu.take_cycle()
            }
            0xb1 => {
                emu.registers.c = res_b_r(emu.registers.c, 6);
                emu.take_cycle()
            }
            0xb2 => {
                emu.registers.d = res_b_r(emu.registers.d, 6);
                emu.take_cycle()
            }
            0xb3 => {
                emu.registers.e = res_b_r(emu.registers.e, 6);
                emu.take_cycle()
            }
            0xb4 => {
                emu.registers.h = res_b_r(emu.registers.h, 6);
                emu.take_cycle()
            }
            0xb5 => {
                emu.registers.l = res_b_r(emu.registers.l, 6);
                emu.take_cycle()
            }
            0xb6 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 6), emu);
            }
            0xb7 => {
                emu.registers.a = res_b_r(emu.registers.a, 6);
                emu.take_cycle()
            }
            0xb8 => {
                emu.registers.b = res_b_r(emu.registers.b, 7);
                emu.take_cycle()
            }
            0xb9 => {
                emu.registers.c = res_b_r(emu.registers.c, 7);
                emu.take_cycle()
            }
            0xba => {
                emu.registers.d = res_b_r(emu.registers.d, 7);
                emu.take_cycle()
            }
            0xbb => {
                emu.registers.e = res_b_r(emu.registers.e, 7);
                emu.take_cycle()
            }
            0xbc => {
                emu.registers.h = res_b_r(emu.registers.h, 7);
                emu.take_cycle()
            }
            0xbd => {
                emu.registers.l = res_b_r(emu.registers.l, 7);
                emu.take_cycle()
            }
            0xbe => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(res_b_r(hl, 7), emu);
            }
            0xbf => {
                emu.registers.a = res_b_r(emu.registers.a, 7);
                emu.take_cycle()
            }
            0xc0 => {
                emu.registers.b = set_b_r(emu.registers.b, 0);
                emu.take_cycle()
            }
            0xc1 => {
                emu.registers.c = set_b_r(emu.registers.c, 0);
                emu.take_cycle()
            }
            0xc2 => {
                emu.registers.d = set_b_r(emu.registers.d, 0);
                emu.take_cycle()
            }
            0xc3 => {
                emu.registers.e = set_b_r(emu.registers.e, 0);
                emu.take_cycle()
            }
            0xc4 => {
                emu.registers.h = set_b_r(emu.registers.h, 0);
                emu.take_cycle()
            }
            0xc5 => {
                emu.registers.l = set_b_r(emu.registers.l, 0);
                emu.take_cycle()
            }
            0xc6 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 0), emu);
            }
            0xc7 => {
                emu.registers.a = set_b_r(emu.registers.a, 0);
                emu.take_cycle()
            }
            0xc8 => {
                emu.registers.b = set_b_r(emu.registers.b, 1);
                emu.take_cycle()
            }
            0xc9 => {
                emu.registers.c = set_b_r(emu.registers.c, 1);
                emu.take_cycle()
            }
            0xca => {
                emu.registers.d = set_b_r(emu.registers.d, 1);
                emu.take_cycle()
            }
            0xcb => {
                emu.registers.e = set_b_r(emu.registers.e, 1);
                emu.take_cycle()
            }
            0xcc => {
                emu.registers.h = set_b_r(emu.registers.h, 1);
                emu.take_cycle()
            }
            0xcd => {
                emu.registers.l = set_b_r(emu.registers.l, 1);
                emu.take_cycle()
            }
            0xce => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 1), emu);
            }
            0xcf => {
                emu.registers.a = set_b_r(emu.registers.a, 1);
                emu.take_cycle()
            }
            0xd0 => {
                emu.registers.b = set_b_r(emu.registers.b, 2);
                emu.take_cycle()
            }
            0xd1 => {
                emu.registers.c = set_b_r(emu.registers.c, 2);
                emu.take_cycle()
            }
            0xd2 => {
                emu.registers.d = set_b_r(emu.registers.d, 2);
                emu.take_cycle()
            }
            0xd3 => {
                emu.registers.e = set_b_r(emu.registers.e, 2);
                emu.take_cycle()
            }
            0xd4 => {
                emu.registers.h = set_b_r(emu.registers.h, 2);
                emu.take_cycle()
            }
            0xd5 => {
                emu.registers.l = set_b_r(emu.registers.l, 2);
                emu.take_cycle()
            }
            0xd6 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 2), emu);
            }
            0xd7 => {
                emu.registers.a = set_b_r(emu.registers.a, 2);
                emu.take_cycle()
            }
            0xd8 => {
                emu.registers.b = set_b_r(emu.registers.b, 3);
                emu.take_cycle()
            }
            0xd9 => {
                emu.registers.c = set_b_r(emu.registers.c, 3);
                emu.take_cycle()
            }
            0xda => {
                emu.registers.d = set_b_r(emu.registers.d, 3);
                emu.take_cycle()
            }
            0xdb => {
                emu.registers.e = set_b_r(emu.registers.e, 3);
                emu.take_cycle()
            }
            0xdc => {
                emu.registers.h = set_b_r(emu.registers.h, 3);
                emu.take_cycle()
            }
            0xdd => {
                emu.registers.l = set_b_r(emu.registers.l, 3);
                emu.take_cycle()
            }
            0xde => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 3), emu);
            }
            0xdf => {
                emu.registers.a = set_b_r(emu.registers.a, 3);
                emu.take_cycle()
            }
            0xe0 => {
                emu.registers.b = set_b_r(emu.registers.b, 4);
                emu.take_cycle()
            }
            0xe1 => {
                emu.registers.c = set_b_r(emu.registers.c, 4);
                emu.take_cycle()
            }
            0xe2 => {
                emu.registers.d = set_b_r(emu.registers.d, 4);
                emu.take_cycle()
            }
            0xe3 => {
                emu.registers.e = set_b_r(emu.registers.e, 4);
                emu.take_cycle()
            }
            0xe4 => {
                emu.registers.h = set_b_r(emu.registers.h, 4);
                emu.take_cycle()
            }
            0xe5 => {
                emu.registers.l = set_b_r(emu.registers.l, 4);
                emu.take_cycle()
            }
            0xe6 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 4), emu);
            }
            0xe7 => {
                emu.registers.a = set_b_r(emu.registers.a, 4);
                emu.take_cycle()
            }
            0xe8 => {
                emu.registers.b = set_b_r(emu.registers.b, 5);
                emu.take_cycle()
            }
            0xe9 => {
                emu.registers.c = set_b_r(emu.registers.c, 5);
                emu.take_cycle()
            }
            0xea => {
                emu.registers.d = set_b_r(emu.registers.d, 5);
                emu.take_cycle()
            }
            0xeb => {
                emu.registers.e = set_b_r(emu.registers.e, 5);
                emu.take_cycle()
            }
            0xec => {
                emu.registers.h = set_b_r(emu.registers.h, 5);
                emu.take_cycle()
            }
            0xed => {
                emu.registers.l = set_b_r(emu.registers.l, 5);
                emu.take_cycle()
            }
            0xee => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 5), emu);
            }
            0xef => {
                emu.registers.a = set_b_r(emu.registers.a, 5);
                emu.take_cycle()
            }
            0xf0 => {
                emu.registers.b = set_b_r(emu.registers.b, 6);
                emu.take_cycle()
            }
            0xf1 => {
                emu.registers.c = set_b_r(emu.registers.c, 6);
                emu.take_cycle()
            }
            0xf2 => {
                emu.registers.d = set_b_r(emu.registers.d, 6);
                emu.take_cycle()
            }
            0xf3 => {
                emu.registers.e = set_b_r(emu.registers.e, 6);
                emu.take_cycle()
            }
            0xf4 => {
                emu.registers.h = set_b_r(emu.registers.h, 6);
                emu.take_cycle()
            }
            0xf5 => {
                emu.registers.l = set_b_r(emu.registers.l, 6);
                emu.take_cycle()
            }
            0xf6 => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 6), emu);
            }
            0xf7 => {
                emu.registers.a = set_b_r(emu.registers.a, 6);
                emu.take_cycle()
            }
            0xf8 => {
                emu.registers.b = set_b_r(emu.registers.b, 7);
                emu.take_cycle()
            }
            0xf9 => {
                emu.registers.c = set_b_r(emu.registers.c, 7);
                emu.take_cycle()
            }
            0xfa => {
                emu.registers.d = set_b_r(emu.registers.d, 7);
                emu.take_cycle()
            }
            0xfb => {
                emu.registers.e = set_b_r(emu.registers.e, 7);
                emu.take_cycle()
            }
            0xfc => {
                emu.registers.h = set_b_r(emu.registers.h, 7);
                emu.take_cycle()
            }
            0xfd => {
                emu.registers.l = set_b_r(emu.registers.l, 7);
                emu.take_cycle()
            }
            0xfe => {
                emu.take_cycle();
                let hl = read_hl_mem_address(emu);
                write_hl_mem_address(set_b_r(hl, 7), emu);
            }
            0xff => {
                emu.registers.a = set_b_r(emu.registers.a, 7);
                emu.take_cycle()
            }
        };
        return;
    }
    match opcode {
        0x00 => {}
        0x01 => {
            let data = emu.get_word();
            emu.registers.set_bc(data);
        }
        0x02 => {
            emu.mem_write(emu.registers.get_bc(), emu.registers.get_a());
        }
        0x03 => {
            let bc = emu.registers.get_bc();
            emu.registers.set_bc(bc.wrapping_add(1));
            emu.take_cycle()
        }
        0x04 => {
            emu.registers.b = inc_n(emu.registers.b, &mut emu.registers);
        }
        0x05 => {
            emu.registers.b = dec_n(emu.registers.b, &mut emu.registers);
        }
        0x06 => {
            emu.registers.b = emu.get_byte();
        }
        0x07 => {
            emu.registers.a = rlc_n(emu.registers.a, &mut emu.registers);
            emu.registers.set_flag(Flags::Z, false);
        }
        0x08 => {
            let address = emu.get_word();
            let stack_pointer = emu.memory.get_stack_pointer();
            emu.write_word(address, stack_pointer);
        }
        0x09 => {
            let result = add_hl_n(
                emu.registers.get_hl(),
                emu.registers.get_bc(),
                &mut emu.registers,
            );
            emu.registers.set_hl(result);
            emu.take_cycle()
        }
        0x0a => {
            let data = emu.mem_read(emu.registers.get_bc());
            emu.registers.set_a(data);
        }
        0x0b => {
            let bc = emu.registers.get_bc();
            emu.registers.set_bc(bc.wrapping_sub(1));
            emu.take_cycle()
        }
        0x0c => {
            emu.registers.c = inc_n(emu.registers.c, &mut emu.registers);
        }
        0x0d => {
            emu.registers.c = dec_n(emu.registers.c, &mut emu.registers);
        }
        0x0e => {
            emu.registers.c = emu.get_byte();
        }
        0x0f => {
            emu.registers.a = rrc_n(emu.registers.a, &mut emu.registers);
            emu.registers.set_flag(Flags::Z, false);
        }
        0x10 => {}
        0x11 => {
            let data = emu.get_word();
            emu.registers.set_de(data);
        }
        0x12 => {
            emu.mem_write(emu.registers.get_de(), emu.registers.get_a());
        }
        0x13 => {
            let de = emu.registers.get_de();
            emu.registers.set_de(de.wrapping_add(1));
            emu.take_cycle()
        }
        0x14 => {
            emu.registers.d = inc_n(emu.registers.d, &mut emu.registers);
        }
        0x15 => {
            emu.registers.d = dec_n(emu.registers.d, &mut emu.registers);
        }
        0x16 => {
            emu.registers.d = emu.get_byte();
        }
        0x17 => {
            emu.registers.a = rl_n(emu.registers.a, &mut emu.registers);
            emu.registers.set_flag(Flags::Z, false);
        }
        0x18 => {
            jr_cc_n(true, emu);
        }
        0x19 => {
            let result = add_hl_n(
                emu.registers.get_hl(),
                emu.registers.get_de(),
                &mut emu.registers,
            );
            emu.registers.set_hl(result);
            emu.take_cycle()
        }
        0x1a => {
            let data = emu.mem_read(emu.registers.get_de());
            emu.registers.set_a(data);
        }
        0x1b => {
            let de = emu.registers.get_de();
            emu.registers.set_de(de.wrapping_sub(1));
            emu.take_cycle()
        }
        0x1c => {
            emu.registers.e = inc_n(emu.registers.e, &mut emu.registers);
        }
        0x1d => {
            emu.registers.e = dec_n(emu.registers.e, &mut emu.registers);
        }
        0x1e => {
            emu.registers.e = emu.get_byte();
        }
        0x1f => {
            emu.registers.a = rr_n(emu.registers.a, &mut emu.registers);
            emu.registers.set_flag(Flags::Z, false);
        }
        0x20 => {
            let z = emu.registers.get_flag(Flags::Z);
            jr_cc_n(z == 0, emu);
        }
        0x21 => {
            let data = emu.get_word();
            emu.registers.set_hl(data);
        }
        0x22 => {
            let address = emu.registers.get_hl();
            let a = emu.registers.get_a();
            emu.mem_write(address, a);
            emu.registers.set_hl(address.wrapping_add(1));
        }
        0x23 => {
            let hl = emu.registers.get_hl();
            emu.registers.set_hl(hl.wrapping_add(1));
            emu.take_cycle()
        }
        0x24 => {
            emu.registers.h = inc_n(emu.registers.h, &mut emu.registers);
        }
        0x25 => {
            emu.registers.h = dec_n(emu.registers.h, &mut emu.registers);
        }
        0x26 => {
            emu.registers.h = emu.get_byte();
        }
        0x27 => {
            daa(&mut emu.registers);
        }
        0x28 => {
            let z = emu.registers.get_flag(Flags::Z);
            jr_cc_n(z == 1, emu);
        }
        0x29 => {
            let result = add_hl_n(
                emu.registers.get_hl(),
                emu.registers.get_hl(),
                &mut emu.registers,
            );
            emu.registers.set_hl(result);
            emu.take_cycle()
        }
        0x2a => {
            let address = emu.registers.get_hl();
            let data = emu.mem_read(address);
            emu.registers.set_a(data);
            emu.registers.set_hl(address.wrapping_add(1));
        }
        0x2b => {
            let hl = emu.registers.get_hl();
            emu.registers.set_hl(hl.wrapping_sub(1));
            emu.take_cycle()
        }
        0x2c => {
            emu.registers.l = inc_n(emu.registers.l, &mut emu.registers);
        }
        0x2d => {
            emu.registers.l = dec_n(emu.registers.l, &mut emu.registers);
        }
        0x2e => {
            emu.registers.l = emu.get_byte();
        }
        0x2f => {
            let a = emu.registers.get_a();
            emu.registers.set_a(!a);
            emu.registers.set_flag(Flags::H, true);
            emu.registers.set_flag(Flags::N, true);
        }
        0x30 => {
            let c = emu.registers.get_flag(Flags::C);
            jr_cc_n(c == 0, emu);
        }
        0x31 => {
            let data = emu.get_word();
            emu.memory.set_stack_pointer(data);
        }
        0x32 => {
            let address = emu.registers.get_hl();
            let a = emu.registers.get_a();
            emu.mem_write(address, a);
            emu.registers.set_hl(address.wrapping_sub(1));
        }
        0x33 => {
            emu.memory.increment_stack_pointer(1);
            emu.take_cycle()
        }
        0x34 => {
            let data = read_hl_mem_address(emu);
            emu.registers
                .set_flag(Flags::Z, test_flag_add(data, 1, Flags::Z));
            emu.registers.set_flag(Flags::N, false);
            emu.registers
                .set_flag(Flags::H, test_flag_add(data, 1, Flags::H));
            write_hl_mem_address(data.wrapping_add(1), emu);
        }
        0x35 => {
            let data = read_hl_mem_address(emu);
            emu.registers
                .set_flag(Flags::Z, test_flag_sub(data, 1, Flags::Z));
            emu.registers.set_flag(Flags::N, true);
            emu.registers
                .set_flag(Flags::H, test_flag_sub(data, 1, Flags::H));
            write_hl_mem_address(data.wrapping_sub(1), emu);
        }
        0x36 => {
            let data = emu.get_byte();
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, data);
        }
        0x37 => {
            emu.registers.set_flag(Flags::C, true);
            emu.registers.set_flag(Flags::N, false);
            emu.registers.set_flag(Flags::H, false);
        }
        0x38 => {
            let c = emu.registers.get_flag(Flags::C);
            jr_cc_n(c == 1, emu);
        }
        0x39 => {
            let result = add_hl_n(
                emu.registers.get_hl(),
                emu.memory.get_stack_pointer(),
                &mut emu.registers,
            );
            emu.registers.set_hl(result);
            emu.take_cycle()
        }
        0x3a => {
            let address = emu.registers.get_hl();
            let data = emu.mem_read(address);
            emu.registers.set_a(data);
            emu.registers.set_hl(address.wrapping_sub(1));
        }
        0x3b => {
            emu.memory.decrement_stack_pointer(1);
            emu.take_cycle()
        }
        0x3c => {
            emu.registers.a = inc_n(emu.registers.a, &mut emu.registers);
        }
        0x3d => {
            emu.registers.a = dec_n(emu.registers.a, &mut emu.registers);
        }
        0x3e => {
            emu.registers.a = emu.get_byte();
        }
        0x3f => {
            let c = emu.registers.get_flag(Flags::C);
            emu.registers.set_flag(Flags::C, c == 0);
            emu.registers.set_flag(Flags::N, false);
            emu.registers.set_flag(Flags::H, false);
        }
        0x40 => {}
        0x41 => {
            emu.registers.b = emu.registers.get_c();
        }
        0x42 => {
            emu.registers.b = emu.registers.get_d();
        }
        0x43 => {
            emu.registers.b = emu.registers.get_e();
        }
        0x44 => {
            emu.registers.b = emu.registers.get_h();
        }
        0x45 => {
            emu.registers.b = emu.registers.get_l();
        }
        0x46 => {
            let address = emu.registers.get_hl();
            let hl = emu.mem_read(address);
            emu.registers.b = hl;
        }
        0x47 => {
            emu.registers.b = emu.registers.get_a();
        }
        0x48 => {
            emu.registers.c = emu.registers.get_b();
        }
        0x49 => {}
        0x4a => {
            emu.registers.c = emu.registers.get_d();
        }
        0x4b => {
            emu.registers.c = emu.registers.get_e();
        }
        0x4c => {
            emu.registers.c = emu.registers.get_h();
        }
        0x4d => {
            emu.registers.c = emu.registers.get_l();
        }
        0x4e => {
            let address = emu.registers.get_hl();
            let hl = emu.mem_read(address);
            emu.registers.c = hl;
        }
        0x4f => {
            emu.registers.c = emu.registers.get_a();
        }
        0x50 => {
            emu.registers.d = emu.registers.get_b();
        }
        0x51 => {
            emu.registers.d = emu.registers.get_c();
        }
        0x52 => {}
        0x53 => {
            emu.registers.d = emu.registers.get_e();
        }
        0x54 => {
            emu.registers.d = emu.registers.get_h();
        }
        0x55 => {
            emu.registers.d = emu.registers.get_l();
        }
        0x56 => {
            let address = emu.registers.get_hl();
            let hl = emu.mem_read(address);
            emu.registers.d = hl;
        }
        0x57 => {
            emu.registers.d = emu.registers.get_a();
        }
        0x58 => {
            emu.registers.e = emu.registers.get_b();
        }
        0x59 => {
            emu.registers.e = emu.registers.get_c();
        }
        0x5a => {
            emu.registers.e = emu.registers.get_d();
        }
        0x5b => {}
        0x5c => {
            emu.registers.e = emu.registers.get_h();
        }
        0x5d => {
            emu.registers.e = emu.registers.get_l();
        }
        0x5e => {
            let address = emu.registers.get_hl();
            let hl = emu.mem_read(address);
            emu.registers.e = hl;
        }
        0x5f => {
            emu.registers.e = emu.registers.get_a();
        }
        0x60 => {
            emu.registers.h = emu.registers.get_b();
        }
        0x61 => {
            emu.registers.h = emu.registers.get_c();
        }
        0x62 => {
            emu.registers.h = emu.registers.get_d();
        }
        0x63 => {
            emu.registers.h = emu.registers.get_e();
        }
        0x64 => {}
        0x65 => {
            emu.registers.h = emu.registers.get_l();
        }
        0x66 => {
            let address = emu.registers.get_hl();
            let hl = emu.mem_read(address);
            emu.registers.h = hl;
        }
        0x67 => {
            emu.registers.h = emu.registers.get_a();
        }
        0x68 => {
            emu.registers.l = emu.registers.get_b();
        }
        0x69 => {
            emu.registers.l = emu.registers.get_c();
        }
        0x6a => {
            emu.registers.l = emu.registers.get_d();
        }
        0x6b => {
            emu.registers.l = emu.registers.get_e();
        }
        0x6c => {
            emu.registers.l = emu.registers.get_h();
        }
        0x6d => {}
        0x6e => {
            let address = emu.registers.get_hl();
            let hl = emu.mem_read(address);
            emu.registers.l = hl;
        }
        0x6f => {
            emu.registers.l = emu.registers.get_a();
        }
        0x70 => {
            let b = emu.registers.b;
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, b);
        }
        0x71 => {
            let c = emu.registers.c;
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, c);
        }
        0x72 => {
            let d = emu.registers.d;
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, d);
        }
        0x73 => {
            let e = emu.registers.e;
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, e);
        }
        0x74 => {
            let h = emu.registers.h;
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, h);
        }
        0x75 => {
            let l = emu.registers.l;
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, l);
        }
        0x76 => {
            emu.timers.is_halted = true;
        }
        0x77 => {
            let a = emu.registers.a;
            let hl = emu.registers.get_hl();
            emu.mem_write(hl, a);
        }
        0x78 => {
            emu.registers.a = emu.registers.get_b();
        }
        0x79 => {
            emu.registers.a = emu.registers.get_c();
        }
        0x7a => {
            emu.registers.a = emu.registers.get_d();
        }
        0x7b => {
            emu.registers.a = emu.registers.get_e();
        }
        0x7c => {
            emu.registers.a = emu.registers.get_h();
        }
        0x7d => {
            emu.registers.a = emu.registers.get_l();
        }
        0x7e => {
            let address = emu.registers.get_hl();
            let hl = emu.mem_read(address);
            emu.registers.a = hl;
        }
        0x7f => {}
        0x80 => {
            emu.registers.a = add_a_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0x81 => {
            emu.registers.a = add_a_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0x82 => {
            emu.registers.a = add_a_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0x83 => {
            emu.registers.a = add_a_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0x84 => {
            emu.registers.a = add_a_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0x85 => {
            emu.registers.a = add_a_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0x86 => {
            let hl = read_hl_mem_address(emu);
            emu.registers.a = add_a_n(hl, emu.registers.a, &mut emu.registers);
        }
        0x87 => {
            emu.registers.a = add_a_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0x88 => {
            emu.registers.a = addc_a_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0x89 => {
            emu.registers.a = addc_a_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0x8a => {
            emu.registers.a = addc_a_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0x8b => {
            emu.registers.a = addc_a_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0x8c => {
            emu.registers.a = addc_a_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0x8d => {
            emu.registers.a = addc_a_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0x8e => {
            let hl = read_hl_mem_address(emu);
            emu.registers.a = addc_a_n(hl, emu.registers.a, &mut emu.registers);
        }
        0x8f => {
            emu.registers.a = addc_a_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0x90 => {
            emu.registers.a = sub_a_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0x91 => {
            emu.registers.a = sub_a_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0x92 => {
            emu.registers.a = sub_a_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0x93 => {
            emu.registers.a = sub_a_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0x94 => {
            emu.registers.a = sub_a_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0x95 => {
            emu.registers.a = sub_a_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0x96 => {
            let hl = read_hl_mem_address(emu);
            emu.registers.a = sub_a_n(hl, emu.registers.a, &mut emu.registers);
        }
        0x97 => {
            emu.registers.a = sub_a_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0x98 => {
            emu.registers.a = subc_a_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0x99 => {
            emu.registers.a = subc_a_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0x9a => {
            emu.registers.a = subc_a_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0x9b => {
            emu.registers.a = subc_a_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0x9c => {
            emu.registers.a = subc_a_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0x9d => {
            emu.registers.a = subc_a_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0x9e => {
            let hl = read_hl_mem_address(emu);
            emu.registers.a = subc_a_n(hl, emu.registers.a, &mut emu.registers);
        }
        0x9f => {
            emu.registers.a = subc_a_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0xa0 => {
            emu.registers.a = and_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0xa1 => {
            emu.registers.a = and_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0xa2 => {
            emu.registers.a = and_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0xa3 => {
            emu.registers.a = and_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0xa4 => {
            emu.registers.a = and_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0xa5 => {
            emu.registers.a = and_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0xa6 => {
            let hl = read_hl_mem_address(emu);
            emu.registers.a = and_n(hl, emu.registers.a, &mut emu.registers);
        }
        0xa7 => {
            emu.registers.a = and_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0xa8 => {
            emu.registers.a = xor_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0xa9 => {
            emu.registers.a = xor_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0xaa => {
            emu.registers.a = xor_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0xab => {
            emu.registers.a = xor_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0xac => {
            emu.registers.a = xor_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0xad => {
            emu.registers.a = xor_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0xae => {
            let hl = read_hl_mem_address(emu);
            emu.registers.a = xor_n(hl, emu.registers.a, &mut emu.registers);
        }
        0xaf => {
            emu.registers.a = xor_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0xb0 => {
            emu.registers.a = or_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0xb1 => {
            emu.registers.a = or_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0xb2 => {
            emu.registers.a = or_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0xb3 => {
            emu.registers.a = or_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0xb4 => {
            emu.registers.a = or_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0xb5 => {
            emu.registers.a = or_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0xb6 => {
            let hl = read_hl_mem_address(emu);
            emu.registers.a = or_n(hl, emu.registers.a, &mut emu.registers);
        }
        0xb7 => {
            emu.registers.a = or_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0xb8 => {
            cp_n(emu.registers.b, emu.registers.a, &mut emu.registers);
        }
        0xb9 => {
            cp_n(emu.registers.c, emu.registers.a, &mut emu.registers);
        }
        0xba => {
            cp_n(emu.registers.d, emu.registers.a, &mut emu.registers);
        }
        0xbb => {
            cp_n(emu.registers.e, emu.registers.a, &mut emu.registers);
        }
        0xbc => {
            cp_n(emu.registers.h, emu.registers.a, &mut emu.registers);
        }
        0xbd => {
            cp_n(emu.registers.l, emu.registers.a, &mut emu.registers);
        }
        0xbe => {
            let hl = read_hl_mem_address(emu);
            cp_n(hl, emu.registers.a, &mut emu.registers);
        }
        0xbf => {
            cp_n(emu.registers.a, emu.registers.a, &mut emu.registers);
        }
        0xc0 => {
            let z = emu.registers.get_flag(Flags::Z);
            ret_cc(z == 0, emu);
            emu.take_cycle();
        }
        0xc1 => {
            let data = emu.pop_from_stack();
            emu.registers.set_bc(data);
        }
        0xc2 => {
            let z = emu.registers.get_flag(Flags::Z);
            jp_cc_nn(z == 0, emu);
        }
        0xc3 => {
            jp_cc_nn(true, emu);
        }
        0xc4 => {
            let z = emu.registers.get_flag(Flags::Z);
            call_cc_nn(z == 0, emu);
        }
        0xc5 => {
            let address = emu.registers.get_bc();
            emu.push_to_stack(address);
            emu.take_cycle()
        }
        0xc6 => {
            let n = emu.get_byte();
            emu.registers.a = add_a_n(n, emu.registers.a, &mut emu.registers);
        }
        0xc7 => {
            rst_n(0x0000, emu);
            emu.take_cycle()
        }
        0xc8 => {
            let z = emu.registers.get_flag(Flags::Z);
            ret_cc(z == 1, emu);
            emu.take_cycle();
        }
        0xc9 => {
            ret_cc(true, emu);
        }
        0xca => {
            let z = emu.registers.get_flag(Flags::Z);
            jp_cc_nn(z == 1, emu);
        }
        0xcb => {
            let address = emu.memory.get_byte();
            execute_opcode(emu, address, true);
        }
        0xcc => {
            let z = emu.registers.get_flag(Flags::Z);
            call_cc_nn(z == 1, emu);
        }
        0xcd => {
            call_cc_nn(true, emu);
        }
        0xce => {
            let n = emu.get_byte();
            emu.registers.a = addc_a_n(n, emu.registers.a, &mut emu.registers);
        }
        0xcf => {
            rst_n(0x0008, emu);
            emu.take_cycle()
        }
        0xd0 => {
            let c = emu.registers.get_flag(Flags::C);
            ret_cc(c == 0, emu);
            emu.take_cycle();
        }
        0xd1 => {
            let data = emu.pop_from_stack();
            emu.registers.set_de(data);
        }
        0xd2 => {
            let c = emu.registers.get_flag(Flags::C);
            jp_cc_nn(c == 0, emu);
        }
        0xd4 => {
            let c = emu.registers.get_flag(Flags::C);
            call_cc_nn(c == 0, emu);
        }
        0xd5 => {
            let address = emu.registers.get_de();
            emu.push_to_stack(address);
            emu.take_cycle()
        }
        0xd6 => {
            let n = emu.get_byte();
            emu.registers.a = sub_a_n(n, emu.registers.a, &mut emu.registers);
        }
        0xd7 => {
            rst_n(0x0010, emu);
            emu.take_cycle()
        }
        0xd8 => {
            let c = emu.registers.get_flag(Flags::C);
            ret_cc(c == 1, emu);
            emu.take_cycle();
        }
        0xd9 => {
            let address = emu.pop_from_stack();
            emu.memory.set_program_counter(address);
            emu.timers.set_master_enabled_on();
            emu.take_cycle()
        }
        0xda => {
            let c = emu.registers.get_flag(Flags::C);
            jp_cc_nn(c == 1, emu);
        }
        0xdc => {
            let c = emu.registers.get_flag(Flags::C);
            call_cc_nn(c == 1, emu);
        }
        0xde => {
            let n = emu.get_byte();
            emu.registers.a = subc_a_n(n, emu.registers.a, &mut emu.registers);
        }
        0xdf => {
            rst_n(0x0018, emu);
            emu.take_cycle()
        }
        0xe0 => {
            let address = 0xff00 | emu.get_byte() as u16;
            let a = emu.registers.get_a();
            emu.mem_write(address, a);
        }
        0xe1 => {
            let data = emu.pop_from_stack();
            emu.registers.set_hl(data);
        }
        0xe2 => {
            let a = emu.registers.get_a();
            let c = emu.registers.get_c();
            emu.mem_write(0xff00 | (c as u16), a);
        }
        0xe5 => {
            let address = emu.registers.get_hl();
            emu.push_to_stack(address);
            emu.take_cycle()
        }
        0xe6 => {
            let n = emu.get_byte();
            emu.registers.a = and_n(n, emu.registers.a, &mut emu.registers);
        }
        0xe7 => {
            rst_n(0x0020, emu);
            emu.take_cycle()
        }
        0xe8 => {
            let data = emu.get_byte() as i8 as u16;
            let address = emu.memory.get_stack_pointer();
            emu.registers.set_flag(Flags::Z, false);
            emu.registers.set_flag(Flags::N, false);
            emu.registers
                .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
            emu.registers
                .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
            emu.memory
                .set_stack_pointer(address.wrapping_add(data as u16));
            emu.take_cycle();
            emu.take_cycle();
        }
        0xe9 => {
            let address = emu.registers.get_hl();
            emu.memory.set_program_counter(address);
        }
        0xea => {
            let word = emu.get_word();
            emu.mem_write(word, emu.registers.get_a());
        }
        0xee => {
            let n = emu.get_byte();
            emu.registers.a = xor_n(n, emu.registers.a, &mut emu.registers);
        }
        0xef => {
            rst_n(0x0028, emu);
            emu.take_cycle()
        }
        0xf0 => {
            let address = 0xff00 | emu.get_byte() as u16;
            emu.registers.a = emu.mem_read(address);
        }
        0xf1 => {
            let data = emu.pop_from_stack();
            emu.registers.set_af(data);
        }
        0xf2 => {
            let c = emu.registers.get_c();
            let data = emu.mem_read(0xff00 | c as u16);
            emu.registers.set_a(data);
        }
        0xf3 => {
            emu.timers.clear_master_enabled();
        }
        0xf5 => {
            let address = emu.registers.get_af();
            emu.push_to_stack(address);
            emu.take_cycle()
        }
        0xf6 => {
            let n = emu.get_byte();
            emu.registers.a = or_n(n, emu.registers.a, &mut emu.registers);
        }
        0xf7 => {
            rst_n(0x0030, emu);
            emu.take_cycle()
        }
        0xf8 => {
            let data = emu.get_byte() as i8 as u16;
            let address = emu.memory.get_stack_pointer();
            emu.registers
                .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
            emu.registers
                .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
            emu.registers.set_flag(Flags::Z, false);
            emu.registers.set_flag(Flags::N, false);
            emu.registers.set_hl(address.wrapping_add(data));
            emu.take_cycle()
        }
        0xf9 => {
            let address = emu.registers.get_hl();
            emu.memory.set_stack_pointer(address);
            emu.take_cycle()
        }
        0xfa => {
            let word = emu.get_word();
            let data = emu.mem_read(word);
            emu.registers.set_a(data);
        }
        0xfb => {
            emu.timers.set_master_enabled_on();
        }
        0xfe => {
            let n = emu.get_byte();
            cp_n(n, emu.registers.a, &mut emu.registers);
        }
        0xff => {
            rst_n(0x0038, emu);
            emu.take_cycle()
        }
        0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {
            panic!("Unexisting code {:X}", opcode)
        }
    };
}

pub fn update(emulator: &mut Emulator) {
    if !emulator.timers.is_halted {
        let opcode = emulator.memory.get_byte();
        emulator.take_cycle();
        return execute_opcode(emulator, opcode, false);
    }
    emulator.take_cycle();
}
