use super::clock::Clock;
use super::memory::Memory;
use super::utils::set_bit_at;

pub struct Timers<'a> {
    memory: &'a mut Memory,
    clock: &'a mut Clock,
}

impl<'a> Timers<'a> {
    pub fn new(clock: &'a mut Clock, memory: &'a mut Memory) -> Self {
        Self { memory, clock }
    }

    fn reset_divider_counter(&mut self) {
        self.clock.divider_counter = 0;
    }
    fn update_div(&mut self) {
        let divider_register = self.memory.get_div();
        if divider_register == 0xff {
            self.memory.set_div(0x0);
        } else {
            self.memory.set_div(divider_register + 1);
        }
    }
    fn reset_timer_counter(&mut self) {
        self.clock.timer_counter = 0;
    }

    pub fn request_interrupt(&mut self, bit: u8) {
        let interrupt_flags = self.memory.read(0xff0f);
        let modified_flag = set_bit_at(interrupt_flags, bit);
        self.memory.write(0xff0f, modified_flag);
        self.clock.is_halted = false;
    }

    fn update_tima(&mut self) {
        let counter = self.memory.get_tima().wrapping_add(1);
        if counter == 0x00 {
            self.reset_timer_counter();
            self.request_interrupt(2);
        }
        self.memory.set_tima(counter);
    }
}

pub fn update(clock: &mut Clock, memory: &mut Memory, opcode_cycles: u32) {
    let mut timers = Timers::new(clock, memory);
    timers.clock.divider_counter += opcode_cycles;
    if timers.clock.divider_counter > timers.clock.divider_frequency {
        timers.reset_divider_counter();
        timers.update_div();
    }

    if timers.memory.get_is_clock_enabled() {
        timers.clock.timer_counter -= opcode_cycles as i32;
        while timers.clock.timer_counter <= 0 {
            timers.clock.timer_counter += timers.memory.input_clock_select as i32;
            timers.update_tima();
        }
    }
}
