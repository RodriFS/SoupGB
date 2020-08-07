use super::constants::*;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};
use byteorder::{BigEndian, ByteOrder};
use std::io::Write;

pub struct Point2D {
    pub x: u8,
    pub y: u8,
}

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

#[derive(PartialEq, Debug, Clone)]
pub enum Bmode {
    RAM,
    ROM,
}

pub struct Memory {
    pub cartridge_memory: Vec<u8>,
    wram: [u8; 0x2000],
    vram: [u8; 0x2000],
    ram: [u8; 0x8000],
    echo: [u8; 0x1e00],
    oam: [u8; 0xa0],
    io_ports: [u8; 0x80],
    hram: [u8; 0x80],
    ie_register: u8,
    pub memory_bank: u8,
    pub wram_bank: u8,
    pub memory_bank_type: MBC,
    rom_size: u8,
    ram_size: u8,
    pub is_ram_enabled: bool,
    pub banking_mode: Bmode,
    pub stack_pointer: u16,
    program_counter: u16,
    pub input_clock_select: u32,
}

// General Initialization functions
impl Memory {
    pub fn default() -> Self {
        let mut io_ports = [0; 0x80];
        // DIV & TIMA
        io_ports[0x04] = 0xab;
        io_ports[0x05] = 0xcc;

        io_ports[0x10] = 0x80;
        io_ports[0x11] = 0xBF;
        io_ports[0x12] = 0xF3;
        io_ports[0x14] = 0xBF;
        io_ports[0x16] = 0x3F;
        io_ports[0x19] = 0xBF;
        io_ports[0x1A] = 0x7F;
        io_ports[0x1B] = 0xFF;
        io_ports[0x1C] = 0x9F;
        io_ports[0x1E] = 0xBF;
        io_ports[0x20] = 0xFF;
        io_ports[0x23] = 0xBF;
        io_ports[0x24] = 0x77;
        io_ports[0x25] = 0xF3;
        io_ports[0x26] = 0xF1;
        io_ports[0x40] = 0x91;
        io_ports[0x47] = 0xFC;
        io_ports[0x48] = 0xFF;
        io_ports[0x49] = 0xFF;
        io_ports[0x41] = 0x84;

        Self {
            wram: [0; 0x2000],
            vram: [0; 0x2000],
            ram: [0; 0x8000],
            echo: [0; 0x1e00],
            oam: [0; 0xa0],
            hram: [0; 0x80],
            ie_register: 0,
            memory_bank_type: MBC::ROM_ONLY,
            memory_bank: 1,
            rom_size: 0,
            ram_size: 0,
            cartridge_memory: Vec::new(),
            wram_bank: 1,
            io_ports,
            is_ram_enabled: false,
            banking_mode: Bmode::ROM,
            stack_pointer: 0xfffe,
            program_counter: 0x100,
            input_clock_select: 1024,
        }
    }
}

// General CPU functions
impl Memory {
    pub fn get_byte(&mut self) -> u8 {
        let data = self.read(self.get_program_counter());
        self.increment_program_counter(1);
        data
    }
    pub fn get_word(&mut self) -> u16 {
        let c = self.get_program_counter();
        self.increment_program_counter(2);
        BigEndian::read_u16(&[self.read(c + 1), self.read(c)])
    }

    pub fn get_byte_debug(&self) -> u8 {
        self.read_memory_at_current_location()
    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        let bytes = data.to_be_bytes();
        self.write(address, bytes[1]);
        self.write(address.wrapping_add(1), bytes[0]);
    }

    pub fn get_word_debug(&self) -> u16 {
        let c = self.get_program_counter();
        BigEndian::read_u16(&[self.read(c + 1), self.read(c)])
    }

    pub fn read_memory_at_current_location(&self) -> u8 {
        self.read(self.get_program_counter())
    }

    pub fn load_rom(&mut self, cartridge_memory: Vec<u8>) {
        self.memory_bank_type = match cartridge_memory[0x147] {
            1 | 2 | 3 => MBC::MBC1,
            5 | 6 => MBC::MBC2,
            _ => MBC::ROM_ONLY,
        };
        self.rom_size = (cartridge_memory[0x148] << 4) + 32;
        self.ram_size = match cartridge_memory[0x149] {
            0 => 0,
            1 => 2,
            2 => 8,
            3 => 32,
            _ => panic!("Unsupported ram size"),
        };
        if let Some(value) = cartridge_memory.get(0xff70) {
            self.wram_bank = *value;
        }
        self.cartridge_memory = cartridge_memory;
    }

