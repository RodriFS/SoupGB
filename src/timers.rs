use super::constants::*;
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

    fn get_is_clock_enabled(&self) -> bool {
        let clock = self.memory.borrow().read(TIMER_CONTROL_ADDRESS);
        //clock & 0x3 == 1
        clock >> 2 == 1
    }

    fn get_clock_frequency(&self) -> u8 {
        self.memory.borrow().read(TIMER_MODULO_ADDRESS) & 0x3
    }
    fn reset_divider_counter(&mut self) {
        self.divider_counter = 0;
    }
    fn update_divider(&mut self) {
        let mut divider_register = self.memory.borrow().read(DIVIDER_COUNTER_ADDRESS);
        if divider_register == 0xff {
            divider_register = 0x0;
        } else {
            divider_register = self.memory.borrow().read(DIVIDER_COUNTER_ADDRESS) + 1;
        }
        self.memory
            .borrow_mut()
            .set_rom(DIVIDER_COUNTER_ADDRESS, divider_register);
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
    fn update_timer(&mut self) {
        let mut counter = self.memory.borrow().read(TIMER_COUNTER_ADDRESS);
        if counter == 0xff {
            counter = self.memory.borrow().read(TIMER_MODULO_ADDRESS);
            self.interrupts.borrow_mut().request_interrupt(2);
        } else {
            counter += 1;
        }
        self.memory
            .borrow_mut()
            .write(TIMER_COUNTER_ADDRESS, counter);
    }

    pub fn update(&mut self, frame_cycles: u32) {
        self.divider_counter += frame_cycles;
        if self.divider_counter > (CLOCKSPEED / self.divider_frequency) {
            self.print_debug_info();
            self.reset_divider_counter();
            self.update_divider();
        }

        if !self.get_is_clock_enabled() {
            return;
        }

        self.timer_counter += frame_cycles;
        if self.timer_counter > (CLOCKSPEED / self.clock_frequency) {
            self.print_debug_info();
            self.reset_timer_counter();
            self.update_timer();
        }
    }

    fn print_debug_info(&self) {
        if !DEBUG_TIMERS {
            return;
        }
        println!("TIMER: -----------------------------");
        println!("Clock frequency: {}", self.clock_frequency);
        println!("Divider frequency: {}", self.divider_frequency);
        println!("Timer enabled: {}", self.get_is_clock_enabled());
        println!(
            "0xff04 Divider counter: {}",
            self.memory.borrow().read(DIVIDER_COUNTER_ADDRESS)
        );
        println!(
            "0xff05 Timer counter: {}",
            self.memory.borrow().read(TIMER_COUNTER_ADDRESS)
        );
        println!(
            "0xff06 Timer modulo: {}",
            self.memory.borrow().read(TIMER_MODULO_ADDRESS)
        );
        println!(
            "0xff07 Timer control: {}",
            self.memory.borrow().read(TIMER_CONTROL_ADDRESS)
        );
    }
}
