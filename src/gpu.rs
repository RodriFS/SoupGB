use super::constants::*;
use super::dispatcher::Action;
use super::emulator::Emulator;
use super::interrupts::{stat_irq, Interrupts, StatCond};
use super::memory::{LcdMode, Point2D, PrevStatCond};
use super::utils::get_bit_at;

fn set_lcd_mode(ctx: &mut Emulator) {
    let current_line = ctx.memory.get_ly();
    let current_mode = ctx.memory.lcd_mode();
    let mut stat_int_requested = stat_irq(ctx, StatCond::LYC);
    match current_mode {
        // mode 2
        LcdMode::ReadOAM => {
            if ctx.timers.scan_line_counter >= 80 {
                // go to mode 3
                ctx.timers.scan_line_counter = 0;
                ctx.dispatcher.dispatch(Action::new_mode(LcdMode::ReadVRAM));
            }
        }
        // mode 3
        LcdMode::ReadVRAM => {
            if ctx.timers.scan_line_counter >= 172 {
                // go to mode 0
                ctx.timers.scan_line_counter = 0;
                ctx.dispatcher.dispatch(Action::new_mode(LcdMode::HBlank));
                stat_int_requested = stat_irq(ctx, StatCond::HBLANK);
                draw_scan_line(ctx);
            }
        }
        // mode 0
        LcdMode::HBlank => {
            if ctx.timers.scan_line_counter >= 204 {
                ctx.timers.scan_line_counter = 0;
                ctx.memory.increment_ly();
                if ctx.memory.get_ly() > 0x8F {
                    // go to mode 1
                    ctx.dispatcher.dispatch(Action::new_mode(LcdMode::VBlank));
                    ctx.dispatcher
                        .dispatch(Action::interrupt_request(Interrupts::VBlank as u8));
                    stat_int_requested = StatCond::or(
                        stat_irq(ctx, StatCond::VBlank),
                        stat_irq(ctx, StatCond::OAM),
                    );
                } else {
                    // go to mode 2
                    ctx.dispatcher.dispatch(Action::new_mode(LcdMode::ReadOAM));
                    stat_int_requested = stat_irq(ctx, StatCond::OAM);
                };
            }
        }
        // mode 1
        LcdMode::VBlank => {
            match current_line {
                0x90..=0x98 if ctx.timers.scan_line_counter >= 456 => {
                    ctx.timers.scan_line_counter = 0;
                    ctx.memory.increment_ly();
                }
                0x99 if ctx.timers.scan_line_counter >= 56 => {
                    ctx.timers.scan_line_counter = 0;
                    ctx.memory.write_ly(0);
                }
                0x00 if ctx.timers.scan_line_counter >= 856 => {
                    // go to mode 2
                    ctx.timers.scan_line_counter = 0;
                    ctx.dispatcher.dispatch(Action::new_mode(LcdMode::ReadOAM));
                    stat_int_requested = stat_irq(ctx, StatCond::OAM);
                }
                _ => {}
            }
        }
    };

    if stat_int_requested.is_stat() && check_stat_conditions(ctx, &stat_int_requested) {
        ctx.dispatcher
            .dispatch(Action::interrupt_request(Interrupts::LCDStat as u8));
        update_prev_stat_condition(ctx, stat_int_requested, current_line);
    }
}

fn check_stat_conditions(ctx: &mut Emulator, stat_request: &StatCond) -> bool {
    if ctx.memory.prev_stat_condition == PrevStatCond::OAM {
        return true;
    }
    let lyc = ctx.memory.get_lyc();
    let ly = ctx.memory.get_ly();
    //   Some STAT-conditions cause the following STAT-condition to be ignored:
    match ctx.memory.prev_stat_condition {
        //   Past  VBLANK           following  LYC=91..99,00        is ignored
        PrevStatCond::VBlank if stat_request == &StatCond::LYC => match lyc {
            0x00 => false,
            0x91..=0x99 => false,
            _ => true,
        },
        //   Past  VBLANK           following  OAM         (at 00)  is ignored
        PrevStatCond::VBlank if stat_request == &StatCond::OAM && ly == 0 => false,
        //   Past  LYC=00 at 99.2   following  OAMs (at 00 and 01) are ignored
        PrevStatCond::LYC(0x00) if stat_request == &StatCond::OAM && (lyc == 0 || lyc == 1) => {
            false
        }
        //   Past  LYC=01..8F       following  OAM     (at 02..90)  is ignored
        PrevStatCond::LYC(0x01..=0x8f) if stat_request == &StatCond::OAM => match ly {
            0x02..=0x90 => false,
            _ => true,
        },
        //   Past  LYC=00..8F       following  HBLANK  (at 00..8F)  is ignored
        PrevStatCond::LYC(0x00..=0x8F) if stat_request == &StatCond::HBLANK => match ly {
            0x00..=0x8F => false,
            _ => true,
        },
        //   Past  LYC=8F           following  VBLANK               is ignored
        PrevStatCond::LYC(0x8F) if stat_request == &StatCond::VBlank => false,
        //   Past  HBLANK           following  OAM                  is ignored
        PrevStatCond::HBLANK(..) if stat_request == &StatCond::OAM => false,
        //   Past  HBLANK at 8F     following  VBLANK               is ignored
        PrevStatCond::HBLANK(0x8F) if stat_request == &StatCond::VBlank => false,
        _ => true,
    }
}

