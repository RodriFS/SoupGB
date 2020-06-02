#[derive(Default)]
pub struct Interrupts {
    master_enabled: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Self { master_enabled: 0 }
    }

    pub fn clear_master_enabled(&mut self) {
        self.master_enabled = 0;
    }
}
