use super::memory::Memory;
use super::utils::set_bit_at;

pub struct Timers {
    pub divider_frequency: u32,
    pub timer_counter: u32,
    pub divider_counter: u32,
    pub scan_line_counter: u32,
    pub master_enabled: bool,
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
            is_halted: false,
        }
    }
    pub fn set_master_enabled_on(&mut self) {
        self.master_enabled = true;
    }
    pub fn clear_master_enabled(&mut self) {
        self.master_enabled = false;
    }
    fn reset_timer_counter(&mut self) {
        self.timer_counter = 0;
    }
}

fn request_interrupt(memory: &mut Memory, timers: &mut Timers, bit: u8) {
    let interrupt_flags = memory.read(0xff0f);
    let modified_flag = set_bit_at(interrupt_flags, bit);
    memory.write(0xff0f, modified_flag);
    timers.is_halted = false;
}

fn update_tima(memory: &mut Memory, timers: &mut Timers) {
    let (counter, overflow) = match memory.get_tima().checked_add(1) {
        Some(c) => (c, false),
        None => (memory.get_tma(), true),
    };

    if overflow {
        timers.reset_timer_counter();
        request_interrupt(memory, timers, 2);
    }
    memory.set_tima(counter);
}

pub fn update(timers: &mut Timers, memory: &mut Memory, opcode_cycles: u32) {
    timers.divider_counter += opcode_cycles;
    while timers.divider_counter >= 64 {
        timers.divider_counter -= 64;
        memory.update_div();
    }

    if !memory.get_is_clock_enabled() {
        return;
    }

    timers.timer_counter += opcode_cycles;
    let clock_freq = memory.input_clock_select;
    while timers.timer_counter >= clock_freq {
        timers.timer_counter -= clock_freq;
        update_tima(memory, timers);
    }
}
