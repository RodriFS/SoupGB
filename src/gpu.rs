use super::memory::{LcdMode, Memory};
use super::timers::Timers;
use super::utils::set_bit_at;

fn req_interrupt(memory: &mut Memory, timers: &mut Timers, bit: u8) {
    let interrupt_flags = memory.read(0xff0f);
    let modified_flag = set_bit_at(interrupt_flags, bit);
    memory.write(0xff0f, modified_flag);
    timers.is_halted = false;
}

fn set_lcd_mode(memory: &mut Memory, timers: &mut Timers) {
    let current_line = memory.get_ly();
    let current_mode = memory.get_lcd_status();
    let mut req_int = false;

    let mode = if current_line >= 144 {
        memory.set_lcd_status(LcdMode::VBlank);
        req_int = memory.is_interrupt_requested(4);
        LcdMode::VBlank
    } else {
        match timers.scan_line_counter {
            0..=80 => {
                memory.set_lcd_status(LcdMode::ReadingOAMRAM);
                req_int = memory.is_interrupt_requested(5);
                //println!("Next video mode in: {}", 80 - scan_line_counter);
                LcdMode::ReadingOAMRAM
            }
            81..=252 => {
                memory.set_lcd_status(LcdMode::TransfToLCDDriver);
                //println!("Next video mode in: {}", 252 - scan_line_counter);
                LcdMode::TransfToLCDDriver
            }
            _ => {
                memory.set_lcd_status(LcdMode::HBlank);
                req_int = memory.is_interrupt_requested(3);
                //println!("Next video mode in: {}", 456 - scan_line_counter);
                LcdMode::HBlank
            }
        }
    };

    if req_int && (mode != current_mode) {
        req_interrupt(memory, timers, 1);
    }
    if current_line == memory.get_lyc() {
        memory.set_coincidence_flag();
        if memory.is_interrupt_requested(6) {
            req_interrupt(memory, timers, 1);
        }
    } else {
        memory.clear_coincidence_flag();
    }
}

fn draw_scan_line() {}

pub fn update(timers: &mut Timers, memory: &mut Memory, frame_cycles: u32) {
    if !memory.is_lcd_enabled() {
        timers.scan_line_counter = 0;
        memory.write_scanline(0);
        memory.set_lcd_status(LcdMode::VBlank);
        return;
    }
    set_lcd_mode(memory, timers);

    timers.scan_line_counter += frame_cycles;
    if timers.scan_line_counter > 456 {
        let scan_line = memory.increment_scanline();
        timers.scan_line_counter = 0;
        match scan_line {
            0..=143 => draw_scan_line(),
            144 => req_interrupt(memory, timers, 0),
            145..=153 => {}
            154 => memory.write_scanline(0),
            _ => panic!("Unreachable, scanline can't be greater than 153"),
        }
    }
}
