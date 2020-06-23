use super::constants::*;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};
use byteorder::{BigEndian, ByteOrder};
use std::io::Write;

#[derive(PartialEq)]
pub enum LcdMode {
    HBlank,
    VBlank,
    ReadingOAMRAM,
    TransfToLCDDriver,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum MBC {
    ROM_ONLY,
    MBC1,
    MBC2,
}

#[derive(Debug, Clone)]
pub enum Bmode {
    RAM,
    ROM,
}

pub struct Memory {
    pub cartridge_memory: Vec<u8>,
    pub internal_memory: [u8; 0x10000],
    pub ram_memory: [u8; 0x8000],
    pub current_rom_bank: u8,
    pub current_ram_bank: u8,
    pub memory_bank_type: MBC,
    pub is_ram_enabled: bool,
    pub banking_mode: Bmode,
    stack_pointer: u16,
    program_counter: u16,
    pub input_clock_select: u32,
}

impl Memory {
    pub fn default() -> Self {
        let mut internal_memory = [0; 0x10000];
        internal_memory[0xFF10] = 0x80;
        internal_memory[0xFF11] = 0xBF;
        internal_memory[0xFF12] = 0xF3;
        internal_memory[0xFF14] = 0xBF;
        internal_memory[0xFF16] = 0x3F;
        internal_memory[0xFF19] = 0xBF;
        internal_memory[0xFF1A] = 0x7F;
        internal_memory[0xFF1B] = 0xFF;
        internal_memory[0xFF1C] = 0x9F;
        internal_memory[0xFF1E] = 0xBF;
        internal_memory[0xFF20] = 0xFF;
        internal_memory[0xFF23] = 0xBF;
        internal_memory[0xFF24] = 0x77;
        internal_memory[0xFF25] = 0xF3;
        internal_memory[0xFF26] = 0xF1;
        internal_memory[0xFF40] = 0x91;
        internal_memory[0xFF47] = 0xFC;
        internal_memory[0xFF48] = 0xFF;
        internal_memory[0xFF49] = 0xFF;

        internal_memory[0xFF41] = 0x84;
        Self {
            memory_bank_type: MBC::ROM_ONLY,
            current_rom_bank: 1,
            cartridge_memory: Vec::new(),
            internal_memory,
            ram_memory: [0; 0x8000],
            current_ram_bank: 0,
            is_ram_enabled: false,
            banking_mode: Bmode::ROM,
            stack_pointer: 0xfffe,
            program_counter: 0x100,
            input_clock_select: 1024,
        }
    }

    pub fn get_next_8(&mut self) -> u8 {
        let data = self.read(self.get_program_counter());
        self.increment_program_counter(1);
        data
    }
    pub fn get_next_16(&mut self) -> u16 {
        let c = self.get_program_counter() as usize;
        self.increment_program_counter(2);
        let address = self.read_range(c..(c + 2));
        BigEndian::read_u16(&[address[0], address[1]]).swap_bytes()
    }

    pub fn get_next_8_debug(&self) -> u8 {
        self.read_memory_at_current_location()
    }

    pub fn write_u16(&mut self, address: u16, data: u16) {
        let bytes = data.to_be_bytes();
        self.write(address, bytes[1]);
        self.write(address.wrapping_add(1), bytes[0]);
    }

    pub fn get_next_16_debug(&self) -> u16 {
        let c = self.get_program_counter() as usize;
        let address = self.read_range((c + 1)..(c + 3));
        BigEndian::read_u16(&[address[0], address[1]])
    }

    pub fn read_memory_at_current_location(&self) -> u8 {
        self.read(self.get_program_counter())
    }

    pub fn load_rom(&mut self, cartridge_memory: Vec<u8>) {
        self.internal_memory[0x0000..0x7FFF].clone_from_slice(&cartridge_memory[0x0000..0x7FFF]);
        self.memory_bank_type = match cartridge_memory[0x147] {
            1 | 2 | 3 => MBC::MBC1,
            5 | 6 => MBC::MBC2,
            _ => MBC::ROM_ONLY,
        };
        self.cartridge_memory = cartridge_memory;
    }

    fn set_is_ram_enabled(&mut self, value: bool) {
        self.is_ram_enabled = value;
    }

    fn set_rom_bank(&mut self, bank: u8) {
        self.current_rom_bank = bank;
    }

    fn set_ram_bank(&mut self, bank: u8) {
        self.current_ram_bank = bank;
    }

    fn set_banking_mode(&mut self, mode: Bmode) {
        self.banking_mode = mode;
    }

