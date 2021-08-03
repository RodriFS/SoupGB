pub const P1_JOYPAD_ADDRESS: u16 = 0xff00;
pub const DIVIDER_COUNTER_ADDRESS: u16 = 0xff04; // DIV
pub const TIMER_COUNTER_ADDRESS: u16 = 0xff05; // TIMA
pub const TIMER_MODULO_ADDRESS: u16 = 0xff06; // TMA
pub const TIMER_CONTROL_ADDRESS: u16 = 0xff07; // TAC

pub const NR10: u16 = 0xff10;
pub const NR11: u16 = 0xff11;
pub const NR12: u16 = 0xff12;
pub const NR13: u16 = 0xff13;
pub const NR14: u16 = 0xff14;
pub const NR20: u16 = 0xff15;
pub const NR21: u16 = 0xff16;
pub const NR22: u16 = 0xff17;
pub const NR23: u16 = 0xff18;
pub const NR24: u16 = 0xff19;
pub const NR30: u16 = 0xff1a;
pub const NR31: u16 = 0xff1b;
pub const NR32: u16 = 0xff1c;
pub const NR33: u16 = 0xff1d;
pub const NR34: u16 = 0xff1e;
pub const NR40: u16 = 0xff1f;
pub const NR41: u16 = 0xff20;
pub const NR42: u16 = 0xff21;
pub const NR43: u16 = 0xff22;
pub const NR44: u16 = 0xff23;
pub const NR50: u16 = 0xff24;
pub const NR51: u16 = 0xff25;
pub const NR52: u16 = 0xff26;

pub const DEBUG_CPU: bool = false;
pub const DEBUG_MEMORY: bool = false;
pub const DEBUG_GPU: bool = false;
pub const DEBUG_TIMERS: bool = false;

pub const STEPS: bool = false;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
