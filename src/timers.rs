use super::emulator::Emulator;
use super::interrupts::request_interrupt;

pub struct Timers {
    pub divider_frequency: u32,
    pub scan_line_counter: u32,
    pub master_enabled: bool,
    pub sched_master_enabled: bool,
    pub is_halted: bool,
}

impl Timers {
    pub fn default() -> Self {
        let divider_frequency = 16384;
        Self {
            scan_line_counter: 0,
            divider_frequency,
            master_enabled: false,
            sched_master_enabled: false,
            is_halted: false,
        }
    }
    pub fn set_master_enabled_on(&mut self) {
        self.sched_master_enabled = true;
    }
    pub fn clear_master_enabled(&mut self) {
        self.master_enabled = false;
        self.sched_master_enabled = false;
    }
}

pub fn update_div_counter(ctx: &mut Emulator, cycles: u16) {
    let div_counter = ctx.memory.get_div_counter();
    let result = div_counter.wrapping_add(cycles);
    ctx.memory.set_div_counter(result);
}

pub fn cc_tima_reload(ctx: &mut Emulator) {
    if ctx.memory.sched_tima_reload {
        ctx.memory.set_tima(ctx.memory.get_tma());
        request_interrupt(ctx, 2);
        ctx.memory.sched_tima_reload = false;
    }
}

pub fn update_tima(ctx: &mut Emulator) {
    cc_tima_reload(ctx);
    let selected_bit =
        (ctx.memory.get_div_counter() >> ctx.memory.tac_freq()) & 0b1 & ctx.memory.tac_enabled();
    if !selected_bit & ctx.memory.prev_bit == 1 {
        let new_tima = ctx.memory.get_tima().wrapping_add(1);
        ctx.memory.set_tima(new_tima);
        if new_tima == 0 {
            ctx.memory.sched_tima_reload = true;
        }
    }
    ctx.memory.prev_bit = selected_bit;
}

pub fn update(ctx: &mut Emulator, opcode_cycles: u32) {
    if ctx.timers.sched_master_enabled {
        ctx.timers.master_enabled = true;
        ctx.timers.sched_master_enabled = false;
    }
    update_div_counter(ctx, opcode_cycles as u16);
    update_tima(ctx);
}
