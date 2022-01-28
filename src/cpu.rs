use super::alu::*;
use super::interrupts::Action;
use super::emulator::Emulator;
use super::registers::Flags;
use super::utils::*;

fn read_hl_mem_address(ctx: &mut Emulator) -> u8 {
    let hl = ctx.registers.get_hl();
    ctx.mem_read(hl)
}

fn write_hl_mem_address(data: u8, ctx: &mut Emulator) {
    let hl = ctx.registers.get_hl();
    ctx.mem_write(hl, data)
}

fn execute_opcode(ctx: &mut Emulator, opcode: u8, is_callback: bool) {
    if is_callback {
        match opcode {
            0x00 => {
                ctx.registers.b = rlc_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x01 => {
                ctx.registers.c = rlc_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x02 => {
                ctx.registers.d = rlc_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x03 => {
                ctx.registers.e = rlc_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x04 => {
                ctx.registers.h = rlc_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x05 => {
                ctx.registers.l = rlc_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x06 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                let data = rlc_n(hl, &mut ctx.registers);
                let hl = ctx.registers.get_hl();
                ctx.mem_write(hl, data);
            }
            0x07 => {
                ctx.registers.a = rlc_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x08 => {
                ctx.registers.b = rrc_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x09 => {
                ctx.registers.c = rrc_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x0a => {
                ctx.registers.d = rrc_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x0b => {
                ctx.registers.e = rrc_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x0c => {
                ctx.registers.h = rrc_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x0d => {
                ctx.registers.l = rrc_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x0e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                let result = rrc_n(hl, &mut ctx.registers);
                write_hl_mem_address(result, ctx);
            }
            0x0f => {
                ctx.registers.a = rrc_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x10 => {
                ctx.registers.b = rl_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x11 => {
                ctx.registers.c = rl_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x12 => {
                ctx.registers.d = rl_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x13 => {
                ctx.registers.e = rl_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x14 => {
                ctx.registers.h = rl_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x15 => {
                ctx.registers.l = rl_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x16 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                let result = rl_n(hl, &mut ctx.registers);
                write_hl_mem_address(result, ctx);
            }
            0x17 => {
                ctx.registers.a = rl_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x18 => {
                ctx.registers.b = rr_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x19 => {
                ctx.registers.c = rr_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x1a => {
                ctx.registers.d = rr_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x1b => {
                ctx.registers.e = rr_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x1c => {
                ctx.registers.h = rr_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x1d => {
                ctx.registers.l = rr_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x1e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                let result = rr_n(hl, &mut ctx.registers);
                write_hl_mem_address(result, ctx);
            }
            0x1f => {
                ctx.registers.a = rr_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x20 => {
                ctx.registers.b = sla_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x21 => {
                ctx.registers.c = sla_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x22 => {
                ctx.registers.d = sla_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x23 => {
                ctx.registers.e = sla_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x24 => {
                ctx.registers.h = sla_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x25 => {
                ctx.registers.l = sla_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x26 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                let result = sla_n(hl, &mut ctx.registers);
                write_hl_mem_address(result, ctx);
            }
            0x27 => {
                ctx.registers.a = sla_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x28 => {
                ctx.registers.b = sra_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x29 => {
                ctx.registers.c = sra_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x2a => {
                ctx.registers.d = sra_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x2b => {
                ctx.registers.e = sra_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x2c => {
                ctx.registers.h = sra_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x2d => {
                ctx.registers.l = sra_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x2e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                let result = sra_n(hl, &mut ctx.registers);
                write_hl_mem_address(result, ctx);
            }
            0x2f => {
                ctx.registers.a = sra_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x30 => {
                ctx.registers.b = swap_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x31 => {
                ctx.registers.c = swap_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x32 => {
                ctx.registers.d = swap_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x33 => {
                ctx.registers.e = swap_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x34 => {
                ctx.registers.h = swap_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x35 => {
                ctx.registers.l = swap_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x36 => {
                ctx.take_cycle();
                let result = swap_n(read_hl_mem_address(ctx), &mut ctx.registers);
                write_hl_mem_address(result, ctx);
            }
            0x37 => {
                ctx.registers.a = swap_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x38 => {
                ctx.registers.b = srl_n(ctx.registers.b, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x39 => {
                ctx.registers.c = srl_n(ctx.registers.c, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x3a => {
                ctx.registers.d = srl_n(ctx.registers.d, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x3b => {
                ctx.registers.e = srl_n(ctx.registers.e, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x3c => {
                ctx.registers.h = srl_n(ctx.registers.h, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x3d => {
                ctx.registers.l = srl_n(ctx.registers.l, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x3e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                let result = srl_n(hl, &mut ctx.registers);
                write_hl_mem_address(result, ctx);
            }
            0x3f => {
                ctx.registers.a = srl_n(ctx.registers.a, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x40 => {
                bit_b_r(ctx.registers.b, 0, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x41 => {
                bit_b_r(ctx.registers.c, 0, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x42 => {
                bit_b_r(ctx.registers.d, 0, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x43 => {
                bit_b_r(ctx.registers.e, 0, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x44 => {
                bit_b_r(ctx.registers.h, 0, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x45 => {
                bit_b_r(ctx.registers.l, 0, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x46 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 0, &mut ctx.registers);
            }
            0x47 => {
                bit_b_r(ctx.registers.a, 0, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x48 => {
                bit_b_r(ctx.registers.b, 1, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x49 => {
                bit_b_r(ctx.registers.c, 1, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x4a => {
                bit_b_r(ctx.registers.d, 1, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x4b => {
                bit_b_r(ctx.registers.e, 1, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x4c => {
                bit_b_r(ctx.registers.h, 1, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x4d => {
                bit_b_r(ctx.registers.l, 1, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x4e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 1, &mut ctx.registers);
            }
            0x4f => {
                bit_b_r(ctx.registers.a, 1, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x50 => {
                bit_b_r(ctx.registers.b, 2, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x51 => {
                bit_b_r(ctx.registers.c, 2, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x52 => {
                bit_b_r(ctx.registers.d, 2, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x53 => {
                bit_b_r(ctx.registers.e, 2, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x54 => {
                bit_b_r(ctx.registers.h, 2, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x55 => {
                bit_b_r(ctx.registers.l, 2, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x56 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 2, &mut ctx.registers);
            }
            0x57 => {
                bit_b_r(ctx.registers.a, 2, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x58 => {
                bit_b_r(ctx.registers.b, 3, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x59 => {
                bit_b_r(ctx.registers.c, 3, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x5a => {
                bit_b_r(ctx.registers.d, 3, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x5b => {
                bit_b_r(ctx.registers.e, 3, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x5c => {
                bit_b_r(ctx.registers.h, 3, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x5d => {
                bit_b_r(ctx.registers.l, 3, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x5e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 3, &mut ctx.registers);
            }
            0x5f => {
                bit_b_r(ctx.registers.a, 3, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x60 => {
                bit_b_r(ctx.registers.b, 4, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x61 => {
                bit_b_r(ctx.registers.c, 4, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x62 => {
                bit_b_r(ctx.registers.d, 4, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x63 => {
                bit_b_r(ctx.registers.e, 4, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x64 => {
                bit_b_r(ctx.registers.h, 4, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x65 => {
                bit_b_r(ctx.registers.l, 4, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x66 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 4, &mut ctx.registers);
            }
            0x67 => {
                bit_b_r(ctx.registers.a, 4, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x68 => {
                bit_b_r(ctx.registers.b, 5, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x69 => {
                bit_b_r(ctx.registers.c, 5, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x6a => {
                bit_b_r(ctx.registers.d, 5, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x6b => {
                bit_b_r(ctx.registers.e, 5, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x6c => {
                bit_b_r(ctx.registers.h, 5, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x6d => {
                bit_b_r(ctx.registers.l, 5, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x6e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 5, &mut ctx.registers);
            }
            0x6f => {
                bit_b_r(ctx.registers.a, 5, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x70 => {
                bit_b_r(ctx.registers.b, 6, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x71 => {
                bit_b_r(ctx.registers.c, 6, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x72 => {
                bit_b_r(ctx.registers.d, 6, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x73 => {
                bit_b_r(ctx.registers.e, 6, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x74 => {
                bit_b_r(ctx.registers.h, 6, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x75 => {
                bit_b_r(ctx.registers.l, 6, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x76 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 6, &mut ctx.registers);
            }
            0x77 => {
                bit_b_r(ctx.registers.a, 6, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x78 => {
                bit_b_r(ctx.registers.b, 7, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x79 => {
                bit_b_r(ctx.registers.c, 7, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x7a => {
                bit_b_r(ctx.registers.d, 7, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x7b => {
                bit_b_r(ctx.registers.e, 7, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x7c => {
                bit_b_r(ctx.registers.h, 7, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x7d => {
                bit_b_r(ctx.registers.l, 7, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x7e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                bit_b_r(hl, 7, &mut ctx.registers);
            }
            0x7f => {
                bit_b_r(ctx.registers.a, 7, &mut ctx.registers);
                ctx.take_cycle()
            }
            0x80 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 0);
                ctx.take_cycle()
            }
            0x81 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 0);
                ctx.take_cycle()
            }
            0x82 => {
                ctx.registers.d = res_b_r(ctx.registers.d, 0);
                ctx.take_cycle()
            }
            0x83 => {
                ctx.registers.e = res_b_r(ctx.registers.e, 0);
                ctx.take_cycle()
            }
            0x84 => {
                ctx.registers.h = res_b_r(ctx.registers.h, 0);
                ctx.take_cycle()
            }
            0x85 => {
                ctx.registers.l = res_b_r(ctx.registers.l, 0);
                ctx.take_cycle()
            }
            0x86 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 0), ctx);
            }
            0x87 => {
                ctx.registers.a = res_b_r(ctx.registers.a, 0);
                ctx.take_cycle()
            }
            0x88 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 1);
                ctx.take_cycle()
            }
            0x89 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 1);
                ctx.take_cycle()
            }
            0x8a => {
                ctx.registers.d = res_b_r(ctx.registers.d, 1);
                ctx.take_cycle()
            }
            0x8b => {
                ctx.registers.e = res_b_r(ctx.registers.e, 1);
                ctx.take_cycle()
            }
            0x8c => {
                ctx.registers.h = res_b_r(ctx.registers.h, 1);
                ctx.take_cycle()
            }
            0x8d => {
                ctx.registers.l = res_b_r(ctx.registers.l, 1);
                ctx.take_cycle()
            }
            0x8e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 1), ctx);
            }
            0x8f => {
                ctx.registers.a = res_b_r(ctx.registers.a, 1);
                ctx.take_cycle()
            }
            0x90 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 2);
                ctx.take_cycle()
            }
            0x91 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 2);
                ctx.take_cycle()
            }
            0x92 => {
                ctx.registers.d = res_b_r(ctx.registers.d, 2);
                ctx.take_cycle()
            }
            0x93 => {
                ctx.registers.e = res_b_r(ctx.registers.e, 2);
                ctx.take_cycle()
            }
            0x94 => {
                ctx.registers.h = res_b_r(ctx.registers.h, 2);
                ctx.take_cycle()
            }
            0x95 => {
                ctx.registers.l = res_b_r(ctx.registers.l, 2);
                ctx.take_cycle()
            }
            0x96 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 2), ctx);
            }
            0x97 => {
                ctx.registers.a = res_b_r(ctx.registers.a, 2);
                ctx.take_cycle()
            }
            0x98 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 3);
                ctx.take_cycle()
            }
            0x99 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 3);
                ctx.take_cycle()
            }
            0x9a => {
                ctx.registers.d = res_b_r(ctx.registers.d, 3);
                ctx.take_cycle()
            }
            0x9b => {
                ctx.registers.e = res_b_r(ctx.registers.e, 3);
                ctx.take_cycle()
            }
            0x9c => {
                ctx.registers.h = res_b_r(ctx.registers.h, 3);
                ctx.take_cycle()
            }
            0x9d => {
                ctx.registers.l = res_b_r(ctx.registers.l, 3);
                ctx.take_cycle()
            }
            0x9e => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 3), ctx);
            }
            0x9f => {
                ctx.registers.a = res_b_r(ctx.registers.a, 3);
                ctx.take_cycle()
            }
            0xa0 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 4);
                ctx.take_cycle()
            }
            0xa1 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 4);
                ctx.take_cycle()
            }
            0xa2 => {
                ctx.registers.d = res_b_r(ctx.registers.d, 4);
                ctx.take_cycle()
            }
            0xa3 => {
                ctx.registers.e = res_b_r(ctx.registers.e, 4);
                ctx.take_cycle()
            }
            0xa4 => {
                ctx.registers.h = res_b_r(ctx.registers.h, 4);
                ctx.take_cycle()
            }
            0xa5 => {
                ctx.registers.l = res_b_r(ctx.registers.l, 4);
                ctx.take_cycle()
            }
            0xa6 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 4), ctx);
            }
            0xa7 => {
                ctx.registers.a = res_b_r(ctx.registers.a, 4);
                ctx.take_cycle()
            }
            0xa8 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 5);
                ctx.take_cycle()
            }
            0xa9 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 5);
                ctx.take_cycle()
            }
            0xaa => {
                ctx.registers.d = res_b_r(ctx.registers.d, 5);
                ctx.take_cycle()
            }
            0xab => {
                ctx.registers.e = res_b_r(ctx.registers.e, 5);
                ctx.take_cycle()
            }
            0xac => {
                ctx.registers.h = res_b_r(ctx.registers.h, 5);
                ctx.take_cycle()
            }
            0xad => {
                ctx.registers.l = res_b_r(ctx.registers.l, 5);
                ctx.take_cycle()
            }
            0xae => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 5), ctx);
            }
            0xaf => {
                ctx.registers.a = res_b_r(ctx.registers.a, 5);
                ctx.take_cycle()
            }
            0xb0 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 6);
                ctx.take_cycle()
            }
            0xb1 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 6);
                ctx.take_cycle()
            }
            0xb2 => {
                ctx.registers.d = res_b_r(ctx.registers.d, 6);
                ctx.take_cycle()
            }
            0xb3 => {
                ctx.registers.e = res_b_r(ctx.registers.e, 6);
                ctx.take_cycle()
            }
            0xb4 => {
                ctx.registers.h = res_b_r(ctx.registers.h, 6);
                ctx.take_cycle()
            }
            0xb5 => {
                ctx.registers.l = res_b_r(ctx.registers.l, 6);
                ctx.take_cycle()
            }
            0xb6 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 6), ctx);
            }
            0xb7 => {
                ctx.registers.a = res_b_r(ctx.registers.a, 6);
                ctx.take_cycle()
            }
            0xb8 => {
                ctx.registers.b = res_b_r(ctx.registers.b, 7);
                ctx.take_cycle()
            }
            0xb9 => {
                ctx.registers.c = res_b_r(ctx.registers.c, 7);
                ctx.take_cycle()
            }
            0xba => {
                ctx.registers.d = res_b_r(ctx.registers.d, 7);
                ctx.take_cycle()
            }
            0xbb => {
                ctx.registers.e = res_b_r(ctx.registers.e, 7);
                ctx.take_cycle()
            }
            0xbc => {
                ctx.registers.h = res_b_r(ctx.registers.h, 7);
                ctx.take_cycle()
            }
            0xbd => {
                ctx.registers.l = res_b_r(ctx.registers.l, 7);
                ctx.take_cycle()
            }
            0xbe => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(res_b_r(hl, 7), ctx);
            }
            0xbf => {
                ctx.registers.a = res_b_r(ctx.registers.a, 7);
                ctx.take_cycle()
            }
            0xc0 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 0);
                ctx.take_cycle()
            }
            0xc1 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 0);
                ctx.take_cycle()
            }
            0xc2 => {
                ctx.registers.d = set_b_r(ctx.registers.d, 0);
                ctx.take_cycle()
            }
            0xc3 => {
                ctx.registers.e = set_b_r(ctx.registers.e, 0);
                ctx.take_cycle()
            }
            0xc4 => {
                ctx.registers.h = set_b_r(ctx.registers.h, 0);
                ctx.take_cycle()
            }
            0xc5 => {
                ctx.registers.l = set_b_r(ctx.registers.l, 0);
                ctx.take_cycle()
            }
            0xc6 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 0), ctx);
            }
            0xc7 => {
                ctx.registers.a = set_b_r(ctx.registers.a, 0);
                ctx.take_cycle()
            }
            0xc8 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 1);
                ctx.take_cycle()
            }
            0xc9 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 1);
                ctx.take_cycle()
            }
            0xca => {
                ctx.registers.d = set_b_r(ctx.registers.d, 1);
                ctx.take_cycle()
            }
            0xcb => {
                ctx.registers.e = set_b_r(ctx.registers.e, 1);
                ctx.take_cycle()
            }
            0xcc => {
                ctx.registers.h = set_b_r(ctx.registers.h, 1);
                ctx.take_cycle()
            }
            0xcd => {
                ctx.registers.l = set_b_r(ctx.registers.l, 1);
                ctx.take_cycle()
            }
            0xce => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 1), ctx);
            }
            0xcf => {
                ctx.registers.a = set_b_r(ctx.registers.a, 1);
                ctx.take_cycle()
            }
            0xd0 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 2);
                ctx.take_cycle()
            }
            0xd1 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 2);
                ctx.take_cycle()
            }
            0xd2 => {
                ctx.registers.d = set_b_r(ctx.registers.d, 2);
                ctx.take_cycle()
            }
            0xd3 => {
                ctx.registers.e = set_b_r(ctx.registers.e, 2);
                ctx.take_cycle()
            }
            0xd4 => {
                ctx.registers.h = set_b_r(ctx.registers.h, 2);
                ctx.take_cycle()
            }
            0xd5 => {
                ctx.registers.l = set_b_r(ctx.registers.l, 2);
                ctx.take_cycle()
            }
            0xd6 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 2), ctx);
            }
            0xd7 => {
                ctx.registers.a = set_b_r(ctx.registers.a, 2);
                ctx.take_cycle()
            }
            0xd8 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 3);
                ctx.take_cycle()
            }
            0xd9 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 3);
                ctx.take_cycle()
            }
            0xda => {
                ctx.registers.d = set_b_r(ctx.registers.d, 3);
                ctx.take_cycle()
            }
            0xdb => {
                ctx.registers.e = set_b_r(ctx.registers.e, 3);
                ctx.take_cycle()
            }
            0xdc => {
                ctx.registers.h = set_b_r(ctx.registers.h, 3);
                ctx.take_cycle()
            }
            0xdd => {
                ctx.registers.l = set_b_r(ctx.registers.l, 3);
                ctx.take_cycle()
            }
            0xde => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 3), ctx);
            }
            0xdf => {
                ctx.registers.a = set_b_r(ctx.registers.a, 3);
                ctx.take_cycle()
            }
            0xe0 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 4);
                ctx.take_cycle()
            }
            0xe1 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 4);
                ctx.take_cycle()
            }
            0xe2 => {
                ctx.registers.d = set_b_r(ctx.registers.d, 4);
                ctx.take_cycle()
            }
            0xe3 => {
                ctx.registers.e = set_b_r(ctx.registers.e, 4);
                ctx.take_cycle()
            }
            0xe4 => {
                ctx.registers.h = set_b_r(ctx.registers.h, 4);
                ctx.take_cycle()
            }
            0xe5 => {
                ctx.registers.l = set_b_r(ctx.registers.l, 4);
                ctx.take_cycle()
            }
            0xe6 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 4), ctx);
            }
            0xe7 => {
                ctx.registers.a = set_b_r(ctx.registers.a, 4);
                ctx.take_cycle()
            }
            0xe8 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 5);
                ctx.take_cycle()
            }
            0xe9 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 5);
                ctx.take_cycle()
            }
            0xea => {
                ctx.registers.d = set_b_r(ctx.registers.d, 5);
                ctx.take_cycle()
            }
            0xeb => {
                ctx.registers.e = set_b_r(ctx.registers.e, 5);
                ctx.take_cycle()
            }
            0xec => {
                ctx.registers.h = set_b_r(ctx.registers.h, 5);
                ctx.take_cycle()
            }
            0xed => {
                ctx.registers.l = set_b_r(ctx.registers.l, 5);
                ctx.take_cycle()
            }
            0xee => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 5), ctx);
            }
            0xef => {
                ctx.registers.a = set_b_r(ctx.registers.a, 5);
                ctx.take_cycle()
            }
            0xf0 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 6);
                ctx.take_cycle()
            }
            0xf1 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 6);
                ctx.take_cycle()
            }
            0xf2 => {
                ctx.registers.d = set_b_r(ctx.registers.d, 6);
                ctx.take_cycle()
            }
            0xf3 => {
                ctx.registers.e = set_b_r(ctx.registers.e, 6);
                ctx.take_cycle()
            }
            0xf4 => {
                ctx.registers.h = set_b_r(ctx.registers.h, 6);
                ctx.take_cycle()
            }
            0xf5 => {
                ctx.registers.l = set_b_r(ctx.registers.l, 6);
                ctx.take_cycle()
            }
            0xf6 => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 6), ctx);
            }
            0xf7 => {
                ctx.registers.a = set_b_r(ctx.registers.a, 6);
                ctx.take_cycle()
            }
            0xf8 => {
                ctx.registers.b = set_b_r(ctx.registers.b, 7);
                ctx.take_cycle()
            }
            0xf9 => {
                ctx.registers.c = set_b_r(ctx.registers.c, 7);
                ctx.take_cycle()
            }
            0xfa => {
                ctx.registers.d = set_b_r(ctx.registers.d, 7);
                ctx.take_cycle()
            }
            0xfb => {
                ctx.registers.e = set_b_r(ctx.registers.e, 7);
                ctx.take_cycle()
            }
            0xfc => {
                ctx.registers.h = set_b_r(ctx.registers.h, 7);
                ctx.take_cycle()
            }
            0xfd => {
                ctx.registers.l = set_b_r(ctx.registers.l, 7);
                ctx.take_cycle()
            }
            0xfe => {
                ctx.take_cycle();
                let hl = read_hl_mem_address(ctx);
                write_hl_mem_address(set_b_r(hl, 7), ctx);
            }
            0xff => {
                ctx.registers.a = set_b_r(ctx.registers.a, 7);
                ctx.take_cycle()
            }
        };
        return;
    }
    match opcode {
        0x00 => {}
        0x01 => {
            let data = ctx.get_word();
            ctx.registers.set_bc(data);
        }
        0x02 => {
            ctx.mem_write(ctx.registers.get_bc(), ctx.registers.get_a());
        }
        0x03 => {
            let bc = ctx.registers.get_bc();
            let result = bc.wrapping_add(1);
            ctx.take_cycle();
            ctx.registers.set_bc(result);
        }
        0x04 => {
            ctx.registers.b = inc_n(ctx.registers.b, &mut ctx.registers);
        }
        0x05 => {
            ctx.registers.b = dec_n(ctx.registers.b, &mut ctx.registers);
        }
        0x06 => {
            ctx.registers.b = ctx.get_byte();
        }
        0x07 => {
            ctx.registers.a = rlc_n(ctx.registers.a, &mut ctx.registers);
            ctx.registers.set_flag(Flags::Z, false);
        }
        0x08 => {
            let address = ctx.get_word();
            let stack_pointer = ctx.memory.get_sp();
            ctx.write_word(address, stack_pointer);
        }
        0x09 => {
            let result = add_hl_n(
                ctx.registers.get_hl(),
                ctx.registers.get_bc(),
                &mut ctx.registers,
            );
            ctx.take_cycle();
            ctx.registers.set_hl(result);
        }
        0x0a => {
            let data = ctx.mem_read(ctx.registers.get_bc());
            ctx.registers.set_a(data);
        }
        0x0b => {
            let bc = ctx.registers.get_bc();
            let result = bc.wrapping_sub(1);
            ctx.take_cycle();
            ctx.registers.set_bc(result);
        }
        0x0c => {
            ctx.registers.c = inc_n(ctx.registers.c, &mut ctx.registers);
        }
        0x0d => {
            ctx.registers.c = dec_n(ctx.registers.c, &mut ctx.registers);
        }
        0x0e => {
            ctx.registers.c = ctx.get_byte();
        }
        0x0f => {
            ctx.registers.a = rrc_n(ctx.registers.a, &mut ctx.registers);
            ctx.registers.set_flag(Flags::Z, false);
        }
        0x10 => {}
        0x11 => {
            let data = ctx.get_word();
            ctx.registers.set_de(data);
        }
        0x12 => {
            ctx.mem_write(ctx.registers.get_de(), ctx.registers.get_a());
        }
        0x13 => {
            let de = ctx.registers.get_de();
            ctx.take_cycle();
            ctx.registers.set_de(de.wrapping_add(1));
        }
        0x14 => {
            ctx.registers.d = inc_n(ctx.registers.d, &mut ctx.registers);
        }
        0x15 => {
            ctx.registers.d = dec_n(ctx.registers.d, &mut ctx.registers);
        }
        0x16 => {
            ctx.registers.d = ctx.get_byte();
        }
        0x17 => {
            ctx.registers.a = rl_n(ctx.registers.a, &mut ctx.registers);
            ctx.registers.set_flag(Flags::Z, false);
        }
        0x18 => {
            jr_cc_n(true, ctx);
        }
        0x19 => {
            let result = add_hl_n(
                ctx.registers.get_hl(),
                ctx.registers.get_de(),
                &mut ctx.registers,
            );
            ctx.take_cycle();
            ctx.registers.set_hl(result);
        }
        0x1a => {
            let data = ctx.mem_read(ctx.registers.get_de());
            ctx.registers.set_a(data);
        }
        0x1b => {
            let de = ctx.registers.get_de();
            let result = de.wrapping_sub(1);
            ctx.take_cycle();
            ctx.registers.set_de(result);
        }
        0x1c => {
            ctx.registers.e = inc_n(ctx.registers.e, &mut ctx.registers);
        }
        0x1d => {
            ctx.registers.e = dec_n(ctx.registers.e, &mut ctx.registers);
        }
        0x1e => {
            ctx.registers.e = ctx.get_byte();
        }
        0x1f => {
            ctx.registers.a = rr_n(ctx.registers.a, &mut ctx.registers);
            ctx.registers.set_flag(Flags::Z, false);
        }
        0x20 => {
            let z = ctx.registers.get_flag(Flags::Z);
            jr_cc_n(z == 0, ctx);
        }
        0x21 => {
            let data = ctx.get_word();
            ctx.registers.set_hl(data);
        }
        0x22 => {
            let address = ctx.registers.get_hl();
            let a = ctx.registers.get_a();
            ctx.mem_write(address, a);
            ctx.registers.set_hl(address.wrapping_add(1));
        }
        0x23 => {
            let hl = ctx.registers.get_hl();
            ctx.take_cycle();
            ctx.registers.set_hl(hl.wrapping_add(1));
        }
        0x24 => {
            ctx.registers.h = inc_n(ctx.registers.h, &mut ctx.registers);
        }
        0x25 => {
            ctx.registers.h = dec_n(ctx.registers.h, &mut ctx.registers);
        }
        0x26 => {
            ctx.registers.h = ctx.get_byte();
        }
        0x27 => {
            daa(&mut ctx.registers);
        }
        0x28 => {
            let z = ctx.registers.get_flag(Flags::Z);
            jr_cc_n(z == 1, ctx);
        }
        0x29 => {
            let result = add_hl_n(
                ctx.registers.get_hl(),
                ctx.registers.get_hl(),
                &mut ctx.registers,
            );
            ctx.take_cycle();
            ctx.registers.set_hl(result);
        }
        0x2a => {
            let address = ctx.registers.get_hl();
            let data = ctx.mem_read(address);
            ctx.registers.set_a(data);
            ctx.registers.set_hl(address.wrapping_add(1));
        }
        0x2b => {
            let hl = ctx.registers.get_hl();
            let result = hl.wrapping_sub(1);
            ctx.take_cycle();
            ctx.registers.set_hl(result);
        }
        0x2c => {
            ctx.registers.l = inc_n(ctx.registers.l, &mut ctx.registers);
        }
        0x2d => {
            ctx.registers.l = dec_n(ctx.registers.l, &mut ctx.registers);
        }
        0x2e => {
            ctx.registers.l = ctx.get_byte();
        }
        0x2f => {
            let a = ctx.registers.get_a();
            ctx.registers.set_a(!a);
            ctx.registers.set_flag(Flags::H, true);
            ctx.registers.set_flag(Flags::N, true);
        }
        0x30 => {
            let c = ctx.registers.get_flag(Flags::C);
            jr_cc_n(c == 0, ctx);
        }
        0x31 => {
            let data = ctx.get_word();
            ctx.memory.set_sp(data);
        }
        0x32 => {
            let address = ctx.registers.get_hl();
            let a = ctx.registers.get_a();
            ctx.mem_write(address, a);
            ctx.registers.set_hl(address.wrapping_sub(1));
        }
        0x33 => {
            ctx.memory.inc_sp(1);
            ctx.take_cycle();
        }
        0x34 => {
            let data = read_hl_mem_address(ctx);
            ctx.registers
                .set_flag(Flags::Z, test_flag_add(data, 1, Flags::Z));
            ctx.registers.set_flag(Flags::N, false);
            ctx.registers
                .set_flag(Flags::H, test_flag_add(data, 1, Flags::H));
            write_hl_mem_address(data.wrapping_add(1), ctx);
        }
        0x35 => {
            let data = read_hl_mem_address(ctx);
            ctx.registers
                .set_flag(Flags::Z, test_flag_sub(data, 1, Flags::Z));
            ctx.registers.set_flag(Flags::N, true);
            ctx.registers
                .set_flag(Flags::H, test_flag_sub(data, 1, Flags::H));
            write_hl_mem_address(data.wrapping_sub(1), ctx);
        }
        0x36 => {
            let data = ctx.get_byte();
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, data);
        }
        0x37 => {
            ctx.registers.set_flag(Flags::C, true);
            ctx.registers.set_flag(Flags::N, false);
            ctx.registers.set_flag(Flags::H, false);
        }
        0x38 => {
            let c = ctx.registers.get_flag(Flags::C);
            jr_cc_n(c == 1, ctx);
        }
        0x39 => {
            let result = add_hl_n(
                ctx.registers.get_hl(),
                ctx.memory.get_sp(),
                &mut ctx.registers,
            );
            ctx.take_cycle();
            ctx.registers.set_hl(result);
        }
        0x3a => {
            let address = ctx.registers.get_hl();
            let data = ctx.mem_read(address);
            ctx.registers.set_a(data);
            ctx.registers.set_hl(address.wrapping_sub(1));
        }
        0x3b => {
            ctx.memory.dec_sp(1);
            ctx.take_cycle()
        }
        0x3c => {
            ctx.registers.a = inc_n(ctx.registers.a, &mut ctx.registers);
        }
        0x3d => {
            ctx.registers.a = dec_n(ctx.registers.a, &mut ctx.registers);
        }
        0x3e => {
            ctx.registers.a = ctx.get_byte();
        }
        0x3f => {
            let c = ctx.registers.get_flag(Flags::C);
            ctx.registers.set_flag(Flags::C, c == 0);
            ctx.registers.set_flag(Flags::N, false);
            ctx.registers.set_flag(Flags::H, false);
        }
        0x40 => {}
        0x41 => {
            ctx.registers.b = ctx.registers.get_c();
        }
        0x42 => {
            ctx.registers.b = ctx.registers.get_d();
        }
        0x43 => {
            ctx.registers.b = ctx.registers.get_e();
        }
        0x44 => {
            ctx.registers.b = ctx.registers.get_h();
        }
        0x45 => {
            ctx.registers.b = ctx.registers.get_l();
        }
        0x46 => {
            let address = ctx.registers.get_hl();
            let hl = ctx.mem_read(address);
            ctx.registers.b = hl;
        }
        0x47 => {
            ctx.registers.b = ctx.registers.get_a();
        }
        0x48 => {
            ctx.registers.c = ctx.registers.get_b();
        }
        0x49 => {}
        0x4a => {
            ctx.registers.c = ctx.registers.get_d();
        }
        0x4b => {
            ctx.registers.c = ctx.registers.get_e();
        }
        0x4c => {
            ctx.registers.c = ctx.registers.get_h();
        }
        0x4d => {
            ctx.registers.c = ctx.registers.get_l();
        }
        0x4e => {
            let address = ctx.registers.get_hl();
            let hl = ctx.mem_read(address);
            ctx.registers.c = hl;
        }
        0x4f => {
            ctx.registers.c = ctx.registers.get_a();
        }
        0x50 => {
            ctx.registers.d = ctx.registers.get_b();
        }
        0x51 => {
            ctx.registers.d = ctx.registers.get_c();
        }
        0x52 => {}
        0x53 => {
            ctx.registers.d = ctx.registers.get_e();
        }
        0x54 => {
            ctx.registers.d = ctx.registers.get_h();
        }
        0x55 => {
            ctx.registers.d = ctx.registers.get_l();
        }
        0x56 => {
            let address = ctx.registers.get_hl();
            let hl = ctx.mem_read(address);
            ctx.registers.d = hl;
        }
        0x57 => {
            ctx.registers.d = ctx.registers.get_a();
        }
        0x58 => {
            ctx.registers.e = ctx.registers.get_b();
        }
        0x59 => {
            ctx.registers.e = ctx.registers.get_c();
        }
        0x5a => {
            ctx.registers.e = ctx.registers.get_d();
        }
        0x5b => {}
        0x5c => {
            ctx.registers.e = ctx.registers.get_h();
        }
        0x5d => {
            ctx.registers.e = ctx.registers.get_l();
        }
        0x5e => {
            let address = ctx.registers.get_hl();
            let hl = ctx.mem_read(address);
            ctx.registers.e = hl;
        }
        0x5f => {
            ctx.registers.e = ctx.registers.get_a();
        }
        0x60 => {
            ctx.registers.h = ctx.registers.get_b();
        }
        0x61 => {
            ctx.registers.h = ctx.registers.get_c();
        }
        0x62 => {
            ctx.registers.h = ctx.registers.get_d();
        }
        0x63 => {
            ctx.registers.h = ctx.registers.get_e();
        }
        0x64 => {}
        0x65 => {
            ctx.registers.h = ctx.registers.get_l();
        }
        0x66 => {
            let address = ctx.registers.get_hl();
            let hl = ctx.mem_read(address);
            ctx.registers.h = hl;
        }
        0x67 => {
            ctx.registers.h = ctx.registers.get_a();
        }
        0x68 => {
            ctx.registers.l = ctx.registers.get_b();
        }
        0x69 => {
            ctx.registers.l = ctx.registers.get_c();
        }
        0x6a => {
            ctx.registers.l = ctx.registers.get_d();
        }
        0x6b => {
            ctx.registers.l = ctx.registers.get_e();
        }
        0x6c => {
            ctx.registers.l = ctx.registers.get_h();
        }
        0x6d => {}
        0x6e => {
            let address = ctx.registers.get_hl();
            let hl = ctx.mem_read(address);
            ctx.registers.l = hl;
        }
        0x6f => {
            ctx.registers.l = ctx.registers.get_a();
        }
        0x70 => {
            let b = ctx.registers.b;
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, b);
        }
        0x71 => {
            let c = ctx.registers.c;
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, c);
        }
        0x72 => {
            let d = ctx.registers.d;
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, d);
        }
        0x73 => {
            let e = ctx.registers.e;
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, e);
        }
        0x74 => {
            let h = ctx.registers.h;
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, h);
        }
        0x75 => {
            let l = ctx.registers.l;
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, l);
        }
        0x76 => {
            ctx.timers.is_halted = true;
        }
        0x77 => {
            let a = ctx.registers.a;
            let hl = ctx.registers.get_hl();
            ctx.mem_write(hl, a);
        }
        0x78 => {
            ctx.registers.a = ctx.registers.get_b();
        }
        0x79 => {
            ctx.registers.a = ctx.registers.get_c();
        }
        0x7a => {
            ctx.registers.a = ctx.registers.get_d();
        }
        0x7b => {
            ctx.registers.a = ctx.registers.get_e();
        }
        0x7c => {
            ctx.registers.a = ctx.registers.get_h();
        }
        0x7d => {
            ctx.registers.a = ctx.registers.get_l();
        }
        0x7e => {
            let address = ctx.registers.get_hl();
            let hl = ctx.mem_read(address);
            ctx.registers.a = hl;
        }
        0x7f => {}
        0x80 => {
            ctx.registers.a = add_a_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0x81 => {
            ctx.registers.a = add_a_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0x82 => {
            ctx.registers.a = add_a_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0x83 => {
            ctx.registers.a = add_a_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0x84 => {
            ctx.registers.a = add_a_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0x85 => {
            ctx.registers.a = add_a_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0x86 => {
            let hl = read_hl_mem_address(ctx);
            ctx.registers.a = add_a_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0x87 => {
            ctx.registers.a = add_a_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0x88 => {
            ctx.registers.a = addc_a_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0x89 => {
            ctx.registers.a = addc_a_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0x8a => {
            ctx.registers.a = addc_a_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0x8b => {
            ctx.registers.a = addc_a_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0x8c => {
            ctx.registers.a = addc_a_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0x8d => {
            ctx.registers.a = addc_a_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0x8e => {
            let hl = read_hl_mem_address(ctx);
            ctx.registers.a = addc_a_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0x8f => {
            ctx.registers.a = addc_a_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0x90 => {
            ctx.registers.a = sub_a_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0x91 => {
            ctx.registers.a = sub_a_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0x92 => {
            ctx.registers.a = sub_a_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0x93 => {
            ctx.registers.a = sub_a_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0x94 => {
            ctx.registers.a = sub_a_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0x95 => {
            ctx.registers.a = sub_a_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0x96 => {
            let hl = read_hl_mem_address(ctx);
            ctx.registers.a = sub_a_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0x97 => {
            ctx.registers.a = sub_a_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0x98 => {
            ctx.registers.a = subc_a_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0x99 => {
            ctx.registers.a = subc_a_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0x9a => {
            ctx.registers.a = subc_a_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0x9b => {
            ctx.registers.a = subc_a_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0x9c => {
            ctx.registers.a = subc_a_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0x9d => {
            ctx.registers.a = subc_a_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0x9e => {
            let hl = read_hl_mem_address(ctx);
            ctx.registers.a = subc_a_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0x9f => {
            ctx.registers.a = subc_a_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0xa0 => {
            ctx.registers.a = and_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0xa1 => {
            ctx.registers.a = and_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0xa2 => {
            ctx.registers.a = and_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0xa3 => {
            ctx.registers.a = and_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0xa4 => {
            ctx.registers.a = and_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0xa5 => {
            ctx.registers.a = and_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0xa6 => {
            let hl = read_hl_mem_address(ctx);
            ctx.registers.a = and_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0xa7 => {
            ctx.registers.a = and_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0xa8 => {
            ctx.registers.a = xor_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0xa9 => {
            ctx.registers.a = xor_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0xaa => {
            ctx.registers.a = xor_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0xab => {
            ctx.registers.a = xor_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0xac => {
            ctx.registers.a = xor_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0xad => {
            ctx.registers.a = xor_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0xae => {
            let hl = read_hl_mem_address(ctx);
            ctx.registers.a = xor_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0xaf => {
            ctx.registers.a = xor_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0xb0 => {
            ctx.registers.a = or_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0xb1 => {
            ctx.registers.a = or_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0xb2 => {
            ctx.registers.a = or_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0xb3 => {
            ctx.registers.a = or_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0xb4 => {
            ctx.registers.a = or_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0xb5 => {
            ctx.registers.a = or_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0xb6 => {
            let hl = read_hl_mem_address(ctx);
            ctx.registers.a = or_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0xb7 => {
            ctx.registers.a = or_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0xb8 => {
            cp_n(ctx.registers.b, ctx.registers.a, &mut ctx.registers);
        }
        0xb9 => {
            cp_n(ctx.registers.c, ctx.registers.a, &mut ctx.registers);
        }
        0xba => {
            cp_n(ctx.registers.d, ctx.registers.a, &mut ctx.registers);
        }
        0xbb => {
            cp_n(ctx.registers.e, ctx.registers.a, &mut ctx.registers);
        }
        0xbc => {
            cp_n(ctx.registers.h, ctx.registers.a, &mut ctx.registers);
        }
        0xbd => {
            cp_n(ctx.registers.l, ctx.registers.a, &mut ctx.registers);
        }
        0xbe => {
            let hl = read_hl_mem_address(ctx);
            cp_n(hl, ctx.registers.a, &mut ctx.registers);
        }
        0xbf => {
            cp_n(ctx.registers.a, ctx.registers.a, &mut ctx.registers);
        }
        0xc0 => {
            let z = ctx.registers.get_flag(Flags::Z);
            ret_cc(z == 0, ctx);
        }
        0xc1 => {
            let data = ctx.s_pop();
            ctx.registers.set_bc(data);
        }
        0xc2 => {
            let z = ctx.registers.get_flag(Flags::Z);
            jp_cc_nn(z == 0, ctx);
        }
        0xc3 => {
            jp_cc_nn(true, ctx);
        }
        0xc4 => {
            let z = ctx.registers.get_flag(Flags::Z);
            call_cc_nn(z == 0, ctx);
        }
        0xc5 => {
            let address = ctx.registers.get_bc();
            ctx.take_cycle();
            ctx.s_push(address);
        }
        0xc6 => {
            let n = ctx.get_byte();
            ctx.registers.a = add_a_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xc7 => {
            rst_n(0x0000, ctx);
        }
        0xc8 => {
            let z = ctx.registers.get_flag(Flags::Z);
            ret_cc(z == 1, ctx);
        }
        0xc9 => {
            ret(ctx);
        }
        0xca => {
            let z = ctx.registers.get_flag(Flags::Z);
            jp_cc_nn(z == 1, ctx);
        }
        0xcb => {
            let address = ctx.fetch_opcode();
            execute_opcode(ctx, address, true);
        }
        0xcc => {
            let z = ctx.registers.get_flag(Flags::Z);
            call_cc_nn(z == 1, ctx);
        }
        0xcd => {
            call_cc_nn(true, ctx);
        }
        0xce => {
            let n = ctx.get_byte();
            ctx.registers.a = addc_a_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xcf => {
            rst_n(0x0008, ctx);
        }
        0xd0 => {
            let c = ctx.registers.get_flag(Flags::C);
            ret_cc(c == 0, ctx);
        }
        0xd1 => {
            let data = ctx.s_pop();
            ctx.registers.set_de(data);
        }
        0xd2 => {
            let c = ctx.registers.get_flag(Flags::C);
            jp_cc_nn(c == 0, ctx);
        }
        0xd4 => {
            let c = ctx.registers.get_flag(Flags::C);
            call_cc_nn(c == 0, ctx);
        }
        0xd5 => {
            let address = ctx.registers.get_de();
            ctx.take_cycle();
            ctx.s_push(address);
        }
        0xd6 => {
            let n = ctx.get_byte();
            ctx.registers.a = sub_a_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xd7 => {
            rst_n(0x0010, ctx);
        }
        0xd8 => {
            let c = ctx.registers.get_flag(Flags::C);
            ret_cc(c == 1, ctx);
        }
        0xd9 => {
            reti(ctx);
        }
        0xda => {
            let c = ctx.registers.get_flag(Flags::C);
            jp_cc_nn(c == 1, ctx);
        }
        0xdc => {
            let c = ctx.registers.get_flag(Flags::C);
            call_cc_nn(c == 1, ctx);
        }
        0xde => {
            let n = ctx.get_byte();
            ctx.registers.a = subc_a_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xdf => {
            rst_n(0x0018, ctx);
        }
        0xe0 => {
            let address = 0xff00 | ctx.get_byte() as u16;
            let a = ctx.registers.get_a();
            ctx.mem_write(address, a);
        }
        0xe1 => {
            let data = ctx.s_pop();
            ctx.registers.set_hl(data);
        }
        0xe2 => {
            let a = ctx.registers.get_a();
            let c = ctx.registers.get_c();
            ctx.mem_write(0xff00 | (c as u16), a);
        }
        0xe5 => {
            let address = ctx.registers.get_hl();
            ctx.take_cycle();
            ctx.s_push(address);
        }
        0xe6 => {
            let n = ctx.get_byte();
            ctx.registers.a = and_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xe7 => {
            rst_n(0x0020, ctx);
        }
        0xe8 => {
            let data = ctx.get_byte() as i8 as u16;
            let address = ctx.memory.get_sp();
            ctx.registers.set_flag(Flags::Z, false);
            ctx.registers.set_flag(Flags::N, false);
            ctx.registers
                .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
            ctx.registers
                .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
            let result = address.wrapping_add(data as u16);
            ctx.take_cycle();
            ctx.memory.set_sp(result);
            ctx.take_cycle();
        }
        0xe9 => {
            let address = ctx.registers.get_hl();
            ctx.memory.set_pc(address);
        }
        0xea => {
            let word = ctx.get_word();
            ctx.mem_write(word, ctx.registers.get_a());
        }
        0xee => {
            let n = ctx.get_byte();
            ctx.registers.a = xor_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xef => {
            rst_n(0x0028, ctx);
        }
        0xf0 => {
            let address = 0xff00 | ctx.get_byte() as u16;
            ctx.registers.a = ctx.mem_read(address);
        }
        0xf1 => {
            let data = ctx.s_pop();
            ctx.registers.set_af(data);
        }
        0xf2 => {
            let c = ctx.registers.get_c();
            let data = ctx.mem_read(0xff00 | c as u16);
            ctx.registers.set_a(data);
        }
        0xf3 => {
            ctx.timers.clear_ime();
        }
        0xf5 => {
            let address = ctx.registers.get_af();
            ctx.take_cycle();
            ctx.s_push(address);
        }
        0xf6 => {
            let n = ctx.get_byte();
            ctx.registers.a = or_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xf7 => {
            rst_n(0x0030, ctx);
        }
        0xf8 => {
            let data = ctx.get_byte() as i8 as u16;
            let address = ctx.memory.get_sp();
            ctx.registers
                .set_flag(Flags::H, (address & 0x0f) + (data & 0x0f) > 0x0f);
            ctx.registers
                .set_flag(Flags::C, (address & 0xff) + (data & 0xff) > 0xff);
            ctx.registers.set_flag(Flags::Z, false);
            ctx.registers.set_flag(Flags::N, false);
            let result = address.wrapping_add(data);
            ctx.take_cycle();
            ctx.registers.set_hl(result);
        }
        0xf9 => {
            let address = ctx.registers.get_hl();
            ctx.take_cycle(); // check if before or after SP
            ctx.memory.set_sp(address);
        }
        0xfa => {
            let word = ctx.get_word();
            let data = ctx.mem_read(word);
            ctx.registers.set_a(data);
        }
        0xfb => {
            ctx.interrupts.dispatch(Action::ime1);
        }
        0xfe => {
            let n = ctx.get_byte();
            cp_n(n, ctx.registers.a, &mut ctx.registers);
        }
        0xff => {
            rst_n(0x0038, ctx);
        }
        0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {
            unreachable!("Unexisting code {:X}", opcode)
        }
    };
}

pub fn update(emulator: &mut Emulator) {
    if !emulator.timers.is_halted {
        let opcode = emulator.fetch_opcode();
        emulator.take_cycle();
        return execute_opcode(emulator, opcode, false);
    }
    emulator.take_cycle();
}
