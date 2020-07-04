use gba::constants::*;
use gba::emulator::Emulator;
use minifb::{Key, Scale, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread;

pub fn main() {
    let mut emulator = Emulator::default();
    let mut args: Vec<String> = std::env::args().collect();
    let file_path = args.pop().unwrap();
    let mut rom = File::open(file_path.clone()).unwrap();
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    emulator.load_rom(buffer);

    let (to_emu, from_window) = mpsc::channel();
    let (to_window, from_emu) = mpsc::channel();
    let handle = thread::spawn(move || emulator.run(to_window, from_window));

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
    let mut video_buffer = Vec::with_capacity(buf_len);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        match from_emu.recv() {
            Ok(line) => {
                video_buffer.extend(line);
                if video_buffer.len() == buf_len + SCREEN_WIDTH {
                    match window.update_with_buffer(&video_buffer, SCREEN_WIDTH, SCREEN_HEIGHT) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{}", e);
                            std::process::exit(0);
                        }
                    }
                    video_buffer.clear();
                }
            }
            Err(_) => {
                to_emu.send("close").unwrap();
            }
        }
    }
    to_emu.send("close").unwrap();
    handle.join().unwrap();
}
