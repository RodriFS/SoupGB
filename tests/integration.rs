use soup_gb::cpu;
use soup_gb::dispatcher::Action;
use soup_gb::emulator::Emulator;
use soup_gb::interrupts;
use soup_gb::memory::LcdMode;
use soup_gb::utils::*;

fn assert_pc_byte_and_sp(emulator: &mut Emulator, pc: u16, byte: u8, sp: u8) {
  assert_eq!(emulator.memory.get_pc(), pc);
  assert_eq!(emulator.memory.get_byte_debug(), byte);
  assert_eq!(emulator.memory.read(emulator.memory.get_sp()), sp);
}

fn run_cpu(ctx: &mut Emulator) {
  interrupts::update(ctx);
  cpu::update(ctx);
}

#[test]
fn line_0_timing_test() {
  // Mode 2 test
  let rom = vec![0; 0x200];

  let mut emulator = Emulator::default();
  emulator.load_rom(rom);

  while emulator.memory.get_ly() != 0 || emulator.memory.lcd_mode() != LcdMode::ReadOAM {
    run_cpu(&mut emulator);
  }
  assert_eq!(emulator.memory.get_ly(), 0);
  assert_eq!(emulator.memory.lcd_mode(), LcdMode::ReadOAM);
  assert_eq!(emulator.timers.scan_line_counter, 4);
  for _ in (4..80).step_by(4) {
    run_cpu(&mut emulator);
    assert_eq!(emulator.memory.get_ly(), 0);
    assert_eq!(emulator.memory.lcd_mode(), LcdMode::ReadOAM);
  }
  run_cpu(&mut emulator);
  assert_eq!(emulator.memory.get_ly(), 0);
  assert_eq!(emulator.memory.lcd_mode(), LcdMode::ReadVRAM);
}

#[test]
fn line_144_timing_test() {
  let rom = vec![0; 0x200];

  let mut emulator = Emulator::default();
  emulator.load_rom(rom);

  while emulator.memory.get_ly() < 144 {
    run_cpu(&mut emulator);
  }
  // clock 0
  emulator.memory.write(0xffff, 0b1110_0001); // v-blank interrupt enabled
  emulator.dispatcher.dispatch(Action::ime1); // ime enabled next cycle
  assert!(!emulator.timers.ime);
  assert_eq!(emulator.memory.get_ly(), 144);
  assert_eq!(emulator.memory.lcd_mode(), LcdMode::HBlank);
  assert!(!get_bit_at(emulator.memory.read(0xff0f), 0));
  // clock 4
  run_cpu(&mut emulator);
  assert!(emulator.timers.ime);
  assert_eq!(emulator.memory.get_ly(), 144);
  assert_eq!(emulator.memory.lcd_mode(), LcdMode::VBlank);
  assert!(get_bit_at(emulator.memory.read(0xff0f), 0));
  // clock 8-452
  // 456 - 8 (previous 2 clocks) - 20 (interrupt takes 5 cycles)
  for _ in (0..428).step_by(4) {
    run_cpu(&mut emulator);
    assert_eq!(emulator.memory.get_ly(), 144);
    assert_eq!(emulator.memory.lcd_mode(), LcdMode::VBlank);
    assert!(!get_bit_at(emulator.memory.read(0xff0f), 0));
  }
  // next line
  run_cpu(&mut emulator);
  assert_eq!(emulator.memory.get_ly(), 145);
}

#[test]
fn halt_ime_1() {
  let boot = vec![0; 0x100];
  let rom = [boot, vec![0xfb, 0x00, 0x76, 0x3c, 0x04], vec![0; 0x47]].concat();

  let mut emulator = Emulator::default();
  emulator.load_rom(rom);

  // IME=1
  assert_pc_byte_and_sp(&mut emulator, 0x100, 0xfb, 0x00);
  run_cpu(&mut emulator);
  assert!(!emulator.timers.ime); // Master enabled in next cycle

  assert_pc_byte_and_sp(&mut emulator, 0x101, 0x00, 0x00);
  run_cpu(&mut emulator);
  assert!(emulator.timers.ime); // Master enabled here

  assert_pc_byte_and_sp(&mut emulator, 0x102, 0x76, 0x00);
  run_cpu(&mut emulator);

  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  // Enable IE for VBlank
  emulator.memory.write(0xffff, 0b0000_0001);
  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  // Enable IF for VBlank
  emulator.memory.write(0xff0f, 0b0000_0001);
  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  assert_eq!(emulator.memory.get_pc(), 0x40);
  assert!(!emulator.timers.is_halted);
  assert_eq!(emulator.memory.read(0xff0f) & 0b0000_0001, 0); // IF cleared
  assert_eq!(emulator.memory.read(emulator.memory.get_sp() + 1), 0x1); // Instruction next to HALT (hi)
  assert_eq!(emulator.memory.read(emulator.memory.get_sp()), 0x03); // Instruction next to HALT (lo)
}

#[test]
fn halt_ime_0() {
  let boot = vec![0; 0x100];
  let rom = [boot, vec![0xf3, 0x00, 0x76, 0x3c, 0x04], vec![0; 0x47]].concat();

  let mut emulator = Emulator::default();
  emulator.load_rom(rom);

  // IME=0
  assert_pc_byte_and_sp(&mut emulator, 0x100, 0xf3, 0x00);
  run_cpu(&mut emulator);
  assert!(!emulator.timers.ime); // Master disabled in next cycle

  assert_pc_byte_and_sp(&mut emulator, 0x101, 0x00, 0x00);
  run_cpu(&mut emulator);
  assert!(!emulator.timers.ime); // Master disabled here

  assert_pc_byte_and_sp(&mut emulator, 0x102, 0x76, 0x00);
  run_cpu(&mut emulator);

  // HALT mode entered
  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  // Enable IE for VBlank
  emulator.memory.write(0xffff, 0b0000_0001);
  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  // Enable IF for VBlank
  emulator.memory.write(0xff0f, 0b0000_0001);
  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert!(emulator.timers.is_halted);
  run_cpu(&mut emulator);

  assert!(!emulator.timers.is_halted);
  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  assert_eq!(emulator.memory.read(0xff0f) & 0b0000_0001, 1); // IF not cleared
  run_cpu(&mut emulator);

  // Halt bug
  assert!(!emulator.timers.is_halted);
  assert_pc_byte_and_sp(&mut emulator, 0x103, 0x3c, 0x00);
  run_cpu(&mut emulator);

  assert!(!emulator.timers.is_halted);
  assert_pc_byte_and_sp(&mut emulator, 0x104, 0x04, 0x00);
}
