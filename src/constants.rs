pub const P1_JOYPAD_ADDRESS: u16 = 0xff00;
pub const DIVIDER_COUNTER_ADDRESS: u16 = 0xff04; // DIV
pub const TIMER_COUNTER_ADDRESS: u16 = 0xff05; // TIMA
pub const TIMER_MODULO_ADDRESS: u16 = 0xff06; // TMA
pub const TIMER_CONTROL_ADDRESS: u16 = 0xff07; // TAC

pub const DEBUG_CPU: bool = true;
pub const DEBUG_MEMORY: bool = true;
pub const DEBUG_GPU: bool = false;
pub const DEBUG_TIMERS: bool = false;

pub const STEPS: bool = false;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
