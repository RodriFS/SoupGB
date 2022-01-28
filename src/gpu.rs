use super::interrupts::Action;
use super::emulator::Emulator;
use super::interrupts::{stat_irq, Interrupts, StatCond};
use super::memory::{LcdMode, PrevStatCond};
use super::ppu::draw_scan_line;

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
                ctx.interrupts.dispatch(Action::new_mode(LcdMode::ReadVRAM));
            }
        }
        // mode 3
        LcdMode::ReadVRAM => {
            if ctx.timers.scan_line_counter >= 172 {
                // go to mode 0
                ctx.timers.scan_line_counter = 0;
                ctx.interrupts.dispatch(Action::new_mode(LcdMode::HBlank));
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
                    ctx.interrupts.dispatch(Action::new_mode(LcdMode::VBlank));
                    ctx.interrupts
                        .dispatch(Action::request_interrupt(Interrupts::VBlank as u8));
                    stat_int_requested = StatCond::or(
                        stat_irq(ctx, StatCond::VBlank),
                        stat_irq(ctx, StatCond::OAM),
                    );
                } else {
                    // go to mode 2
                    ctx.interrupts.dispatch(Action::new_mode(LcdMode::ReadOAM));
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
                    ctx.interrupts.dispatch(Action::new_mode(LcdMode::ReadOAM));
                    stat_int_requested = stat_irq(ctx, StatCond::OAM);
                }
                _ => {}
            }
        }
    };

    if stat_int_requested.is_stat() && check_stat_conditions(ctx, &stat_int_requested) {
        ctx.interrupts
            .dispatch(Action::request_interrupt(Interrupts::LCDStat as u8));
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

pub fn update(ctx: &mut Emulator) {
    if !ctx.memory.is_lcd_enabled() {
        ctx.timers.scan_line_counter = 0;
        ctx.memory.write_ly(0x00);
        ctx.memory.set_lcd_status(LcdMode::VBlank); // Check
        return;
    }
    ctx.timers.scan_line_counter += 4;
    set_lcd_mode(ctx);
}
