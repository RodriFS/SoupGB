use crate::memory::Memory;

use super::constants::*;
use super::emulator::Emulator;
use super::memory::Point2D;
use super::utils::get_bit_at;

pub struct RenderProps {
  wx: u8,
  wy: u8,
  sx: u8,
  sy: u8,
  palette: u8,
  bg_map: u16,
  ly: u8,
}

impl RenderProps {
  pub fn new(memory: &Memory) -> Self {
    let Point2D { x: sx, y: sy } = memory.background_position();
    let Point2D { x: wx, y: wy } = memory.window_position();
    let palette = memory.background_palette();
    let bg_map = memory.map_select();
    let ly = memory.get_ly();
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

  fn reload(&mut self, memory: &Memory) -> &Self {
    let Point2D { x: sx, y: sy } = memory.background_position();
    let Point2D { x: wx, y: wy } = memory.window_position();
    self.wx = wx;
    self.wy = wy;
    self.sx = sx;
    self.sy = sy;
    self.palette = memory.background_palette();
    self.bg_map = memory.map_select();
    self.ly = memory.get_ly();
    self
  }
}

fn get_color(pixel: &u8, palette: &u8) -> u32 {
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
  (0..8).rev().map(|i| {
    let hi = get_bit_at(data2, i) as u8;
    let lo = get_bit_at(data1, i) as u8;
    hi << 1 | lo
  }).collect()
}

#[test]
fn make_pixels_test() {
  let result = make_pixels(0b1111_1111, 0b1111_1111);
  assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 3, 2]);
}

fn get_tile_ids(memory: &Memory, bg_mem: u16) -> (u16, u16) {
  let tiledata_region = memory.bg_tile_data_select();
  let data = memory.read_unchecked(bg_mem);
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

fn get_sprites_palette(memory: &Memory, attributes: u8) -> u8 {
  if get_bit_at(attributes, 4) {
    return memory.read_unchecked(0xff49);
  }
  memory.read_unchecked(0xff48)
}

fn has_priority(attributes: u8) -> bool {
  !get_bit_at(attributes, 7)
}

fn make_tiles(memory: &Memory, bg_mem: u16, pixel_row: u16) -> Vec<u8> {
  let (tile1, tile2) = get_tile_ids(memory, bg_mem);
  let data1 = memory.read_unchecked(pixel_row + tile1);
  let data2 = memory.read_unchecked(pixel_row + tile2);
  make_pixels(data1, data2)
}



fn render_background(ctx: &Memory, buffer: &mut [(u8, u8)], props: &RenderProps) {
  let y_pos = get_y_pos(false, props.sy, props.ly);
  let from = props.bg_map + (y_pos as u16 / 8) * 32;
  let to = from + 32;
  let pixel_row = (y_pos % 8) * 2;
  (from..to)
    .flat_map(|bg_mem| make_tiles(ctx, bg_mem, pixel_row as u16))
    .enumerate()
    .for_each(|(i, pixel)| {
      let x_pos = get_x_pos(false, props.sx, 0, i as u8) as usize;
      if x_pos < SCREEN_WIDTH {
        buffer[x_pos] = (pixel, props.palette)
      }
    })
}

#[test]
fn render_background_test() {
  let ctx = Emulator::default();
  let mut buffer = [(0, 0); SCREEN_WIDTH];
  let props = RenderProps::new(&ctx.memory);

  render_background(&ctx.memory, &mut buffer, &props);
  let result = [(0, 252); SCREEN_WIDTH];
  assert_eq!(result, buffer)
}

fn render_window(memory: &Memory, buffer: &mut [(u8, u8)], props: &RenderProps) {
  let y_pos = get_y_pos(false, props.wy, props.ly);
  let from = props.bg_map + (y_pos as u16 / 8) * 32;
  let to = from + 32;
  let pixel_row = (y_pos % 8) * 2;
  (from..to)
    .flat_map(|bg_mem| make_tiles(memory, bg_mem, pixel_row as u16))
    .enumerate()
    .for_each(|(i, pixel)| {
      let x_pos = get_x_pos(true, 0, props.wx, i as u8) as usize;
      if x_pos < SCREEN_WIDTH {
        buffer[x_pos] = (pixel, props.palette)
      }
    })
}

fn render_sprites(memory: &Memory, buffer: &mut [(u8, u8)]) {
  let size = memory.sprite_size();
  let current_line = memory.get_ly();
  for sprite_pos in (0..160).step_by(4) {
    let y_pos = memory
      .read_unchecked(0xfe00 + sprite_pos)
      .wrapping_sub(16);
    let x_pos = memory
      .read_unchecked(0xfe00 + sprite_pos + 1)
      .wrapping_sub(8);
    let tile_location = memory.read_unchecked(0xfe00 + sprite_pos + 2);
    let attributes = memory.read_unchecked(0xfe00 + sprite_pos + 3);
    let palette = get_sprites_palette(memory, attributes);
    let mut pixel_row = current_line.wrapping_sub(y_pos);
    if current_line >= y_pos && current_line < (y_pos + size) {
      if get_y_flip(attributes) {
        pixel_row = pixel_row.wrapping_sub(size);
        pixel_row = !pixel_row;
      }
      let data_address = (0x8000 + (tile_location as u16 * 16)) + pixel_row as u16 * 2;
      let data1 = memory.read_unchecked(data_address);
      let data2 = memory.read_unchecked(data_address + 1);
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
  let memory = &ctx.memory;
  let mut buffer = ctx.line_buffer;
  let render_props = ctx.render_props.reload(memory);
  let window_enabled = memory.window_enabled() && render_props.wy <= render_props.ly;
  if memory.background_enabled() {
    render_background(memory, &mut buffer, &render_props);
  }
  if window_enabled {
    render_window(memory, &mut buffer, &render_props);
  }
  if memory.sprite_enabled() {
    render_sprites(memory, &mut buffer);
  }

  let current_line = render_props.ly as usize * SCREEN_WIDTH;
  buffer
    .iter()
    .enumerate()
    .for_each(|(n, (pixel, palette))| {
      let pixel = get_color(&&pixel, &&palette);
      if ctx.active_buffer {
        ctx.frame_buffer[current_line + n] = pixel
      } else {
        ctx.background_buffer[current_line + n] = pixel
      }
    })
}
