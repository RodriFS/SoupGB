use super::interrupts::Interrupts;
use super::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Gpu {
  scan_line_counter: u32,
  lcd_enabled: bool,
  memory: Rc<RefCell<Memory>>,
  interrupts: Rc<RefCell<Interrupts>>,
}

impl Gpu {
  pub fn new(memory: Rc<RefCell<Memory>>, interrupts: Rc<RefCell<Interrupts>>) -> Self {
    Self {
      scan_line_counter: 0,
      lcd_enabled: true,
      memory,
      interrupts,
    }
  }

  fn set_lcd_status(&self) {}

  fn draw_scan_line(&self) {}

  pub fn update(&mut self, frame_cycles: u32) {
    self.set_lcd_status();

    if self.lcd_enabled {
      self.scan_line_counter += frame_cycles;
    } else {
      return;
    }

    if self.scan_line_counter > 456 {
      let scan_line = self.memory.borrow().read(0xFF44);
      self
        .memory
        .borrow_mut()
        .write(0xFF44, scan_line.wrapping_add(1));

      // BYTE currentline = ReadMemory(0xFF44) ;
      self.scan_line_counter = 0;

      if scan_line == 144 {
        self.interrupts.borrow_mut().request_interrupt(0);
      } else if scan_line > 153 {
        self.memory.borrow_mut().write(0xFF44, 0);
      } else if scan_line < 144 {
        self.draw_scan_line();
      }
    }
  }
}
