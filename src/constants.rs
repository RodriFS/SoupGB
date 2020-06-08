pub const CLOCKSPEED: u32 = 4_194_304;
pub const FPS: u32 = 60;
pub const MAXCYCLES: u32 = CLOCKSPEED / FPS;

pub const DIVIDER_COUNTER_ADDRESS: u16 = 0xff04;
pub const TIMER_COUNTER_ADDRESS: u16 = 0xff05;
pub const TIMER_CONTROL_ADDRESS: u16 = 0xff07;
pub const TIMER_MODULO_ADDRESS: u16 = 0xff06;

pub const DEBUG: bool = true;
pub const STEPS: bool = false;