    fn set_is_ram_enabled(&mut self, value: bool) {
        self.is_ram_enabled = value;
    }

    fn set_banking_mode(&mut self, mode: Bmode) {
        self.banking_mode = mode;
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
        self.io_ports[0x44] = data;
    }
}

// General Timer functions
impl Memory {
    pub fn get_div(&self) -> u8 {
        self.read(DIVIDER_COUNTER_ADDRESS)
    }
    pub fn set_div(&mut self, data: u8) {
        self.io_ports[DIVIDER_COUNTER_ADDRESS as usize - 0xff00] = data;
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
        self.read(TIMER_CONTROL_ADDRESS) & 0b0000_0111
    }
    pub fn get_is_clock_enabled(&self) -> bool {
        let timers = self.get_tac();
        timers >> 2 == 1
    }

    pub fn update_div(&mut self) {
        let divider_register = self.get_div().wrapping_add(1);
        self.set_div(divider_register);
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
}

// General Gpu functions
impl Memory {
    pub fn background_position(&self) -> Point2D {
        let (x, y) = if self.background_enabled() {
            (self.read(0xff43), self.read(0xff42))
        } else {
            (0, 0)
        };
        Point2D { x, y }
    }

    pub fn window_position(&self) -> Point2D {
        Point2D {
            x: self.read(0xff40) - 7,
            y: self.read(0xff4a),
        }
    }

    pub fn is_lcd_enabled(&self) -> bool {
        get_bit_at(self.read(0xff40), 7)
    }

    fn window_map_select(&self) -> u16 {
        if get_bit_at(self.read(0xff40), 6) {
            return 0x9c00;
        }
        0x9800
    }

    pub fn window_enabled(&self) -> bool {
        get_bit_at(self.read(0xff40), 5)
    }

    pub fn bg_tile_data_select(&self) -> u16 {
        if get_bit_at(self.read(0xff40), 4) {
            return 0x8000;
        }
        0x8800
    }

    fn background_map_select(&self) -> u16 {
        if get_bit_at(self.read(0xff40), 3) {
            return 0x9c00;
        }
        0x9800
    }

    pub fn map_select(&self) -> u16 {
        let w_pos = self.window_position();
        if self.window_enabled() && self.get_ly() >= w_pos.y {
            self.window_map_select()
        } else {
            self.background_map_select()
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if get_bit_at(self.read(0xff40), 2) {
            return 16;
        }
        8
    }

    pub fn sprite_enabled(&self) -> bool {
        get_bit_at(self.read(0xff40), 1)
    }

    pub fn background_enabled(&self) -> bool {
        get_bit_at(self.read(0xff40), 0)
    }

    pub fn background_palette(&self) -> u8 {
        self.read(0xff47)
    }

    pub fn sprite_palette1(&self) -> u8 {
        self.read(0xff48)
    }

    pub fn sprite_palette2(&self) -> u8 {
        self.read(0xff49)
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

    pub fn set_coincidence_flag(&mut self) {
        let lcd_status = self.read(0xff41);
        self.write(0xff41, set_bit_at(lcd_status, 2));
    }

    pub fn clear_coincidence_flag(&mut self) {
        let lcd_status = self.read(0xff41);
        self.write(0xff41, clear_bit_at(lcd_status, 2));
    }

    pub fn increment_scanline(&mut self) -> u8 {
        let mut scan_line = self.read(0xff44);
        scan_line = scan_line.wrapping_add(1);
        self.write_scanline(scan_line);
        scan_line
    }

    pub fn get_ly(&self) -> u8 {
        self.read(0xff44)
    }

    pub fn get_lyc(&self) -> u8 {
        self.read(0xff45)
    }

    fn dma_transfer(&mut self, data: u8) {
        let address = (data as u16) << 8;
        for i in 0..0xA0 {
            self.write(0xfe00 + i, self.read(address + i));
        }
    }
}

// General Interrupts functions
impl Memory {
    pub fn is_interrupt_requested(&self, bit: u8) -> bool {
        let lcd_status = self.read(0xff41);
        get_bit_at(lcd_status, bit)
    }
    pub fn interrupt_execution(&mut self, request: u8, interrupt: u8) {
        let clear_request = clear_bit_at(request, interrupt);
        self.write(0xff0f, clear_request);
        let pc = self.get_program_counter();
        self.push_to_stack(pc);
        let pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => return,
        };
        self.set_program_counter(pc);
    }
}

// Memory Read/Write functions
impl Memory {
    pub fn get_bank2_as_low(&self) -> u8 {
        (self.memory_bank & 0b0110_0000) >> 5
    }

