use super::constants::*;
use super::debugger::print_debug_timers_info;
use super::interrupts::Interrupts;
use super::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Timers {
    memory: Rc<RefCell<Memory>>,
    interrupts: Rc<RefCell<Interrupts>>,
    clock_frequency: u32,
    divider_frequency: u32,
    timer_counter: u32,
    divider_counter: u32,
}

impl Timers {
    pub fn new(memory: Rc<RefCell<Memory>>, interrupts: Rc<RefCell<Interrupts>>) -> Self {
        let clock_frequency = 4096;
        let divider_frequency = 16384;
        Self {
            memory,
            interrupts,
            clock_frequency,
            divider_frequency,
            timer_counter: 0,
            divider_counter: 0,
        }
    }

    fn get_div(&self) -> u8 {
        self.memory.borrow().read(DIVIDER_COUNTER_ADDRESS)
    }
    fn set_div(&mut self, data: u8) {
        self.memory
            .borrow_mut()
            .set_rom(DIVIDER_COUNTER_ADDRESS, data);
    }
    fn get_tima(&self) -> u8 {
        self.memory.borrow().read(TIMER_COUNTER_ADDRESS)
    }
    fn set_tima(&self, counter: u8) {
        self.memory
            .borrow_mut()
            .write(TIMER_COUNTER_ADDRESS, counter);
    }
    fn get_tma(&self) -> u8 {
        self.memory.borrow().read(TIMER_MODULO_ADDRESS)
    }
    fn get_tac(&self) -> u8 {
        self.memory.borrow().read(TIMER_CONTROL_ADDRESS)
    }
    fn get_is_clock_enabled(&self) -> bool {
        let clock = self.get_tac();
        clock >> 2 == 1
    }
    fn get_clock_frequency(&self) -> u8 {
        self.get_tac() & 0x3
    }
    fn reset_divider_counter(&mut self) {
        self.divider_counter = 0;
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
            0 => self.timer_counter = 1024, // freq 4096
            1 => self.timer_counter = 16,   // freq 262144
            2 => self.timer_counter = 64,   // freq 65536
            3 => self.timer_counter = 256,  // freq 16382
            _ => panic!("Frequency not supported"),
        }
    }
    fn update_tima(&mut self) {
        let mut counter = self.get_tima();
        if counter == 0xff {
            counter = self.get_tma();
            self.interrupts.borrow_mut().request_interrupt(2);
        } else {
            counter += 1;
        }
        self.set_tima(counter);
    }
    pub fn update(&mut self, opcode_cycles: u32) {
        let mut print_debug = false;
        self.divider_counter += opcode_cycles;
        if self.divider_counter > (CLOCKSPEED / self.divider_frequency) {
            print_debug = true;
            self.reset_divider_counter();
            self.update_div();
        }

        if self.get_is_clock_enabled() {
            self.timer_counter += opcode_cycles;
            if self.timer_counter > (CLOCKSPEED / self.clock_frequency) {
                print_debug = true;
                self.reset_timer_counter();
                self.update_tima();
            }
        }
        if print_debug {
            print_debug_timers_info(
                self.get_clock_frequency(),
                self.divider_frequency,
                self.get_is_clock_enabled(),
                self.get_div(),
                self.get_tima(),
                self.get_tma(),
                self.get_tac(),
            );
        }
    }
}
