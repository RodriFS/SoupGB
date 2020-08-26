use super::emulator::Emulator;
use super::utils::get_bit_at;
use super::utils::*;

#[derive(PartialEq, Debug)]
pub enum StatCond {
    HBLANK,
    VBlank,
    OAM,
    LYC,
    None,
}

impl StatCond {
    pub fn or(stat1: StatCond, stat2: StatCond) -> StatCond {
        if stat1 == StatCond::None {
            stat2
        } else {
            stat1
        }
    }

    pub fn is_stat(&self) -> bool {
        self != &StatCond::None
    }
}

pub fn update(ctx: &mut Emulator) {
    let i_f = ctx.memory.read(0xff0f);
    let i_e = ctx.memory.read(0xffff);
    if i_f > 0 {
        for bit in 0..5 {
            if get_bit_at(i_f, bit) && get_bit_at(i_e, bit) {
                if ctx.timers.is_halted && !ctx.timers.ime {
                    ctx.timers.halt_bug = true;
                }
                ctx.timers.is_halted = false;
                if ctx.timers.ime {
                    ctx.timers.clear_ime();
                    let cancelled = interrupt_execution(ctx, bit);
                    if !cancelled {
                        break;
                    }
                }
            }
        }
    }
}

pub fn interrupt_execution(ctx: &mut Emulator, interrupt: u8) -> bool {
    let pc = ctx.memory.get_pc();
    ctx.s_push_hi(pc);
    let interrupt_enable = ctx.memory.read(0xffff);
    if !get_bit_at(interrupt_enable, interrupt) {
        ctx.memory.set_pc(0);
        true
    } else {
        ctx.s_push_lo(pc);
        let i_f = ctx.memory.read(0xff0f);
        let clear_request = clear_bit_at(i_f, interrupt);
        ctx.memory.write(0xff0f, clear_request);
        let new_pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => return true,
        };
        ctx.memory.set_pc(new_pc as u16);
        false
    }
}

pub fn stat_irq(ctx: &Emulator, bit: u8) -> StatCond {
    let lcd_status = ctx.memory.read(0xff41);
    if get_bit_at(lcd_status, bit) {
        match bit {
            3 => StatCond::HBLANK,
            4 => StatCond::VBlank,
            5 => StatCond::OAM,
            6 => StatCond::LYC,
            _ => StatCond::None,
        }
    } else {
        StatCond::None
    }
}

pub fn request_interrupt(ctx: &mut Emulator, bit: u8) {
    let interrupt_flags = ctx.memory.read(0xff0f);
    let modified_flag = set_bit_at(interrupt_flags, bit);
    ctx.memory.write(0xff0f, modified_flag);
    ctx.timers.is_halted = false;
}
