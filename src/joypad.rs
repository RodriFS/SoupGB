use super::dispatcher::Action;
use super::emulator::Emulator;
use super::interrupts::Interrupts;
use minifb::{Key, Window};

pub fn update(ctx: &mut Emulator, window: &Window) {
  for &input in &[
    Key::Up,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Enter,
    Key::Space,
    Key::Z,
    Key::X,
  ] {
    if window.is_key_down(input) {
      let joypad_reg = match input {
        Key::Down => 0b10_1000,
        Key::Up => 0b10_0100,
        Key::Left => 0b10_0010,
        Key::Right => 0b10_0001,
        Key::Enter => 0b01_1000,
        Key::Space => 0b01_0100,
        Key::Z => 0b01_0010,
        Key::X => 0b01_0001,
        _ => unreachable!(),
      };
      ctx.memory.write(0xff00, joypad_reg | 0b1100_0000);
      let bit_enabled = joypad_reg & 0b0000_1111;
      if bit_enabled != 1 && ctx.memory.prev_joypad_bit == 1 {
        ctx
          .dispatcher
          .dispatch(Action::request_interrupt(Interrupts::Joypad as u8));
      }
      ctx.memory.prev_joypad_bit = bit_enabled;
      break;
    }
  }
}
