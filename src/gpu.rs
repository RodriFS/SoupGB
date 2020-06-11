use super::interrupts::Interrupts;
use super::memory::Memory;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
enum LcdMode {
    HBlank,
    VBlank,
    ReadingOAMRAM,
    TransfToLCDDriver,
}

pub struct Gpu {
    scan_line_counter: u32,
    memory: Rc<RefCell<Memory>>,
    interrupts: Rc<RefCell<Interrupts>>,
}

impl Gpu {
    pub fn new(memory: Rc<RefCell<Memory>>, interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Self {
            scan_line_counter: 0,
            memory,
            interrupts,
        }
    }

    fn mem_read(&self, address: u16) -> u8 {
        self.memory.borrow().read(address)
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.memory.borrow_mut().write(address, data);
    }

    fn mem_write_scanline(&mut self, data: u8) {
        self.memory.borrow_mut().write_scanline(data);
    }

    fn increment_scanline(&mut self) -> u8 {
        let mut scan_line = self.mem_read(0xff44);
        scan_line = scan_line.wrapping_add(1);
        self.memory.borrow_mut().write_scanline(scan_line);
        scan_line
    }

    fn req_interrupt(&mut self, bit: u8) {
        self.interrupts.borrow_mut().request_interrupt(bit);
    }

    fn is_lcd_enabled(&self) -> bool {
        get_bit_at(self.mem_read(0xff40), 7)
    }

    fn get_lcd_status(&self) -> LcdMode {
        let lcd_status = self.mem_read(0xff41);
        match lcd_status & 0x3 {
            0x0 => LcdMode::HBlank,
            0x1 => LcdMode::VBlank,
            0x2 => LcdMode::ReadingOAMRAM,
            0x3 => LcdMode::TransfToLCDDriver,
            _ => panic!("Unreachable lcd status"),
        }
    }

    fn set_lcd_status(&mut self, status: LcdMode) {
        let lcd_status = self.mem_read(0xff41);
        let new_status = match status {
            LcdMode::HBlank => {
                let temp_status = clear_bit_at(lcd_status, 1);
                clear_bit_at(temp_status, 0)
            }
            LcdMode::VBlank => {
                let temp_status = clear_bit_at(lcd_status, 1);
                set_bit_at(temp_status, 0)
            }
            LcdMode::ReadingOAMRAM => {
                let temp_status = set_bit_at(lcd_status, 1);
                clear_bit_at(temp_status, 0)
            }
            LcdMode::TransfToLCDDriver => {
                let temp_status = set_bit_at(lcd_status, 1);
                set_bit_at(temp_status, 0)
            }
        };
        self.mem_write(0xff41, new_status);
    }

    fn is_interrupt_requested(&self, bit: u8) -> bool {
        let lcd_status = self.mem_read(0xff41);
        get_bit_at(lcd_status, bit)
    }

    fn set_coincidence_flag(&mut self) {
        let lcd_status = self.mem_read(0xff41);
        self.mem_write(0xff41, set_bit_at(lcd_status, 2));
    }

    fn clear_coincidence_flag(&mut self) {
        let lcd_status = self.mem_read(0xff41);
        self.mem_write(0xff41, clear_bit_at(lcd_status, 2));
    }

    fn set_lcd_mode(&mut self) {
        let current_line = self.mem_read(0xff44);
        let current_mode = self.get_lcd_status();
        let mut req_int = false;

        let mode = if current_line >= 144 {
            self.set_lcd_status(LcdMode::VBlank);
            req_int = self.is_interrupt_requested(4);
            LcdMode::VBlank
        } else {
            match self.scan_line_counter {
                0..=204 => {
                    self.set_lcd_status(LcdMode::ReadingOAMRAM);
                    req_int = self.is_interrupt_requested(5);
                    LcdMode::ReadingOAMRAM
                }
                205..=284 => {
                    self.set_lcd_status(LcdMode::TransfToLCDDriver);
                    LcdMode::TransfToLCDDriver
                }
                285..=456 => {
                    self.set_lcd_status(LcdMode::HBlank);
                    req_int = self.is_interrupt_requested(3);
                    LcdMode::HBlank
                }
                _ => panic!("Unreachable scanline counter"),
            }
        };

        // just entered a new mode so request interupt
        if req_int && (mode != current_mode) {
            self.req_interrupt(1);
        }
        // check the conincidence flag
        if current_line == self.mem_read(0xff45) {
            self.set_coincidence_flag();
            if self.is_interrupt_requested(6) {
                self.req_interrupt(1);
            }
        } else {
            self.clear_coincidence_flag();
        }
    }

    fn draw_scan_line(&self) {}

    pub fn update(&mut self, frame_cycles: u32) {
        self.set_lcd_mode();
        if !self.is_lcd_enabled() {
            self.scan_line_counter = 0;
            self.mem_write_scanline(0);
            self.set_lcd_status(LcdMode::VBlank);
            return;
        }

        self.scan_line_counter += frame_cycles;
        if self.scan_line_counter > 456 {
            let scan_line = self.increment_scanline();
            self.scan_line_counter = 0;
            match scan_line {
                0..=143 => self.draw_scan_line(),
                144 => self.req_interrupt(0),
                145..=153 => {}
                154 => self.mem_write_scanline(0),
                _ => panic!("Unreachable, scanline can't be greater than 153"),
            }
        }
    }
}
