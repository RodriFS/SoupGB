use super::emulator::Emulator;
use super::interrupts::request_interrupt;

pub struct Timers {
    pub divider_frequency: u32,
    pub scan_line_counter: u32,
    pub ime: bool,
    pub is_halted: bool,
    pub halt_bug: bool,
}

impl Timers {
    pub fn default() -> Self {
        let divider_frequency = 16384;
        Self {
            scan_line_counter: 0,
            divider_frequency,
            ime: false,
            is_halted: false,
            halt_bug: false,
        }
    }
    pub fn clear_ime(&mut self) {
        self.ime = false;
    }
}

pub fn update_div_counter(ctx: &mut Emulator) {
    let div_counter = ctx.memory.get_div_counter();
    let result = div_counter.wrapping_add(4);
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

pub fn update(ctx: &mut Emulator) {
    update_div_counter(ctx);
    update_tima(ctx);
}
