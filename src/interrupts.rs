use super::emulator::Emulator;
use super::utils::get_bit_at;
use super::utils::*;
use super::memory::LcdMode;

#[derive(PartialEq, Debug)]
pub enum StatCond {
    HBLANK,
    VBlank,
    OAM,
    LYC,
    None,
}

impl StatCond {
    pub fn or(stat1: StatCond, stat2: StatCond) -> StatCond {
        if stat1 == StatCond::None {
            stat2
        } else {
            stat1
        }
    }

    pub fn is_stat(&self) -> bool {
        self != &StatCond::None
    }
}

pub enum Interrupts {
    VBlank = 0,
    LCDStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

pub fn update(ctx: &mut Emulator) {
    let i_f = ctx.memory.read(0xff0f);
    let i_e = ctx.memory.read(0xffff);
    if i_f > 0 {
        for bit in 0..5 {
            if get_bit_at(i_f, bit) && get_bit_at(i_e, bit) {
                if ctx.timers.is_halted && !ctx.timers.ime {
                    ctx.timers.halt_bug = true;
                }
                ctx.timers.is_halted = false;
                if ctx.timers.ime {
                    ctx.timers.clear_ime();
                    let cancelled = interrupt_execution(ctx, bit);
                    if !cancelled {
                        break;
                    }
                }
            }
        }
    }
}

pub fn interrupt_execution(ctx: &mut Emulator, interrupt: u8) -> bool {
    ctx.take_cycle();
    ctx.take_cycle();
    let pc = ctx.memory.get_pc();
    ctx.s_push_hi(pc);
    let interrupt_enable = ctx.memory.read(0xffff);
    if get_bit_at(interrupt_enable, interrupt) {
        ctx.s_push_lo(pc);
        let i_f = ctx.memory.read(0xff0f);
        let clear_request = clear_bit_at(i_f, interrupt);
        ctx.memory.write(0xff0f, clear_request);
        let new_pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => return true,
        };
        ctx.memory.set_pc(new_pc as u16);
        ctx.take_cycle();
        false
    } else {
        ctx.memory.set_pc(0);
        true
    }
}

pub fn stat_irq(ctx: &Emulator, stat: StatCond) -> StatCond {
    let bit = match stat {
        StatCond::HBLANK => 3,
        StatCond::VBlank => 4,
        StatCond::OAM => 5,
        StatCond::LYC => 6,
        _ => unreachable!(),
    };
    let lcd_status = ctx.memory.read(0xff41);
    if get_bit_at(lcd_status, bit) {
        stat
    } else {
        StatCond::None
    }
}

pub fn request_interrupt(ctx: &mut Emulator, bit: u8) {
    let interrupt_flags = ctx.memory.read(0xff0f);
    let modified_flag = set_bit_at(interrupt_flags, bit);
    ctx.memory.write(0xff0f, modified_flag);
    ctx.timers.is_halted = false;
}


#[allow(non_camel_case_types)]
pub enum Action {
  new_mode(LcdMode),
  request_interrupt(u8),
  ime1,
  reload_tima(bool),
}

#[derive(Default)]
pub struct GeneralInterrupts {
  mode: Option<LcdMode>,
  request_interrupt: Option<u8>,
  ime1: bool,
  reload_tima: bool
}

impl GeneralInterrupts {
  pub fn run(ctx: &mut Emulator) {
    if let Some(mode) = ctx.interrupts.mode {
      ctx.memory.set_lcd_status(mode);
      ctx.interrupts.mode = None;
    }

    if let Some(bit) = ctx.interrupts.request_interrupt {
      request_interrupt(ctx, bit);
      ctx.interrupts.request_interrupt = None;
    }

    if ctx.interrupts.ime1 {
      ctx.timers.ime = true;
      ctx.interrupts.ime1 = false;
    }

    if ctx.interrupts.reload_tima && ctx.memory.get_tima() == 0 {
      ctx.memory.set_tima(ctx.memory.get_tma());
      request_interrupt(ctx, Interrupts::Timer as u8);
      ctx.interrupts.dispatch(Action::reload_tima(false));
      // ignores tima writes in memory
      ctx.memory.tima_reloading = true;
      ctx.interrupts.reload_tima = false;
    } else {
      ctx.memory.tima_reloading = false;
    }
  }

  pub fn dispatch(&mut self, action: Action) {
    match action {
      Action::new_mode(mode) => self.mode = Some(mode),
      Action::request_interrupt(bit) => self.request_interrupt = Some(bit),
      Action::ime1 => self.ime1 = true,
      Action::reload_tima(value) => self.reload_tima = value,
    }
  }
}
