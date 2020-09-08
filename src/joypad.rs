use super::constants::*;
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
    Key::P,
    Key::B,
    Key::S,
    Key::W,
  ] {
    if window.is_key_down(input) {
      let p1 = !ctx.memory.read(P1_JOYPAD_ADDRESS) & 0b0011_1111;
      let joypad_reg = match input {
        Key::Down if p1 == 0b10_0000 => 0b1000,
        Key::Up if p1 == 0b10_0000 => 0b0100,
        Key::Left if p1 == 0b10_0000 => 0b0010,
        Key::Right if p1 == 0b10_0000 => 0b0001,
        Key::Enter if p1 == 0b01_0000 => 0b1000,
        Key::Space if p1 == 0b01_0000 => 0b0100,
        Key::Z if p1 == 0b01_0000 => 0b0010,
        Key::X if p1 == 0b01_0000 => 0b0001,
        Key::P => {
          ctx.debug();
          0
        }
        Key::B => {
          ctx.toggle_background();
          0
        }
        Key::S => {
          ctx.toggle_sprites();
          0
        }
        Key::W => {
          ctx.toggle_window();
          0
        }
        _ => 0,
      };
      if joypad_reg != 0 {
        ctx.memory.write(P1_JOYPAD_ADDRESS, p1 | joypad_reg);
        ctx
          .dispatcher
          .dispatch(Action::request_interrupt(Interrupts::Joypad as u8));
        break;
      }
    }
  }
}
