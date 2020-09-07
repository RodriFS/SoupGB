use super::constants::*;
use super::emulator::Emulator;
use super::memory::Point2D;
use super::utils::get_bit_at;

struct RenderProps {
  wx: u8,
  wy: u8,
  sx: u8,
  sy: u8,
  palette: u8,
  bg_map: u16,
  ly: u8,
}

impl RenderProps {
  fn new(ctx: &mut Emulator) -> Self {
    let Point2D { x: sx, y: sy } = ctx.memory.background_position();
    let Point2D { x: wx, y: wy } = ctx.memory.window_position();
    let palette = ctx.memory.background_palette();
    let bg_map = ctx.memory.map_select();
    let ly = ctx.memory.get_ly();
    Self {
      wx,
      wy,
      sx,
      sy,
      palette,
      bg_map,
      ly,
    }
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

fn get_y_pos(window_enabled: bool, y: u8, current_line: u8) -> u8 {
  if window_enabled {
    current_line.wrapping_sub(y)
  } else {
    current_line.wrapping_add(y)
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

fn render_background(ctx: &mut Emulator, buffer: &mut Vec<(u8, u8)>, props: &RenderProps) {
  let y_pos = get_y_pos(false, props.sy, props.ly);
  let from = props.bg_map + (y_pos as u16 / 8) * 32;
  let to = from + 32;
  let pixel_row = (y_pos % 8) * 2;
  (from..to)
    .flat_map(|bg_mem| {
      let (tile1, tile2) = get_tile_ids(ctx, bg_mem);
      let data1 = ctx.memory.read_unchecked(pixel_row as u16 + tile1);
      let data2 = ctx.memory.read_unchecked(pixel_row as u16 + tile2);
      make_pixels(data1, data2)
    })
    .enumerate()
    .for_each(|(i, pixel)| {
      let x_pos = get_x_pos(false, props.sx, 0, i as u8) as usize;
      if x_pos < SCREEN_WIDTH {
        buffer[x_pos] = (pixel, props.palette)
      }
    })
}

fn render_window(ctx: &mut Emulator, buffer: &mut Vec<(u8, u8)>, props: &RenderProps) {
  let y_pos = get_y_pos(false, props.wy, props.ly);
  let from = props.bg_map + (y_pos as u16 / 8) * 32;
  let to = from + 32;
  let pixel_row = (y_pos % 8) * 2;
  for (tile_pos, bg_mem) in (from..to).enumerate() {
    let (tile1, tile2) = get_tile_ids(ctx, bg_mem);
    let data1 = ctx.memory.read_unchecked(pixel_row as u16 + tile1);
    let data2 = ctx.memory.read_unchecked(pixel_row as u16 + tile2);
    let pixels = make_pixels(data1, data2);
    pixels.iter().enumerate().for_each(|(i, pixel)| {
      let pixel_pos = (tile_pos * 8) + i;
      let x_pos = get_x_pos(true, 0, props.wx, pixel_pos as u8) as usize;
      if x_pos < SCREEN_WIDTH {
        buffer[x_pos] = (*pixel, props.palette)
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

pub fn draw_scan_line(ctx: &mut Emulator) {
  let mut buffer: Vec<(u8, u8)> = vec![(0, 0); SCREEN_WIDTH];
  let render_props = RenderProps::new(ctx);
  let window_enabled = ctx.memory.window_enabled() && render_props.wy <= render_props.ly;
  if ctx.memory.background_enabled() {
    render_background(ctx, &mut buffer, &render_props);
  }
  if window_enabled {
    render_window(ctx, &mut buffer, &render_props);
  }
  if ctx.memory.sprite_enabled() {
    render_sprites(ctx, &mut buffer);
  }

  let current_line = render_props.ly as usize * SCREEN_WIDTH;
  buffer
    .into_iter()
    .map(|(pixel, palette)| get_color(pixel, palette))
    .enumerate()
    .for_each(|(n, pixel)| ctx.frame_buffer[current_line + n] = pixel)
}
