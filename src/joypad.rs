use super::constants::*;
use super::interrupts::Action;
use super::emulator::Emulator;
use super::interrupts::Interrupts;
use minifb::{Key, Window};

pub fn update(ctx: &mut Emulator, window: &Window) {
  let p1 = !ctx.memory.read(P1_JOYPAD_ADDRESS) & 0b0011_1111;
  match p1 {
    0b10_0000 if window.is_key_down(Key::Down) => request_joypad_interrupt(ctx, p1, 0b1000),
    0b10_0000 if window.is_key_down(Key::Up) => request_joypad_interrupt(ctx, p1, 0b0100),
    0b10_0000 if window.is_key_down(Key::Left) => request_joypad_interrupt(ctx, p1, 0b0010),
    0b10_0000 if window.is_key_down(Key::Right) => request_joypad_interrupt(ctx, p1, 0b0001),
    0b01_0000 if window.is_key_down(Key::Enter) => request_joypad_interrupt(ctx, p1, 0b1000),
    0b01_0000 if window.is_key_down(Key::Space) => request_joypad_interrupt(ctx, p1, 0b0100),
    0b01_0000 if window.is_key_down(Key::Z) => request_joypad_interrupt(ctx, p1, 0b0010),
    0b01_0000 if window.is_key_down(Key::X) => request_joypad_interrupt(ctx, p1, 0b0001),
    _ if window.is_key_down(Key::P) => ctx.debug(),
    _ if window.is_key_down(Key::B) => ctx.toggle_background(),
    _ if window.is_key_down(Key::S) => ctx.toggle_sprites(),
    _ if window.is_key_down(Key::W) => ctx.toggle_window(),
    _ => {}
  };
}

fn request_joypad_interrupt(ctx: &mut Emulator, p1: u8, joypad_reg: u8) {
  ctx.memory.write(P1_JOYPAD_ADDRESS, p1 | joypad_reg);
  ctx
    .interrupts
    .dispatch(Action::request_interrupt(Interrupts::Joypad as u8));
}
