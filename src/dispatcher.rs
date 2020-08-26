use super::emulator::Emulator;
use super::interrupts::request_interrupt;
use super::memory::LcdMode;
use std::iter::FromIterator;

#[allow(non_camel_case_types)]
pub enum Action {
  new_mode(LcdMode),
  interrupt_request(u8),
  ime1,
}

#[derive(Default)]
pub struct Dispatcher {
  actions_queue: Vec<Action>,
}

impl Dispatcher {
  pub fn run(ctx: &mut Emulator) {
    let actions_queue = Vec::from_iter(ctx.dispatcher.actions_queue.drain(..));
    for action in actions_queue {
      match action {
        Action::new_mode(mode) => ctx.memory.set_lcd_status(mode),
        Action::interrupt_request(bit) => request_interrupt(ctx, bit),
        Action::ime1 => ctx.timers.ime = true,
      }
    }
  }

  pub fn dispatch(&mut self, action: Action) {
    self.actions_queue.push(action);
  }
}
