use super::constants::*;

pub struct Timers {
  clock_frequency: u32,
  timer_counter: u32,
}

impl Timers {
  pub fn new() -> Self {
    let clock_frequency = 4096;
    Self {
      clock_frequency,
      timer_counter: CLOCKSPEED / clock_frequency,
    }
  }
  pub fn update(&self, frame_cycles: u32) {
    // DoDividerRegister(cycles);

    // // the clock must be enabled to update the clock
    // if (IsClockEnabled()) {
    //   m_TimerCounter -= cycles;

    //   // enough cpu clock cycles have happened to update the timer
    //   if (m_TimerCounter <= 0) {
    //     // reset m_TimerTracer to the correct value
    //     SetClockFreq();

    //     // timer about to overflow
    //     if (ReadMemory(TIMA) == 255) {
    //       WriteMemory(TIMA, ReadMemory(TMA));
    //       RequestInterupt(2);
    //     } else {
    //       WriteMemory(TIMA, ReadMemory(TIMA) + 1);
    //     }
    //   }
    // }
  }
}
