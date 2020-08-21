use super::emulator::Emulator;
use super::utils::get_bit_at;
use super::utils::*;

pub fn update(ctx: &mut Emulator) {
    let request = ctx.memory.read(0xff0f);
    let interrupt_enable = ctx.memory.read(0xffff);
    if request > 0 {
        for bit in 0..5 {
            if get_bit_at(request, bit) && get_bit_at(interrupt_enable, bit) {
                ctx.timers.is_halted = false;
                if ctx.timers.master_enabled {
                    ctx.timers.clear_master_enabled();
                    let cancelled = interrupt_execution(ctx, request, bit);
                    if !cancelled {
                        break;
                    }
                }
            }
        }
    }
}

pub fn stat_irq(ctx: &Emulator, bit: u8) -> bool {
    let lcd_status = ctx.memory.read(0xff41);
    get_bit_at(lcd_status, bit)
}

pub fn interrupt_execution(ctx: &mut Emulator, request: u8, interrupt: u8) -> bool {
    let pc = ctx.memory.get_program_counter();
    ctx.push_to_stack_hi(pc);
    let interrupt_enable = ctx.mem_read(0xffff);
    if !get_bit_at(interrupt_enable, interrupt) {
        ctx.take_cycle();
        ctx.memory.set_program_counter(0);
        true
    } else {
        ctx.push_to_stack_lo(pc);
        let clear_request = clear_bit_at(request, interrupt);
        ctx.mem_write(0xff0f, clear_request);
        let new_pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => return true,
        };
        ctx.take_cycle();
        ctx.memory.set_program_counter(new_pc as u16);
        false
    }
}

pub fn request_interrupt(ctx: &mut Emulator, bit: u8) {
    let interrupt_flags = ctx.memory.read(0xff0f);
    let modified_flag = set_bit_at(interrupt_flags, bit);
    ctx.memory.write(0xff0f, modified_flag);
    ctx.timers.is_halted = false;
}