    pub fn get_bank2_as_high(&self) -> u8 {
        self.memory_bank & 0b0110_0000
    }

    fn set_bank1(&mut self, data: u8) {
        let lower_bits = data & 0b0001_1111;
        let upper_bits = self.memory_bank & 0b0110_0000;
        let rom_bank = lower_bits | upper_bits;
        if lower_bits == 0 {
            self.memory_bank = rom_bank + 1;
        } else {
            self.memory_bank = rom_bank;
        }
    }

    fn set_bank2(&mut self, data: u8) {
        let lower_bits = self.memory_bank & 0b0001_1111;
        let upper_bits = (data & 0b0000_0011) << 5;
        let next_rom_bank = upper_bits | lower_bits;
        self.memory_bank = next_rom_bank;
    }

    fn write_io_ports(&mut self, address: u16, data: u8) {
        self.io_ports[address as usize - 0xff00] = data;
    }

    fn read_io_ports(&self, address: u16) -> u8 {
        self.io_ports[address as usize - 0xff00]
    }

    fn read_rom(&self, address: u16, bank: u8) -> u8 {
        let mut rom_address = address;
        if address > 0x3fff {
            rom_address -= 0x4000;
        }
        self.cartridge_memory
            .get((rom_address as u32 + (bank as u32 * 0x4000)) as usize)
            .unwrap_or(&0x0)
            .to_owned()
    }

    fn read_vram(&self, address: u16) -> u8 {
        let vram_address = address - 0x8000;
        self.vram[vram_address as usize]
    }

    fn read_echo(&self, address: u16) -> u8 {
        let echo_address = address - 0xe000;
        self.echo[echo_address as usize]
    }

    fn write_vram(&mut self, address: u16, data: u8) {
        let vram_address = address - 0x8000;
        self.vram[vram_address as usize] = data;
    }

    fn read_ram(&self, address: u16, bank: u8) -> u8 {
        let ram_address = address - 0xA000;
        self.ram[(ram_address + (bank as u16 * 0x2000)) as usize]
    }

    fn write_ram(&mut self, address: u16, bank: u8, data: u8) {
        let bank_address = address - 0xa000;
        self.ram[(bank_address + (bank as u16 * 0x2000)) as usize] = data;
    }

    fn read_wram(&self, address: u16) -> u8 {
        let bank_address = address - 0xc000;
        self.wram[bank_address as usize]
    }

    fn write_wram(&mut self, address: u16, data: u8) {
        let bank_address = address - 0xc000;
        self.wram[bank_address as usize] = data;
        if address >= 0xC000 && address <= 0xDDFF {
            self.echo[bank_address as usize] = data;
        }
    }

    fn read_oam(&self, address: u16) -> u8 {
        self.oam[(address - 0xFE00) as usize]
    }

    fn write_oam(&mut self, address: u16, data: u8) {
        self.oam[(address - 0xFE00) as usize] = data;
    }

    fn read_hram(&self, address: u16) -> u8 {
        self.hram[(address - 0xff80) as usize]
    }

