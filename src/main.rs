use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream};
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
use std::sync::mpsc::{channel, Receiver};
use std::time::Instant;

pub fn main() {
    let (tx, rx) = channel();
    let (_device, _stream, _sample_rate) = initialize_cpal(rx);
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

    emulator.memory.apu.load_sender(tx);

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

fn initialize_cpal(rx: Receiver<f32>) -> (Device, Stream, f32) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let supported_config = device
        .default_output_config()
        .expect("No default output config");

    let config = supported_config.config();
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let sample = rx.try_recv().unwrap_or(0.0);
            write_data(data, channels, sample)
        },
        move |err| {
            eprintln!("an error occurred on the output audio stream: {}", err);
        },
    );
    let stream = stream.unwrap();
    stream.play().unwrap();
    let sample_rate = config.sample_rate.0 as f32;
    (device, stream, sample_rate)
}

fn write_data<T>(output: &mut [T], channels: usize, sample: f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value = cpal::Sample::from::<f32>(&sample);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