fn update_prev_stat_condition(ctx: &mut Emulator, stat_int_requested: StatCond, current_line: u8) {
    ctx.memory.prev_stat_condition = match stat_int_requested {
        StatCond::HBLANK => PrevStatCond::HBLANK(current_line),
        StatCond::VBlank => PrevStatCond::VBlank,
        StatCond::OAM => PrevStatCond::OAM,
        StatCond::LYC => PrevStatCond::LYC(current_line),
        _ => unreachable!(),
    }
}

fn get_color(pixel: u8, palette: u8) -> u32 {
    let color = match pixel {
        0x00 => palette & 0b0000_0011,
        0x01 => (palette & 0b0000_1100) >> 2,
        0x02 => (palette & 0b0011_0000) >> 4,
        0x03 => (palette & 0b1100_0000) >> 6,
        _ => unreachable!(),
    };
    match color {
        0x00 => 0xff_ff_ff,
        0x01 => 0xea_ec_ee,
        0x02 => 0x56_65_73,
        0x03 => 0x00_00_00,
        _ => unreachable!(),
    }
}

fn make_pixels(data1: u8, data2: u8) -> Vec<u8> {
    let hi_byte = (0..8).rev().map(|i| get_bit_at(data2, i) as u8);
    let low_byte = (0..8).rev().map(|i| get_bit_at(data1, i) as u8);
    hi_byte.zip(low_byte).map(|(hi, lo)| hi << 1 | lo).collect()
}

fn get_tile_ids(ctx: &mut Emulator, bg_mem: u16) -> (u16, u16) {
    let tiledata_region = ctx.memory.bg_tile_data_select();
    let data = ctx.memory.read_unchecked(bg_mem);
    let tile_id = match tiledata_region {
        0x8000 => data as u16 * 16,
        0x8800 => ((data as i8) as u16).wrapping_add(128) * 16,
        _ => unreachable!(),
    };
    (tiledata_region + tile_id, tiledata_region + tile_id + 1)
}

fn get_x_pos(window_enabled: bool, sx: u8, wx: u8, pixel_pos: u8) -> u8 {
    if window_enabled && pixel_pos >= wx {
        pixel_pos.wrapping_add(wx)
    } else {
        pixel_pos.wrapping_sub(sx)
    }
}

fn get_y_pos(window_enabled: bool, sy: u8, wy: u8, current_line: u8) -> u8 {
    if window_enabled {
        current_line.wrapping_sub(wy)
    } else {
        current_line.wrapping_add(sy)
    }
}

fn get_x_flip(attributes: u8) -> bool {
    get_bit_at(attributes, 5)
}

fn get_y_flip(attributes: u8) -> bool {
    get_bit_at(attributes, 6)
}

fn get_sprites_palette(ctx: &mut Emulator, attributes: u8) -> u8 {
    if get_bit_at(attributes, 4) {
        return ctx.memory.read_unchecked(0xff49);
    }
    ctx.memory.read_unchecked(0xff48)
}

fn has_priority(attributes: u8) -> bool {
    !get_bit_at(attributes, 7)
}

fn render_background(ctx: &mut Emulator, buffer: &mut Vec<(u8, u8)>) {
    let palette = ctx.memory.background_palette();
    let bg_map = ctx.memory.map_select();
    let Point2D { x: sx, y: sy } = ctx.memory.background_position();
    let current_line = ctx.memory.get_ly();
    let y_pos = get_y_pos(false, sy, 0, current_line);
    let visible_tiles = 256 as u16 / 8;
    let from = bg_map + (y_pos as u16 / 8) * 32;
    let pixel_row = (y_pos % 8) * 2;
    for (tile_pos, bg_mem) in (from..(from + visible_tiles)).enumerate() {
        let (tile1, tile2) = get_tile_ids(ctx, bg_mem);
        let data1 = ctx.memory.read_unchecked(pixel_row as u16 + tile1);
        let data2 = ctx.memory.read_unchecked(pixel_row as u16 + tile2);
        let pixels = make_pixels(data1, data2);
        pixels.iter().enumerate().for_each(|(i, pixel)| {
            let pixel_pos = (tile_pos * 8) + i;
            let x_pos = get_x_pos(false, sx, 0, pixel_pos as u8) as usize;
            if x_pos < SCREEN_WIDTH {
                buffer[x_pos] = (*pixel, palette)
            }
        });
    }
}

