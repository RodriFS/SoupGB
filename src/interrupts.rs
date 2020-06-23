use super::memory::Memory;
use super::timers::Timers;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};

pub struct Interrupts<'a> {
    timers: &'a mut Timers,
    memory: &'a mut Memory,
}

impl<'a> Interrupts<'a> {
    pub fn new(timers: &'a mut Timers, memory: &'a mut Memory) -> Self {
        Self { timers, memory }
    }

    pub fn request_interrupt(&mut self, bit: u8) {
        let interrupt_flags = self.memory.read(0xff0f);
        let modified_flag = set_bit_at(interrupt_flags, bit);
        self.memory.write(0xff0f, modified_flag);
        self.timers.is_halted = false;
    }

    fn interrupt_execution(&mut self, request: u8, interrupt: u8) {
        self.timers.master_enabled = false;
        let clear_request = clear_bit_at(request, interrupt);
        self.memory.write(0xff0f, clear_request);

        let pc = self.memory.get_program_counter();
        self.memory.push_to_stack(pc);

        let pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            4 => 0x60,
            _ => return,
        };
        self.memory.set_program_counter(pc);
    }
}

pub fn update(timers: &mut Timers, memory: &mut Memory) {
    let mut interrupts = Interrupts::new(timers, memory);
    if interrupts.timers.master_enabled {
        let request = interrupts.memory.read(0xff0f);
        let interrupt_enable = interrupts.memory.read(0xffff);
        if request > 0 {
            for bit in 0..5 {
                if get_bit_at(request, bit) && get_bit_at(interrupt_enable, bit) {
                    interrupts.interrupt_execution(request, bit);
                }
            }
        }
    }
}
