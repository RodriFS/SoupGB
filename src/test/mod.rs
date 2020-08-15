#[cfg(test)]
use super::cpu;
#[allow(unused_imports)]
use super::memory::LcdMode;
#[allow(unused_imports)]
use super::utils::*;

#[test]
fn line_144_timing_test() {
  let boot = vec![0; 100];
  let rom = [boot, vec![0; 1000]].concat();

  let mut emulator = super::emulator::Emulator::default();
  emulator.load_rom(rom);

  while emulator.memory.get_ly() < 144 {
    cpu::update(&mut emulator);
  }
  // clock 0
  assert_eq!(emulator.memory.get_ly(), 144);
  assert_eq!(emulator.memory.get_lcd_status(), LcdMode::HBlank);
  assert_eq!(get_bit_at(emulator.memory.read(0xff0f), 0), false);
  // clock 4
  cpu::update(&mut emulator);
  assert_eq!(emulator.memory.get_ly(), 144);
  assert_eq!(emulator.memory.get_lcd_status(), LcdMode::VBlank);
  assert_eq!(get_bit_at(emulator.memory.read(0xff0f), 0), true);
  // clock 8-452
  emulator.memory.write(0xffff, 0b1110_0001); // v-blank interrupt enabled
  emulator.timers.set_master_enabled_on();
  // 456 - 8 (previous 2 clocks) - 20 (interrupt takes 5 cycles)
  for _ in (0..428).step_by(4) {
    cpu::update(&mut emulator);
    assert_eq!(emulator.memory.get_ly(), 144);
    assert_eq!(emulator.memory.get_lcd_status(), LcdMode::VBlank);
    assert_eq!(get_bit_at(emulator.memory.read(0xff0f), 0), false);
  }
  // next line
  cpu::update(&mut emulator);
  assert_eq!(emulator.memory.get_ly(), 145);
}
