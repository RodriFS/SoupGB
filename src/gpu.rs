use super::constants::*;
use super::memory::{LcdMode, Memory, Point2D};
use super::timers::Timers;
use super::utils::{get_bit_at, set_bit_at};
use std::sync::mpsc::Sender;

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
                LcdMode::ReadingOAMRAM
            }
            81..=252 => {
                memory.set_lcd_status(LcdMode::TransfToLCDDriver);
                LcdMode::TransfToLCDDriver
            }
            _ => {
                memory.set_lcd_status(LcdMode::HBlank);
                req_int = memory.is_interrupt_requested(3);
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

fn get_color(pixel: u8, palette: u8) -> u32 {
    let color = match pixel {
        0x00 => palette & 0b0000_0011,
        0x01 => (palette & 0b0000_1100) >> 2,
        0x02 => (palette & 0b0011_0000) >> 4,
        0x03 => (palette & 0b1100_0000) >> 6,
        _ => panic!("Unreachable"),
    };
    match color {
        0x00 => 0xff_ff_ff,
        0x01 => 0xea_ec_ee,
        0x02 => 0x56_65_73,
        0x03 => 0x00_00_00,
        _ => panic!("Unreachable"),
    }
}

fn into_pixels(buffer: &mut Vec<u32>, data1: u8, data2: u8, palette: u8) -> &mut Vec<u32> {
    let p1 = ((get_bit_at(data2, 7) as u8) << 1) | (get_bit_at(data1, 7) as u8);
    let p2 = ((get_bit_at(data2, 6) as u8) << 1) | (get_bit_at(data1, 6) as u8);
    let p3 = ((get_bit_at(data2, 5) as u8) << 1) | (get_bit_at(data1, 5) as u8);
    let p4 = ((get_bit_at(data2, 4) as u8) << 1) | (get_bit_at(data1, 4) as u8);
    let p5 = ((get_bit_at(data2, 3) as u8) << 1) | (get_bit_at(data1, 3) as u8);
    let p6 = ((get_bit_at(data2, 2) as u8) << 1) | (get_bit_at(data1, 2) as u8);
    let p7 = ((get_bit_at(data2, 1) as u8) << 1) | (get_bit_at(data1, 1) as u8);
    let p8 = ((get_bit_at(data2, 0) as u8) << 1) | (get_bit_at(data1, 0) as u8);
    buffer.push(get_color(p1, palette));
    buffer.push(get_color(p2, palette));
    buffer.push(get_color(p3, palette));
    buffer.push(get_color(p4, palette));
    buffer.push(get_color(p5, palette));
    buffer.push(get_color(p6, palette));
    buffer.push(get_color(p7, palette));
    buffer.push(get_color(p8, palette));
    buffer
}

fn render_background(provider: &Sender<Vec<u32>>, memory: &mut Memory) {
    let Point2D { x: sx, y: sy } = memory.background_position();
    let Point2D { x: _wx, y: _wy } = memory.window_position();
    let bg_tilemap_region = memory.background_map_select();
    let tile_data_region = memory.tile_data_select();
    //let window_enabled = memory.window_enabled();
    let palette = memory.background_palette();
    let current_line = memory.get_ly();
    let mut buffer = Vec::with_capacity(SCREEN_WIDTH);
    let visible_tiles = SCREEN_WIDTH as u16 / 8;
    let from = bg_tilemap_region + (current_line as u16 / 8) * visible_tiles;
    for bg_mem in from..(from + visible_tiles) {
        let address = memory.read(bg_mem);
        let tile_id = match tile_data_region {
            0x8000 => address as u16 * 16,
            0x8800 => (address as i16 + 128) as u16 * 16,
            _ => panic!("Unreachable"),
        };
        let pixel_row = (current_line % 8 * 2) as u16;
        let data1 = memory.read(pixel_row + tile_data_region + tile_id);
        let data2 = memory.read(pixel_row + tile_data_region + tile_id + 1);
        into_pixels(&mut buffer, data1, data2, palette);
    }
    for (i, pixel) in buffer.into_iter().enumerate() {
        let h_pos = (sx).wrapping_add(i as u8) as u16;
        let v_pos = (256 - sy as u16).wrapping_mul(256);
        let line = (current_line as u16).wrapping_mul(256 as u16);
        let pos = h_pos.wrapping_add(v_pos).wrapping_add(line) as usize;
        if pos < memory.video_buffer.len() {
            memory.video_buffer[pos] = pixel;
        }
    }
    if current_line == 143 {
        provider.send(memory.video_buffer.clone()).unwrap();
    }
}

fn draw_scan_line(provider: &Sender<Vec<u32>>, memory: &mut Memory) {
    if memory.background_enabled() {
        render_background(provider, memory);
    }
    if memory.sprite_enabled() {}
}

pub fn update(
    provider: &Sender<Vec<u32>>,
    timers: &mut Timers,
    memory: &mut Memory,
    frame_cycles: u32,
) {
    if !memory.is_lcd_enabled() {
        timers.scan_line_counter = 0;
        memory.write_scanline(0);
        memory.set_lcd_status(LcdMode::VBlank);
        return;
    }
    set_lcd_mode(memory, timers);

    timers.scan_line_counter += frame_cycles;
    if timers.scan_line_counter > 456 {
        let scan_line = memory.get_ly();
        timers.scan_line_counter = 0;
        match scan_line {
            0..=143 => draw_scan_line(provider, memory),
            144 => {
                draw_scan_line(provider, memory);
                req_interrupt(memory, timers, 0)
            }
            145..=153 => {}
            154 => {
                memory.write_scanline(0);
                return;
            }
            _ => panic!("Unreachable, scanline can't be greater than 153"),
        }
        memory.increment_scanline();
    }
}
