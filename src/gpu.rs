use super::clock::Clock;
use super::memory::Memory;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};

#[derive(PartialEq)]
enum LcdMode {
    HBlank,
    VBlank,
    ReadingOAMRAM,
    TransfToLCDDriver,
}

pub struct Gpu<'a> {
    clock: &'a mut Clock,
    memory: &'a mut Memory,
}

impl<'a> Gpu<'a> {
    pub fn new(clock: &'a mut Clock, memory: &'a mut Memory) -> Self {
        Self { clock, memory }
    }

    fn mem_write_scanline(&mut self, data: u8) {
        self.memory.write_scanline(data);
    }

    fn increment_scanline(&mut self) -> u8 {
        let mut scan_line = self.memory.read(0xff44);
        scan_line = scan_line.wrapping_add(1);
        self.memory.write_scanline(scan_line);
        scan_line
    }

    fn req_interrupt(&mut self, bit: u8) {
        let interrupt_flags = self.memory.read(0xff0f);
        let modified_flag = set_bit_at(interrupt_flags, bit);
        self.memory.write(0xff0f, modified_flag);
        self.clock.is_halted = false;
    }

    fn is_lcd_enabled(&self) -> bool {
        get_bit_at(self.memory.read(0xff40), 7)
    }

    fn get_lcd_status(&self) -> LcdMode {
        let lcd_status = self.memory.read(0xff41);
        match lcd_status & 0x3 {
            0x0 => LcdMode::HBlank,
            0x1 => LcdMode::VBlank,
            0x2 => LcdMode::ReadingOAMRAM,
            0x3 => LcdMode::TransfToLCDDriver,
            _ => panic!("Unreachable lcd status"),
        }
    }

    fn set_lcd_status(&mut self, status: LcdMode) {
        let lcd_status = self.memory.read(0xff41);
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
        self.memory.write(0xff41, new_status);
    }

    fn is_interrupt_requested(&self, bit: u8) -> bool {
        let lcd_status = self.memory.read(0xff41);
        get_bit_at(lcd_status, bit)
    }

    fn set_coincidence_flag(&mut self) {
        let lcd_status = self.memory.read(0xff41);
        self.memory.write(0xff41, set_bit_at(lcd_status, 2));
    }

    fn clear_coincidence_flag(&mut self) {
        let lcd_status = self.memory.read(0xff41);
        self.memory.write(0xff41, clear_bit_at(lcd_status, 2));
    }

    fn get_ly(&self) -> u8 {
        self.memory.read(0xff44)
    }

    fn get_lyc(&self) -> u8 {
        self.memory.read(0xff45)
    }

    fn set_lcd_mode(&mut self) {
        let current_line = self.get_ly();
        let current_mode = self.get_lcd_status();
        let mut req_int = false;

        let mode = if current_line >= 144 {
            self.set_lcd_status(LcdMode::VBlank);
            req_int = self.is_interrupt_requested(4);
            LcdMode::VBlank
        } else {
            match self.clock.scan_line_counter {
                0..=80 => {
                    self.set_lcd_status(LcdMode::ReadingOAMRAM);
                    req_int = self.is_interrupt_requested(5);
                    //println!("Next video mode in: {}", 80 - self.scan_line_counter);
                    LcdMode::ReadingOAMRAM
                }
                81..=252 => {
                    self.set_lcd_status(LcdMode::TransfToLCDDriver);
                    //println!("Next video mode in: {}", 252 - self.scan_line_counter);
                    LcdMode::TransfToLCDDriver
                }
                _ => {
                    self.set_lcd_status(LcdMode::HBlank);
                    req_int = self.is_interrupt_requested(3);
                    //println!("Next video mode in: {}", 456 - self.scan_line_counter);
                    LcdMode::HBlank
                }
            }
        };

        if req_int && (mode != current_mode) {
            self.req_interrupt(1);
        }
        if current_line == self.get_lyc() {
            self.set_coincidence_flag();
            if self.is_interrupt_requested(6) {
                self.req_interrupt(1);
            }
        } else {
            self.clear_coincidence_flag();
        }
    }

    fn draw_scan_line(&self) {}
}

pub fn update(clock: &mut Clock, memory: &mut Memory, frame_cycles: u32) {
    let mut gpu = Gpu::new(clock, memory);
    if !gpu.is_lcd_enabled() {
        gpu.clock.scan_line_counter = 0;
        gpu.mem_write_scanline(0);
        gpu.set_lcd_status(LcdMode::VBlank);
        return;
    }
    gpu.set_lcd_mode();

    gpu.clock.scan_line_counter += frame_cycles;
    if gpu.clock.scan_line_counter > 456 {
        let scan_line = gpu.increment_scanline();
        gpu.clock.scan_line_counter = 0;
        match scan_line {
            0..=143 => gpu.draw_scan_line(),
            144 => gpu.req_interrupt(0),
            145..=153 => {}
            154 => gpu.mem_write_scanline(0),
            _ => panic!("Unreachable, scanline can't be greater than 153"),
        }
    }
}
