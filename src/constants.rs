pub const CLOCKSPEED: u32 = 4_194_304;
pub const FPS: u32 = 60;
pub const MAXCYCLES: u32 = CLOCKSPEED / FPS;

pub const TIMER_ADDRESS: u16 = 0xff05;
pub const TIMER_CONTROLLER_ADDRESS: u16 = 0xff07;
pub const TIMER_MODULATOR_ADDRESS: u16 = 0xff06;
