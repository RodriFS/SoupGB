use gba::constants::*;
use gba::cpu;
use gba::debugger::print_debug;
use gba::emulator::Emulator;
use gba::interrupts;
use minifb::{Key, Scale, Window, WindowOptions};
use std::fs::File;
use std::io::Read;

pub fn main() {
    let mut emulator = Emulator::default();
    let mut args: Vec<String> = std::env::args().collect();
    let file_path = args.pop().unwrap();
    let mut rom = File::open(file_path.clone()).unwrap();
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    emulator.load_rom(buffer);

    let windows_options = WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    };

    let mut window = Window::new(&file_path, SCREEN_WIDTH, SCREEN_HEIGHT, windows_options)
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let buf_len = SCREEN_WIDTH * SCREEN_HEIGHT;
    emulator.debug();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        print_debug(
            emulator.debug,
            &emulator.memory,
            &emulator.timers,
            &emulator.registers,
        );
        interrupts::update(&mut emulator);
        cpu::update(&mut emulator);
        if emulator.frame_buffer.len() == buf_len {
            // if emulator.memory.get_ly() == 143 && emulator.memory.get_lcd_status() == LcdMode::HBlank {
            match window.update_with_buffer(&emulator.frame_buffer, SCREEN_WIDTH, SCREEN_HEIGHT) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(0);
                }
            }
            emulator.frame_buffer.clear();
        }
    }
}
