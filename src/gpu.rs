use super::constants::*;
use super::emulator::Emulator;
use super::memory::{LcdMode, Point2D};
use super::utils::{get_bit_at, set_bit_at};

struct Gpu<'a> {
    emu: &'a mut Emulator,
}

impl<'a> Gpu<'a> {
    fn new(emu: &'a mut Emulator) -> Self {
        Self { emu }
    }

    fn req_interrupt(&mut self, bit: u8) {
        let interrupt_flags = self.emu.memory.read(0xff0f);
        let modified_flag = set_bit_at(interrupt_flags, bit);
        self.emu.memory.write(0xff0f, modified_flag);
        self.emu.timers.is_halted = false;
    }

    fn set_lcd_mode(&mut self) {
        let current_line = self.emu.memory.get_ly();
        let current_mode = self.emu.memory.get_lcd_status();
        let mut req_int = false;
        let mode = if current_line >= 144 {
            self.emu.memory.set_lcd_status(LcdMode::VBlank);
            req_int = self.emu.memory.is_interrupt_requested(4);
            LcdMode::VBlank
        } else {
            match self.emu.timers.scan_line_counter {
                0..=80 => {
                    self.emu.memory.set_lcd_status(LcdMode::ReadingOAMRAM);
                    req_int = self.emu.memory.is_interrupt_requested(5);
                    LcdMode::ReadingOAMRAM
                }
                81..=252 => {
                    self.emu.memory.set_lcd_status(LcdMode::TransfToLCDDriver);
                    LcdMode::TransfToLCDDriver
                }
                _ => {
                    self.emu.memory.set_lcd_status(LcdMode::HBlank);
                    req_int = self.emu.memory.is_interrupt_requested(3);
                    LcdMode::HBlank
                }
            }
        };
        if req_int && (mode != current_mode) {
            self.req_interrupt(1);
        }
        if current_line == self.emu.memory.get_lyc() {
            self.emu.memory.set_coincidence_flag();
            if self.emu.memory.is_interrupt_requested(6) {
                self.req_interrupt(1);
            }
        } else {
            self.emu.memory.clear_coincidence_flag();
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

    fn make_pixels(&self, data1: u8, data2: u8) -> Vec<u8> {
        let hi_byte = (0..8).rev().map(|i| get_bit_at(data2, i) as u8);
        let low_byte = (0..8).rev().map(|i| get_bit_at(data1, i) as u8);
        hi_byte.zip(low_byte).map(|(hi, lo)| hi << 1 | lo).collect()
    }

    fn get_tile_ids(&self, bg_mem: u16) -> (u16, u16) {
        let tiledata_region = self.emu.memory.bg_tile_data_select();
        let data = self.emu.memory.read(bg_mem);
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

    fn get_y_pos(&self, window_enabled: bool, sy: u8, wy: u8, current_line: u8) -> u8 {
        if window_enabled {
            current_line.wrapping_sub(wy)
        } else {
            current_line.wrapping_add(sy)
        }
    }

    fn get_x_flip(&self, attributes: u8) -> bool {
        get_bit_at(attributes, 5)
    }

    fn get_y_flip(&self, attributes: u8) -> bool {
        get_bit_at(attributes, 6)
    }

    fn get_sprites_palette(&self, attributes: u8) -> u8 {
        if get_bit_at(attributes, 4) {
            return self.emu.memory.read(0xff49);
        }
        self.emu.memory.read(0xff48)
    }

    fn has_priority(&self, attributes: u8) -> bool {
        !get_bit_at(attributes, 7)
    }

    fn render_background(&mut self, buffer: &mut Vec<(u8, u8)>) {
        let palette = self.emu.memory.background_palette();
        let bg_map = self.emu.memory.map_select();
        let Point2D { x: sx, y: sy } = self.emu.memory.background_position();
        let Point2D { x: wx, y: wy } = self.emu.memory.window_position();
        let current_line = self.emu.memory.get_ly();
        let window_enabled = self.emu.memory.window_enabled() && wy <= current_line;
        let y_pos = self.get_y_pos(window_enabled, sy, wy, current_line);
        let visible_tiles = 256 as u16 / 8;
        let from = bg_map + (y_pos as u16 / 8) * 32;
        let pixel_row = (y_pos % 8) * 2;
        for (tile_pos, bg_mem) in (from..(from + visible_tiles)).enumerate() {
            let (tile1, tile2) = self.get_tile_ids(bg_mem);
            let data1 = self.emu.memory.read(pixel_row as u16 + tile1);
            let data2 = self.emu.memory.read(pixel_row as u16 + tile2);
            let pixels = self.make_pixels(data1, data2);
            pixels.iter().enumerate().for_each(|(i, pixel)| {
                let pixel_pos = (tile_pos * 8) + i;
                let x_pos = self.get_x_pos(window_enabled, sx, wx, pixel_pos as u8) as usize;
                if x_pos < SCREEN_WIDTH {
                    buffer[x_pos] = (*pixel, palette)
                }
            });
        }
    }

    fn render_sprites(&mut self, buffer: &mut Vec<(u8, u8)>) {
        let size = self.emu.memory.sprite_size();
        let current_line = self.emu.memory.get_ly();
        for sprite_pos in (0..160).step_by(4) {
            let y_pos = self.emu.memory.read(0xfe00 + sprite_pos).wrapping_sub(16);
            let x_pos = self
                .emu
                .memory
                .read(0xfe00 + sprite_pos + 1)
                .wrapping_sub(8);
            let tile_location = self.emu.memory.read(0xfe00 + sprite_pos + 2);
            let attributes = self.emu.memory.read(0xfe00 + sprite_pos + 3);
            let palette = self.get_sprites_palette(attributes);
            let mut pixel_row = current_line.wrapping_sub(y_pos);
            if current_line >= y_pos && current_line < (y_pos + size) {
                if self.get_y_flip(attributes) {
                    pixel_row = pixel_row.wrapping_sub(size);
                    pixel_row = !pixel_row;
                }
                let data_address = (0x8000 + (tile_location as u16 * 16)) + pixel_row as u16 * 2;
                let data1 = self.emu.memory.read(data_address);
                let data2 = self.emu.memory.read(data_address + 1);
                let mut pixels = self.make_pixels(data1, data2);
                if self.get_x_flip(attributes) {
                    pixels.reverse();
                }
                pixels.iter().enumerate().for_each(|(i, pixel)| {
                    let pixel_pos = x_pos as usize + i;
                    if pixel_pos < SCREEN_WIDTH && *pixel != 0x00 {
                        let bg_pixel = buffer[pixel_pos];
                        if self.has_priority(attributes) || bg_pixel.0 == 0x00 {
                            buffer[pixel_pos] = (*pixel, palette);
                        }
                    }
                });
            }
        }
    }

    fn draw_scan_line(&mut self) {
        let mut buffer: Vec<(u8, u8)> = vec![(0, 0); SCREEN_WIDTH];
        if self.emu.memory.background_enabled() {
            self.render_background(&mut buffer);
        }
        if self.emu.memory.sprite_enabled() {
            self.render_sprites(&mut buffer);
        }
        let colored_pixels: Vec<u32> = buffer
            .into_iter()
            .map(|(pixel, palette)| Gpu::get_color(pixel, palette))
            .collect();
        self.emu.frame_buffer.extend(colored_pixels);
    }
}

pub fn update(emu: &mut Emulator, frame_cycles: u32) {
    if !emu.memory.is_lcd_enabled() {
        emu.timers.scan_line_counter = 0;
        emu.memory.write_scanline(0);
        emu.memory.set_lcd_status(LcdMode::VBlank);
        emu.frame_buffer.clear();
        return;
    }
    let mut gpu = Gpu::new(emu);
    gpu.set_lcd_mode();

    gpu.emu.timers.scan_line_counter += frame_cycles;
    if gpu.emu.timers.scan_line_counter > 456 {
        let scan_line = gpu.emu.memory.get_ly();
        gpu.emu.timers.scan_line_counter = 0;
        match scan_line {
            0..=143 => {
                gpu.draw_scan_line();
            }
            144 => {
                gpu.req_interrupt(0);
            }
            _ => {}
        };
        if scan_line > 153 {
            gpu.emu.memory.write_scanline(0);
        } else {
            gpu.emu.memory.increment_scanline();
        }
    }
}
