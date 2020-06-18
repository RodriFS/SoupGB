use super::memory::Memory;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interrupts {
    memory: Rc<RefCell<Memory>>,
    master_enabled: bool,
}

impl Interrupts {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Self {
        Self {
            memory,
            master_enabled: false,
        }
    }

    pub fn set_master_enabled_on(&mut self) {
        self.master_enabled = true;
    }
    pub fn clear_master_enabled(&mut self) {
        self.master_enabled = false;
    }

    pub fn request_interrupt(&self, bit: u8) {
        let interrupt_flags = self.memory.borrow().read(0xff0f);
        let modified_flag = set_bit_at(interrupt_flags, bit);
        self.memory.borrow_mut().write(0xff0f, modified_flag)
    }

    fn interrupt_execution(&mut self, request: u8, interrupt: u8) {
        self.master_enabled = false;
        let clear_request = clear_bit_at(request, interrupt);
        self.memory.borrow_mut().write(0xff0f, clear_request);

        let pc = self.memory.borrow().get_program_counter();
        self.memory
            .borrow_mut()
            .push_to_stack(pc);

        let pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            4 => 0x60,
            _ => return,
        };
        self.memory.borrow_mut().set_program_counter(pc);
    }

    pub fn update(&mut self) {
        if self.master_enabled {
            let request = self.memory.borrow().read(0xff0f);
            let interrupt_enable = self.memory.borrow().read(0xffff);
            if request > 0 {
                for bit in 0..5 {
                    if get_bit_at(request, bit) && get_bit_at(interrupt_enable, bit) {
                        self.interrupt_execution(request, bit);
                    }
                }
            }
        }
    }
}