fn render_window(ctx: &mut Emulator, buffer: &mut Vec<(u8, u8)>) {
    let palette = ctx.memory.background_palette();
    let bg_map = ctx.memory.map_select();
    let Point2D { x: sx, y: sy } = ctx.memory.background_position();
    let Point2D { x: wx, y: wy } = ctx.memory.window_position();
    let current_line = ctx.memory.get_ly();
    let window_enabled = ctx.memory.window_enabled() && wy <= current_line;
    let y_pos = get_y_pos(window_enabled, sy, wy, current_line);
    let visible_tiles = 256 as u16 / 8;
    let from = bg_map + (y_pos as u16 / 8) * 32;
    let pixel_row = (y_pos % 8) * 2;
    for (tile_pos, bg_mem) in (from..(from + visible_tiles)).enumerate() {
        let (tile1, tile2) = get_tile_ids(ctx, bg_mem);
        let data1 = ctx.memory.read_unchecked(pixel_row as u16 + tile1);
        let data2 = ctx.memory.read_unchecked(pixel_row as u16 + tile2);
        let pixels = make_pixels(data1, data2);
        pixels.iter().enumerate().for_each(|(i, pixel)| {
            let pixel_pos = (tile_pos * 8) + i;
            let x_pos = get_x_pos(window_enabled, sx, wx, pixel_pos as u8) as usize;
            if x_pos < SCREEN_WIDTH {
                buffer[x_pos] = (*pixel, palette)
            }
        });
    }
}

fn render_sprites(ctx: &mut Emulator, buffer: &mut Vec<(u8, u8)>) {
    let size = ctx.memory.sprite_size();
    let current_line = ctx.memory.get_ly();
    for sprite_pos in (0..160).step_by(4) {
        let y_pos = ctx
            .memory
            .read_unchecked(0xfe00 + sprite_pos)
            .wrapping_sub(16);
        let x_pos = ctx
            .memory
            .read_unchecked(0xfe00 + sprite_pos + 1)
            .wrapping_sub(8);
        let tile_location = ctx.memory.read_unchecked(0xfe00 + sprite_pos + 2);
        let attributes = ctx.memory.read_unchecked(0xfe00 + sprite_pos + 3);
        let palette = get_sprites_palette(ctx, attributes);
        let mut pixel_row = current_line.wrapping_sub(y_pos);
        if current_line >= y_pos && current_line < (y_pos + size) {
            if get_y_flip(attributes) {
                pixel_row = pixel_row.wrapping_sub(size);
                pixel_row = !pixel_row;
            }
            let data_address = (0x8000 + (tile_location as u16 * 16)) + pixel_row as u16 * 2;
            let data1 = ctx.memory.read_unchecked(data_address);
            let data2 = ctx.memory.read_unchecked(data_address + 1);
            let mut pixels = make_pixels(data1, data2);
            if get_x_flip(attributes) {
                pixels.reverse();
            }
            pixels.iter().enumerate().for_each(|(i, pixel)| {
                let pixel_pos = x_pos as usize + i;
                if pixel_pos < SCREEN_WIDTH && *pixel != 0x00 {
                    let bg_pixel = buffer[pixel_pos];
                    if has_priority(attributes) || bg_pixel.0 == 0x00 {
                        buffer[pixel_pos] = (*pixel, palette);
                    }
                }
            });
        }
    }
}

fn draw_scan_line(ctx: &mut Emulator) {
    let mut buffer: Vec<(u8, u8)> = vec![(0, 0); SCREEN_WIDTH];
    if ctx.memory.background_enabled() {
        render_background(ctx, &mut buffer);
    }
    if ctx.memory.window_enabled() {
        render_window(ctx, &mut buffer);
    }
    if ctx.memory.sprite_enabled() {
        render_sprites(ctx, &mut buffer);
    }
    let colored_pixels: Vec<u32> = buffer
        .into_iter()
        .map(|(pixel, palette)| get_color(pixel, palette))
        .collect();
    ctx.frame_buffer.extend(colored_pixels);
}

pub fn update(ctx: &mut Emulator) {
    if !ctx.memory.is_lcd_enabled() {
        ctx.timers.scan_line_counter = 0;
        ctx.memory.write_ly(0x00);
        ctx.memory.set_lcd_status(LcdMode::VBlank); // Check
        ctx.frame_buffer.clear();
        return;
    }
    ctx.timers.scan_line_counter += 4;
    set_lcd_mode(ctx);
}
