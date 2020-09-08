use super::dispatcher::Action;
use super::emulator::Emulator;
use std::fmt;

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

pub fn update_tima(ctx: &mut Emulator) {
    let selected_bit = ctx.memory.get_div_counter() >> ctx.memory.tac_freq() & 0b1;
    let bit_enabled = selected_bit & ctx.memory.tac_enabled();
    if ((bit_enabled ^ 0b1) & ctx.memory.prev_timer_bit) == 1 {
        let new_tima = ctx.memory.get_tima().wrapping_add(1);
        if new_tima == 0 {
            ctx.dispatcher.dispatch(Action::reload_tima(true));
        }
        ctx.memory.set_tima(new_tima);
    }
    ctx.memory.prev_timer_bit = bit_enabled;
}

pub fn update(ctx: &mut Emulator) {
    update_div_counter(ctx);
    update_tima(ctx);
}

impl fmt::Debug for Timers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TIMERS ------------------------\n\
            MASTER ENABLED: {}\n\
            IS HALTED: {}\n",
            self.ime, self.is_halted
        )
    }
}