    fn set_ram(&mut self, address: u16, data: u8) {
        self.ram_memory[address as usize] = data;
    }

    pub fn set_rom(&mut self, address: u16, data: u8) {
        self.internal_memory[address as usize] = data;
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x4000..=0x7FFF => {
                let mb_address = address - 0x4000;
                self.cartridge_memory
                    .get((mb_address + (self.current_rom_bank as u16 * 0x4000)) as usize)
                    .unwrap()
                    .to_owned()
            }
            0xA000..=0xBFFF => {
                if !self.is_ram_enabled {
                    return 0xff;
                }
                let ram_address = address - 0xA000;
                self.ram_memory[(ram_address + (self.current_ram_bank as u16 * 0x2000)) as usize]
            }
            _ => self.internal_memory[address as usize],
        }
    }

    pub fn read_range(&self, range: std::ops::Range<usize>) -> &[u8] {
        self.internal_memory.get(range).unwrap()
    }

    pub fn push_to_stack(&mut self, data: u16) {
        self.decrement_stack_pointer(2);
        let bytes = data.to_be_bytes();
        self.write(self.stack_pointer, bytes[1]);
        self.write(self.stack_pointer.wrapping_add(1), bytes[0]);
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let byte1 = self.read(self.stack_pointer);
        let byte2 = self.read(self.stack_pointer.wrapping_add(1));
        self.increment_stack_pointer(2);
        (byte2 as u16) << 8 | byte1 as u16
    }

    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn set_stack_pointer(&mut self, address: u16) {
        self.stack_pointer = address;
    }

    pub fn increment_program_counter(&mut self, increment: u16) {
        self.program_counter = self.program_counter.wrapping_add(increment);
    }

    pub fn increment_stack_pointer(&mut self, increment: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_add(increment);
    }

    pub fn decrement_stack_pointer(&mut self, decrement: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(decrement);
    }

