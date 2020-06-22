use super::clock::Clock;
use super::constants::*;
use super::debugger::print_debug_timers_info;
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

    fn get_div(&self) -> u8 {
        self.memory.read(DIVIDER_COUNTER_ADDRESS)
    }
    fn set_div(&mut self, data: u8) {
        self.memory.set_rom(DIVIDER_COUNTER_ADDRESS, data);
    }
    fn get_tima(&self) -> u8 {
        self.memory.read(TIMER_COUNTER_ADDRESS)
    }
    fn set_tima(&mut self, counter: u8) {
        self.memory.write(TIMER_COUNTER_ADDRESS, counter);
    }
    fn get_tma(&self) -> u8 {
        self.memory.read(TIMER_MODULO_ADDRESS)
    }
    fn get_tac(&self) -> u8 {
        self.memory.read(TIMER_CONTROL_ADDRESS)
    }
    fn get_is_clock_enabled(&self) -> bool {
        let clock = self.get_tac();
        clock >> 2 == 1
    }
    fn get_clock_frequency(&self) -> u8 {
        self.get_tac() & 0x3
    }
    fn reset_divider_counter(&mut self) {
        self.clock.divider_counter = 0;
    }
    fn update_div(&mut self) {
        let divider_register = self.get_div();
        if divider_register == 0xff {
            self.set_div(0x0);
        } else {
            self.set_div(divider_register + 1);
        }
    }
    fn reset_timer_counter(&mut self) {
        match self.get_clock_frequency() {
            0 => self.clock.timer_counter = 1024, // freq 4096
            1 => self.clock.timer_counter = 16,   // freq 262144
            2 => self.clock.timer_counter = 64,   // freq 65536
            3 => self.clock.timer_counter = 256,  // freq 16382
            _ => panic!("Frequency not supported"),
        }
    }

    pub fn request_interrupt(&mut self, bit: u8) {
        let interrupt_flags = self.memory.read(0xff0f);
        let modified_flag = set_bit_at(interrupt_flags, bit);
        self.memory.write(0xff0f, modified_flag);
        self.clock.is_halted = false;
    }

    fn update_tima(&mut self) {
        let mut counter = self.get_tima();
        if counter == 0xff {
            counter = self.get_tma();
            self.request_interrupt(2);
        } else {
            counter += 1;
        }
        self.set_tima(counter);
    }
}

pub fn update(clock: &mut Clock, memory: &mut Memory, opcode_cycles: u32) {
    let mut timers = Timers::new(clock, memory);
    let mut print_debug = false;
    timers.clock.divider_counter += opcode_cycles;
    if timers.clock.divider_counter > (CLOCKSPEED / timers.clock.divider_frequency) {
        print_debug = true;
        timers.reset_divider_counter();
        timers.update_div();
    }

    if timers.get_is_clock_enabled() {
        timers.clock.timer_counter += opcode_cycles;
        if timers.clock.timer_counter > (CLOCKSPEED / timers.clock.clock_frequency) {
            print_debug = true;
            timers.reset_timer_counter();
            timers.update_tima();
        }
    }
    if print_debug {
        print_debug_timers_info(
            timers.get_clock_frequency(),
            timers.clock.divider_frequency,
            timers.get_is_clock_enabled(),
            timers.get_div(),
            timers.get_tima(),
            timers.get_tma(),
            timers.get_tac(),
        );
    }
}
