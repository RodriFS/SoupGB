use super::constants::*;
use super::memory::{LcdMode, Memory, Point2D};
use super::timers::Timers;
use super::utils::{get_bit_at, set_bit_at};
use std::sync::mpsc::Sender;

struct Gpu<'a> {
    memory: &'a mut Memory,
    timers: &'a mut Timers,
}

impl<'a> Gpu<'a> {
    fn new(memory: &'a mut Memory, timers: &'a mut Timers) -> Self {
        Self { memory, timers }
    }

    fn req_interrupt(&mut self, bit: u8) {
        let interrupt_flags = self.memory.read(0xff0f);
        let modified_flag = set_bit_at(interrupt_flags, bit);
        self.memory.write(0xff0f, modified_flag);
        self.timers.is_halted = false;
    }

    fn set_lcd_mode(&mut self) {
        let current_line = self.memory.get_ly();
        let current_mode = self.memory.get_lcd_status();
        let mut req_int = false;
        let mode = if current_line >= 144 {
            self.memory.set_lcd_status(LcdMode::VBlank);
            req_int = self.memory.is_interrupt_requested(4);
            LcdMode::VBlank
        } else {
            match self.timers.scan_line_counter {
                0..=80 => {
                    self.memory.set_lcd_status(LcdMode::ReadingOAMRAM);
                    req_int = self.memory.is_interrupt_requested(5);
                    LcdMode::ReadingOAMRAM
                }
                81..=252 => {
                    self.memory.set_lcd_status(LcdMode::TransfToLCDDriver);
                    LcdMode::TransfToLCDDriver
                }
                _ => {
                    self.memory.set_lcd_status(LcdMode::HBlank);
                    req_int = self.memory.is_interrupt_requested(3);
                    LcdMode::HBlank
                }
            }
        };
        if req_int && (mode != current_mode) {
            self.req_interrupt(1);
        }
        if current_line == self.memory.get_lyc() {
            self.memory.set_coincidence_flag();
            if self.memory.is_interrupt_requested(6) {
                self.req_interrupt(1);
            }
        } else {
            self.memory.clear_coincidence_flag();
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

    fn make_pixels(&self, data1: u8, data2: u8) -> Vec<u32> {
        let palette = self.memory.background_palette();
        let hi_byte = (0..8).rev().map(|i| get_bit_at(data2, i) as u8);
        let low_byte = (0..8).rev().map(|i| get_bit_at(data1, i) as u8);
        hi_byte
            .zip(low_byte)
            .map(|(hi, lo)| Gpu::get_color(hi << 1 | lo, palette))
            .collect()
    }

    fn get_tile_ids(&self, bg_mem: u16) -> (u16, u16) {
        let tiledata_region = self.memory.bg_tile_data_select();
        let data = self.memory.read(bg_mem);
        let tile_id = match tiledata_region {
            0x8000 => data as u16 * 16,
            0x8800 => ((data as i8) as u16).wrapping_add(128) * 16,
            _ => panic!("Unreachable"),
        };
        (tiledata_region + tile_id, tiledata_region + tile_id + 1)
    }

    fn get_x_pos(&self, window_enabled: bool, sx: u8, wx: u8, pixel_pos: u8) -> u8 {
        if window_enabled && pixel_pos >= wx {
            pixel_pos.wrapping_add(wx)
        } else {
            pixel_pos.wrapping_sub(sx)
        }
    }

    fn get_y_pos(&self, window_enabled: bool, sy: u8, wy: u8) -> u16 {
        if window_enabled {
            wy as u16
        } else {
            (sy as u16 % 8) * 16
        }
    }

    fn render_background(&mut self, buffer: &mut Vec<u32>) {
        let bg_map = self.memory.map_select();
        let Point2D { x: sx, y: sy } = self.memory.background_position();
        let Point2D { x: wx, y: wy } = self.memory.window_position();
        let current_line = self.memory.get_ly();
        let window_enabled = self.memory.window_enabled() && wy <= current_line;
        let visible_tiles = 256 as u16 / 8;
        let from = bg_map + (current_line as u16 / 8) * 32;
        let pixel_row = current_line % 8 * 2;
        for (tile_pos, bg_mem) in (from..(from + visible_tiles)).enumerate() {
            let _y_pos = self.get_y_pos(window_enabled, sy, wy);
            let (tile1, tile2) = self.get_tile_ids(bg_mem); //+ y_pos
            let data1 = self.memory.read(pixel_row as u16 + tile1);
            let data2 = self.memory.read(pixel_row as u16 + tile2);
            let pixels = self.make_pixels(data1, data2);
            pixels.iter().enumerate().for_each(|(i, pixel)| {
                //let base = current_line as usize * SCREEN_WIDTH;
                let pixel_pos = (tile_pos * 8) + i;
                let x_pos = self.get_x_pos(window_enabled, sx, wx, pixel_pos as u8) as usize;
                if (x_pos) < SCREEN_WIDTH {
                    buffer.push(*pixel);
                }
            });
        }
    }

    // fn render_sprites(&mut self) {
    //     let size = self.memory.sprite_size();
    //     for (sprite_pos) in (0..40) {}
    // }

    fn draw_scan_line(&mut self, provider: &Sender<Vec<u32>>) {
        let mut buffer: Vec<u32> = Vec::with_capacity(SCREEN_WIDTH);
        if self.memory.background_enabled() {
            self.render_background(&mut buffer);
        }
        if self.memory.sprite_enabled() {
            //   self.render_sprites();
        }
        provider.send(buffer).unwrap();
    }
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
    let mut gpu = Gpu::new(memory, timers);
    gpu.set_lcd_mode();

    gpu.timers.scan_line_counter += frame_cycles;
    if gpu.timers.scan_line_counter > 456 {
        let scan_line = gpu.memory.get_ly();
        gpu.timers.scan_line_counter = 0;
        match scan_line {
            0..=143 => gpu.draw_scan_line(provider),
            144 => {
                gpu.draw_scan_line(provider);
                gpu.req_interrupt(0)
            }
            145..=153 => {}
            154 => {
                memory.write_scanline(0);
                return;
            }
            _ => panic!(
                "Unreachable, scanline can't be greater than 153: {}",
                scan_line
            ),
        }
        memory.increment_scanline();
    }
}
