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

pub fn update_div_counter(emu: &mut Emulator, data: u16) {
    let div_counter = emu.memory.get_div_counter();
    let result = div_counter.wrapping_add(data);
    emu.memory.set_div_counter(result);
}

pub fn cc_reload_tima(emu: &mut Emulator) {
    if emu.memory.sched_tima_increment {
        emu.memory.set_tima(emu.memory.get_tma());
        request_interrupt(emu, 2);
        emu.memory.sched_tima_increment = false;
    }
}
pub fn update_tima(emu: &mut Emulator) {
    let selected_bit = (emu.memory.get_div_counter() >> emu.memory.input_clock_select)
        & 0b1
        & emu.memory.is_clock_enabled();
    cc_reload_tima(emu);
    if !selected_bit & emu.memory.prev_bit == 1 {
        let new_tima = emu.memory.get_tima().wrapping_add(1);
        emu.memory.set_tima(new_tima);
        if new_tima == 0 {
            // emu.memory.set_div_counter(0);
            emu.memory.sched_tima_increment = true;
        }
    }
    emu.memory.prev_bit = selected_bit;
}

pub fn update(emu: &mut Emulator, opcode_cycles: u32) {
    if emu.timers.sched_master_enabled {
        emu.timers.master_enabled = true;
        emu.timers.sched_master_enabled = false;
    }
    update_div_counter(emu, opcode_cycles as u16);
    update_tima(emu);
}
