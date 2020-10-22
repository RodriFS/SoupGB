use minifb::{Key, Scale, Window, WindowOptions};
use soup_gb::constants::*;
use soup_gb::cpu;
use soup_gb::debugger::print_debug;
use soup_gb::emulator::Emulator;
use soup_gb::interrupts;
use soup_gb::joypad;
use soup_gb::memory::LcdMode;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

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
    let mut frame_time = Instant::now();
    let mut frame_counter = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        interrupts::update(&mut emulator);
        print_debug(
            emulator.debug,
            &emulator.memory,
            &emulator.timers,
            &emulator.registers,
        );
        cpu::update(&mut emulator);
        joypad::update(&mut emulator, &window);
        if emulator.memory.get_ly() == 0x90 && emulator.memory.lcd_mode() == LcdMode::HBlank {
            match window.update_with_buffer(&emulator.frame_buffer, SCREEN_WIDTH, SCREEN_HEIGHT) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(0);
                }
            }
            if frame_time.elapsed().as_millis() >= 1000 {
                window.set_title(&format!("FPS: {}", frame_counter));
                frame_time = Instant::now();
                frame_counter = 0;
            }
            frame_counter += 1
        }
    }
}
