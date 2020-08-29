use super::emulator::Emulator;
use super::interrupts::request_interrupt;
use super::interrupts::Interrupts;
use super::memory::LcdMode;
use std::iter::FromIterator;

#[allow(non_camel_case_types)]
pub enum Action {
  new_mode(LcdMode),
  request_interrupt(u8),
  ime1,
  reload_tima(bool),
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
        Action::request_interrupt(bit) => request_interrupt(ctx, bit),
        Action::ime1 => ctx.timers.ime = true,
        Action::reload_tima(true) => {
          // TIMA != 0 means that tima was overwritten, reload is prevented
          if ctx.memory.get_tima() == 0 {
            ctx.memory.set_tima(ctx.memory.get_tma());
            request_interrupt(ctx, Interrupts::Timer as u8);
            ctx.dispatcher.dispatch(Action::reload_tima(false));
            // ignores tima writes in memory
            ctx.memory.tima_reloading = true;
          }
        }
        Action::reload_tima(false) => {
          ctx.memory.tima_reloading = false;
        }
      }
    }
  }

  pub fn dispatch(&mut self, action: Action) {
    self.actions_queue.push(action);
  }
}