    fn write_hram(&mut self, address: u16, data: u8) {
        self.hram[(address - 0xff80) as usize] = data;
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff if self.banking_mode == Bmode::ROM => self.read_rom(address, 0),
            0x0000..=0x3fff if self.banking_mode == Bmode::RAM => {
                self.read_rom(address, self.get_bank2_as_high())
            }
            0x4000..=0x7fff => self.read_rom(address, self.memory_bank),
            0x8000..=0x9fff => self.read_vram(address),
            0xa000..=0xbfff if !self.is_ram_enabled => 0xff,
            0xa000..=0xbfff if self.banking_mode == Bmode::ROM => self.read_ram(address, 0),
            0xa000..=0xbfff if self.banking_mode == Bmode::RAM => {
                self.read_ram(address, self.get_bank2_as_low())
            }
            0xc000..=0xdfff => self.read_wram(address),
            0xe000..=0xfdff => self.read_echo(address),
            0xfe00..=0xfe9f => self.read_oam(address),
            0xfea0..=0xfeff => 0,
            0xff00..=0xff7f => self.read_io_ports(address),
            0xff80..=0xfffe => self.read_hram(address),
            0xffff => self.ie_register,
            _ => panic!("Unsupported memory address read: {:04X}", address),
        }
    }
    pub fn handle_bank_type(&mut self, address: u16, data: u8) {
        match self.memory_bank_type {
            MBC::ROM_ONLY if address > 0x8000 => {
                panic!("Trying to write to address greater than 0x8000")
            }
            MBC::MBC2 if get_bit_at(address.to_be_bytes()[1], 4) => {}
            MBC::MBC1 | MBC::MBC2 if address <= 0x1fff => match data & 0xf {
                0b1010 => self.set_is_ram_enabled(true),
                _ => self.set_is_ram_enabled(false),
            },
            MBC::MBC1 if (address >= 0x2000) && (address <= 0x3fff) => self.set_bank1(data),
            MBC::MBC2 if (address >= 0x2000) && (address <= 0x3fff) => self.set_bank1(data),
            MBC::MBC1 if (address >= 0x4000) && (address <= 0x5fff) => self.set_bank2(data),
            MBC::MBC1 if (address >= 0x6000) && (address <= 0x7FFF) => match data & 0b1 {
                0x0 => self.set_banking_mode(Bmode::ROM),
                0x1 => self.set_banking_mode(Bmode::RAM),
                _ => panic!("Unsupported banking mode"),
            },
            _ => panic!("MBC case not supported"),
        };
    }
    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7fff => self.handle_bank_type(address, data),
            0x8000..=0x9FFF => self.write_vram(address, data),
            0xa000..=0xbfff if self.is_ram_enabled => {
                self.write_ram(address, self.get_bank2_as_low(), data)
            }
            0xa000..=0xbfff => {}
            0xc000..=0xdfff => self.write_wram(address, data),
            0xe000..=0xfdff => {}
            0xfe00..=0xfe9f => self.write_oam(address, data),
            0xfea0..=0xfeff => {}
            0xff00 | 0xff20 => self.write_io_ports(address, data | 0b1100_0000),
            0xff01 => {
                self.write_io_ports(address, data);
                self.write_io_ports(0xff02, 0x81);
                let c = self.read_io_ports(0xff01) as char;
                let mut out = std::io::stdout();
                print!("{}", c);
                let _ = out.flush();
            }
            0xff02 => self.write_io_ports(address, data | 0b0111_1110),
            0xff04 => {
                self.write_io_ports(0xff04, 0);
                self.write_io_ports(0xff05, 0); // TO CHECK
            }
            0xff07 => {
                self.set_clock_frequency(data & 0x3);
                self.write_io_ports(address, data | 0b1111_1000)
            }
            0xff0f => self.write_io_ports(address, data | 0b1110_0000),
            0xff10 | 0xff41 => self.write_io_ports(address, data | 0b1000_0000),
            0xff1a => self.write_io_ports(address, data | 0b0111_1111),
            0xff1c => self.write_io_ports(address, data | 0b1001_1111),
            0xff23 => self.write_io_ports(address, data | 0b0011_1111),
            0xff26 => self.write_io_ports(address, data | 0b0111_0000),
            0xff44 => self.write_io_ports(address, 0),
            0xff46 => {
                self.dma_transfer(data);
                self.write_io_ports(address, data)
            }
            0xff03 | 0xff08..=0xff0e | 0xff15 | 0xff1f | 0xff27..=0xff29 | 0xff4c..=0xff7f => {
                self.write_io_ports(address, data | 0b1111_1111)
            }
            0xff80..=0xfffe => self.write_hram(address, data),
            0xffff => self.ie_register = data,
            _ => self.write_io_ports(address, data),
        }
    }
}