    pub fn add_to_program_counter(&mut self, addition: u16) -> u16 {
        self.program_counter.wrapping_add(addition)
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn get_stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    pub fn write_scanline(&mut self, data: u8) {
        self.set_rom(0xff44, data);
    }

    pub fn handle_bank_type(&mut self, address: u16, data: u8) {
        match self.memory_bank_type {
            MBC::ROM_ONLY if address > 0x8000 => {
                panic!("Trying to write to address greater than 0x8000")
            }
            MBC::MBC2 if get_bit_at(address.to_be_bytes()[1], 4) => {}
            MBC::MBC1 | MBC::MBC2 if address <= 0x1fff => match data & 0xf {
                0x0a => self.set_is_ram_enabled(true),
                0x00 => self.set_is_ram_enabled(false),
                _ => {}
            },
            MBC::MBC1 if (address >= 0x2000) && (address <= 0x3fff) => {
                let test = (self.current_rom_bank & 224) | (data & 31);
                if test == 0 {
                    self.set_rom_bank(1);
                } else {
                    self.set_rom_bank(test);
                }
            }
            MBC::MBC2 if (address >= 0x2000) && (address <= 0x3fff) => {
                let lower_bits = data & 0xf;
                if lower_bits == 0 {
                    self.set_rom_bank(1);
                } else {
                    self.set_rom_bank(lower_bits);
                }
            }
            MBC::MBC1 if (address >= 0x4000) && (address <= 0x5fff) => match self.banking_mode {
                Bmode::ROM => {
                    let lower_bits = self.current_rom_bank & 0xe1;
                    let upper_bits = data & 0x1f;
                    let next_rom_bank = upper_bits | lower_bits;
                    if next_rom_bank == 0 {
                        self.set_rom_bank(1);
                    }
                    self.set_rom_bank(next_rom_bank);
                }
                Bmode::RAM => {
                    self.set_ram_bank(data & 0x3);
                }
            },
            MBC::MBC1 if (address >= 0x6000) && (address <= 0x7FFF) => match data & 0x1 {
                0x00 => {
                    self.set_banking_mode(Bmode::ROM);
                    self.set_rom_bank(0);
                }
                0x01 => self.set_banking_mode(Bmode::RAM),
                _ => panic!("Unsupported banking mode"),
            },
            _ => panic!("MBC case not supported"),
        };
    }

    pub fn get_div(&self) -> u8 {
        self.read(DIVIDER_COUNTER_ADDRESS)
    }
    pub fn set_div(&mut self, data: u8) {
        self.set_rom(DIVIDER_COUNTER_ADDRESS, data);
    }
    pub fn get_tima(&self) -> u8 {
        self.read(TIMER_COUNTER_ADDRESS)
    }
    pub fn set_tima(&mut self, counter: u8) {
        self.write(TIMER_COUNTER_ADDRESS, counter);
    }
    pub fn get_tma(&self) -> u8 {
        self.read(TIMER_MODULO_ADDRESS)
    }
    pub fn get_tac(&self) -> u8 {
        self.read(TIMER_CONTROL_ADDRESS)
    }
    pub fn get_is_clock_enabled(&self) -> bool {
        let timers = self.get_tac();
        timers >> 2 == 1
    }

    pub fn update_div(&mut self) {
        let divider_register = self.get_div();
        if divider_register == 0xff {
            self.set_div(0x0);
        } else {
            self.set_div(divider_register + 1);
        }
    }

    pub fn get_clock_frequency(&self) -> u8 {
        self.get_tac() & 0x3
    }

    pub fn set_clock_frequency(&mut self, bits: u8) {
        match bits {
            0 => self.input_clock_select = 1024, // freq 4096
            1 => self.input_clock_select = 16,   // freq 262144
            2 => self.input_clock_select = 64,   // freq 65536
            3 => self.input_clock_select = 256,  // freq 16382
            _ => panic!("Frequency not supported"),
        }
    }

    pub fn increment_scanline(&mut self) -> u8 {
        let mut scan_line = self.read(0xff44);
        scan_line = scan_line.wrapping_add(1);
        self.write_scanline(scan_line);
        scan_line
    }

    pub fn is_lcd_enabled(&self) -> bool {
        get_bit_at(self.read(0xff40), 7)
    }

    pub fn get_lcd_status(&self) -> LcdMode {
        let lcd_status = self.read(0xff41);
        match lcd_status & 0x3 {
            0x0 => LcdMode::HBlank,
            0x1 => LcdMode::VBlank,
            0x2 => LcdMode::ReadingOAMRAM,
            0x3 => LcdMode::TransfToLCDDriver,
            _ => panic!("Unreachable lcd status"),
        }
    }

    pub fn set_lcd_status(&mut self, status: LcdMode) {
        let lcd_status = self.read(0xff41);
        let new_status = match status {
            LcdMode::HBlank => {
                let temp_status = clear_bit_at(lcd_status, 1);
                clear_bit_at(temp_status, 0)
            }
            LcdMode::VBlank => {
                let temp_status = clear_bit_at(lcd_status, 1);
                set_bit_at(temp_status, 0)
            }
            LcdMode::ReadingOAMRAM => {
                let temp_status = set_bit_at(lcd_status, 1);
                clear_bit_at(temp_status, 0)
            }
            LcdMode::TransfToLCDDriver => {
                let temp_status = set_bit_at(lcd_status, 1);
                set_bit_at(temp_status, 0)
            }
        };
        self.write(0xff41, new_status);
    }

    pub fn is_interrupt_requested(&self, bit: u8) -> bool {
        let lcd_status = self.read(0xff41);
        get_bit_at(lcd_status, bit)
    }

    pub fn set_coincidence_flag(&mut self) {
        let lcd_status = self.read(0xff41);
        self.write(0xff41, set_bit_at(lcd_status, 2));
    }

    pub fn clear_coincidence_flag(&mut self) {
        let lcd_status = self.read(0xff41);
        self.write(0xff41, clear_bit_at(lcd_status, 2));
    }

    pub fn get_ly(&self) -> u8 {
        self.read(0xff44)
    }

    pub fn get_lyc(&self) -> u8 {
        self.read(0xff45)
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7fff => self.handle_bank_type(address, data),
            0xa000..=0xbfff if self.is_ram_enabled => {
                let bank_address = address - 0xa000;
                self.set_ram(bank_address + self.current_ram_bank as u16 * 0x2000, data);
            }
            0xe000..=0xfdff => {
                self.set_rom(address, data);
                self.write(address - 0x2000, data);
            }
            0xfea0..=0xfefe => {}
            0xff01 => {
                self.set_rom(address, data);
                self.set_rom(0xff02, 0x81);
                let c = self.internal_memory[0xff01] as char;
                let mut out = std::io::stdout();
                print!("{}", c);
                let _ = out.flush();
            }
            0xff04 => self.set_rom(0xff04, 0),
            0xff07 => {
                self.set_clock_frequency(data & 0x3);
                self.set_rom(address, data)
            }
            0xff44 => self.set_rom(address, 0),
            _ => self.set_rom(address, data),
        }
    }
}
