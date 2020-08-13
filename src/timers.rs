use super::emulator::Emulator;
use super::interrupts::request_interrupt;

pub struct Timers {
    pub divider_frequency: u32,
    pub timer_counter: u32,
    pub divider_counter: u32,
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
            timer_counter: 0,
            divider_counter: 0,
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
    fn reset_timer_counter(&mut self) {
        self.timer_counter = 0;
    }
}

fn update_tima(emu: &mut Emulator) {
    let (counter, overflow) = match emu.memory.get_tima().checked_add(1) {
        Some(c) => (c, false),
        None => (emu.memory.get_tma(), true),
    };

    if overflow {
        emu.timers.reset_timer_counter();
        request_interrupt(emu, 2);
    }
    emu.memory.set_tima(counter);
}

pub fn update(emu: &mut Emulator, opcode_cycles: u32) {
    if emu.timers.sched_master_enabled {
        emu.timers.master_enabled = true;
        emu.timers.sched_master_enabled = false;
    }
    emu.timers.divider_counter += opcode_cycles;
    while emu.timers.divider_counter >= 64 {
        emu.timers.divider_counter -= 64;
        emu.memory.update_div();
    }

    if !emu.memory.get_is_clock_enabled() {
        return;
    }

    emu.timers.timer_counter += opcode_cycles;
    let clock_freq = emu.memory.input_clock_select;
    if emu.timers.timer_counter >= clock_freq {
        emu.timers.timer_counter -= clock_freq;
        update_tima(emu);
    }
}
