use super::cartridge::mbc1::MBC1;
use super::cartridge::mbc2::MBC2;
use super::cartridge::mbc3::MBC3;
use super::cartridge::rom_only::RomOnly;
use super::cartridge::Cartridge;
use super::constants::*;
use super::utils::{clear_bit_at, get_bit_at, set_bit_at};
use byteorder::{BigEndian, ByteOrder};
use std::fmt;
use std::io::Write;

pub struct Point2D {
    pub x: u8,
    pub y: u8,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum LcdMode {
    HBlank,
    VBlank,
    ReadOAM,
    ReadVRAM,
}

#[derive(PartialEq)]
pub enum PrevStatCond {
    VBlank,
    LYC(u8),
    HBLANK(u8),
    OAM,
}

pub struct Memory {
    pub cartridge: Box<dyn Cartridge>,
    wram: [u8; 0x2000],
    vram: [u8; 0x2000],
    echo: [u8; 0x1e00],
    oam: [u8; 0xa0],
    io_ports: [u8; 0x80],
    hram: [u8; 0x80],
    ie_register: u8,
    wram_bank: u8,
    pub stack_pointer: u16,
    program_counter: u16,
    dma_copy_address: u16,
    dma_copy_in_progress: bool,
    dma_cursor: u16,
    pub prev_timer_bit: u16,
    pub tima_reloading: bool,
    pub prev_stat_condition: PrevStatCond,
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

        Self {
            cartridge: Box::new(RomOnly::default()),
            wram: [0; 0x2000],
            vram: [0; 0x2000],
            echo: [0; 0x1e00],
            oam: [0; 0xa0],
            hram: [0; 0x80],
            ie_register: 0,
            wram_bank: 1,
            io_ports,
            stack_pointer: 0xfffe,
            program_counter: 0x100,
            dma_copy_address: 0,
            dma_copy_in_progress: false,
            dma_cursor: 0,
            prev_timer_bit: 0,
            tima_reloading: false,
            prev_stat_condition: PrevStatCond::OAM, // everything following oam recognized.
        }
    }
}

// General CPU functions
impl Memory {
    pub fn get_word(&mut self) -> u16 {
        let c = self.get_pc();
        self.inc_pc(2);
        BigEndian::read_u16(&[self.read(c + 1), self.read(c)])
    }

    pub fn get_byte_debug(&self) -> u8 {
        self.read(self.get_pc())
    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        let bytes = data.to_be_bytes();
        self.write(address, bytes[1]);
        self.write(address.wrapping_add(1), bytes[0]);
    }

    pub fn get_word_debug(&self) -> u16 {
        let c = self.get_pc();
        BigEndian::read_u16(&[self.read(c + 1), self.read(c)])
    }

    pub fn load_rom(&mut self, cartridge: Vec<u8>) {
        if let Some(value) = cartridge.get(0xff70) {
            self.wram_bank = *value;
        }
        match cartridge[0x147] {
            0x00 => self.cartridge = Box::new(RomOnly::new(cartridge)),
            0x01..=0x03 => self.cartridge = Box::new(MBC1::new(cartridge)),
            0x05 | 0x06 => self.cartridge = Box::new(MBC2::new(cartridge)),
            0x0f..=0x13 => self.cartridge = Box::new(MBC3::new(cartridge)),
            bank => {
                let bank_type = match bank {
                    0x08 => "08h  ROM+RAM",
                    0x09 => "09h  ROM+RAM+BATTERY",
                    0x0b => "0Bh  MMM01",
                    0x0c => "0Ch  MMM01+RAM",
                    0x0d => "0Dh  MMM01+RAM+BATTERY",
                    0x15 => "15h  MBC4",
                    0x16 => "16h  MBC4+RAM",
                    0x17 => "17h  MBC4+RAM+BATTERY",
                    0x19 => "19h  MBC5",
                    0x1a => "1Ah  MBC5+RAM",
                    0x1b => "1Bh  MBC5+RAM+BATTERY",
                    0x1c => "1Ch  MBC5+RUMBLE",
                    0x1d => "1Dh  MBC5+RUMBLE+RAM",
                    0x1e => "1Eh  MBC5+RUMBLE+RAM+BATTERY",
                    0xfc => "FCh  POCKET CAMERA",
                    0xfd => "FDh  BANDAI TAMA5",
                    0xfe => "FEh  HuC3",
                    0xff => "FFh  HuC1+RAM+BATTERY",
                    _ => "Unknown",
                };
                panic!("MBC case not supported {}", bank_type);
            }
        };
        self.cartridge.debug();
    }

    pub fn s_push(&mut self, data: u16) {
        self.dec_sp(2);
        let bytes = data.to_be_bytes();
        self.write(self.stack_pointer, bytes[1]);
        self.write(self.stack_pointer.wrapping_add(1), bytes[0]);
    }

    pub fn s_pop(&mut self) -> u16 {
        let byte1 = self.read(self.stack_pointer);
        let byte2 = self.read(self.stack_pointer.wrapping_add(1));
        self.inc_sp(2);
        (byte2 as u16) << 8 | byte1 as u16
    }

    pub fn set_pc(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn set_sp(&mut self, address: u16) {
        self.stack_pointer = address;
    }

    pub fn inc_pc(&mut self, increment: u16) {
        self.program_counter = self.program_counter.wrapping_add(increment);
    }

    pub fn dec_pc(&mut self, decrement: u16) {
        self.program_counter = self.program_counter.wrapping_sub(decrement);
    }

    pub fn inc_sp(&mut self, increment: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_add(increment);
    }

    pub fn dec_sp(&mut self, decrement: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(decrement);
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn get_sp(&self) -> u16 {
        self.stack_pointer
    }

    pub fn write_ly(&mut self, data: u8) {
        self.check_ly_eq_lyc();
        self.io_ports[0x44] = data;
    }
}

// General Timer functions
impl Memory {
    pub fn get_div(&self) -> u8 {
        self.read_unchecked(DIVIDER_COUNTER_ADDRESS)
    }
    pub fn get_div_counter(&self) -> u16 {
        (self.get_div() as u16) << 8 | self.read_unchecked(DIVIDER_COUNTER_ADDRESS - 1) as u16
    }
    pub fn set_div_counter(&mut self, data: u16) {
        self.write_unchecked(DIVIDER_COUNTER_ADDRESS - 1, data as u8);
        self.write_unchecked(DIVIDER_COUNTER_ADDRESS, (data >> 8) as u8);
    }

    pub fn get_tima(&self) -> u8 {
        self.read_unchecked(TIMER_COUNTER_ADDRESS)
    }
    pub fn set_tima(&mut self, counter: u8) {
        self.write_unchecked(TIMER_COUNTER_ADDRESS, counter);
    }
    pub fn get_tma(&self) -> u8 {
        self.read_unchecked(TIMER_MODULO_ADDRESS)
    }
    pub fn get_tac(&self) -> u8 {
        self.read_unchecked(TIMER_CONTROL_ADDRESS)
    }
    pub fn tac_enabled(&self) -> u16 {
        (self.get_tac() >> 2 & 0b1) as u16
    }
    pub fn tac_freq(&self) -> u8 {
        let bits = self.get_tac() & 0b11;
        match bits {
            0 => 9, // freq 4096 / 1024
            1 => 3, // freq 262144 / 16
            2 => 5, // freq 65536 / 64
            3 => 7, // freq 16382 / 256
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

    pub fn lcd_mode(&self) -> LcdMode {
        let lcd_status = self.read(0xff41);
        match lcd_status & 0x3 {
            0x0 => LcdMode::HBlank,
            0x1 => LcdMode::VBlank,
            0x2 => LcdMode::ReadOAM,
            0x3 => LcdMode::ReadVRAM,
            _ => unreachable!(),
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
            LcdMode::ReadOAM => {
                let temp_status = set_bit_at(lcd_status, 1);
                clear_bit_at(temp_status, 0)
            }
            LcdMode::ReadVRAM => {
                let temp_status = set_bit_at(lcd_status, 1);
                set_bit_at(temp_status, 0)
            }
        };
        self.write_unchecked(0xff41, new_status);
    }

    pub fn set_coincidence_flag(&mut self) {
        let lcd_status = self.read(0xff41);
        self.write_unchecked(0xff41, set_bit_at(lcd_status, 2));
    }

    pub fn clear_coincidence_flag(&mut self) {
        let lcd_status = self.read(0xff41);
        self.write_unchecked(0xff41, clear_bit_at(lcd_status, 2));
    }

    pub fn increment_ly(&mut self) -> u8 {
        let mut scan_line = self.read(0xff44);
        scan_line = scan_line.wrapping_add(1);
        self.write_ly(scan_line);
        scan_line
    }

    pub fn check_ly_eq_lyc(&mut self) {
        if self.get_ly() == self.get_lyc() {
            self.set_coincidence_flag();
        } else {
            self.clear_coincidence_flag();
        }
    }

    pub fn get_ly(&self) -> u8 {
        self.read(0xff44)
    }

    pub fn get_lyc(&self) -> u8 {
        self.read(0xff45)
    }

    fn start_dma_transfer(&mut self, data: u8) {
        self.dma_cursor = 0;
        self.dma_copy_address = (data as u16) << 8;
        self.dma_copy_in_progress = true;
    }

    pub fn dma_copy_byte(&mut self) {
        if !self.dma_copy_in_progress {
            return;
        }
        if self.dma_cursor > 0xA0 {
            self.dma_copy_in_progress = false;
            self.dma_cursor = 0;
            self.dma_copy_address = 0;
        } else {
            let data = self.read_unchecked(self.dma_copy_address + self.dma_cursor);
            self.write_unchecked(0xfe00 + self.dma_cursor, data);
            self.dma_cursor += 1;
        }
    }
}

// Memory Read/Write functions
impl Memory {
    fn write_io_ports(&mut self, address: u16, data: u8) {
        self.io_ports[address as usize - 0xff00] = data;
    }

    fn read_io_ports(&self, address: u16) -> u8 {
        self.io_ports[address as usize - 0xff00]
    }

    fn read_vram(&self, address: u16) -> u8 {
        let vram_address = address - 0x8000;
        self.vram[vram_address as usize]
    }

    fn read_echo(&self, address: u16) -> u8 {
        let echo_address = address - 0xe000;
        self.echo[echo_address as usize]
    }

    fn write_echo(&mut self, address: u16, data: u8) {
        let bank_address = address - 0xe000;
        self.echo[bank_address as usize] = data;
        if address >= 0xe000 && address <= 0xfdff {
            self.wram[bank_address as usize] = data;
        }
    }

    fn write_vram(&mut self, address: u16, data: u8) {
        let vram_address = address - 0x8000;
        self.vram[vram_address as usize] = data;
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
            0x8000..=0x9fff if self.dma_copy_in_progress => 0xff,
            0x8000..=0x9fff if self.lcd_mode() == LcdMode::ReadVRAM => 0xff,
            0xfe00..=0xfe9f if self.dma_copy_in_progress => 0xff,
            0xa000..=0xbfff if !self.cartridge.ram_enabled() => 0xff,
            0xfe00..=0xfe9f if self.dma_copy_in_progress => 0xff,
            0xfe00..=0xfe9f if self.lcd_mode() == LcdMode::ReadOAM => 0xff,
            0xfe00..=0xfe9f if self.lcd_mode() == LcdMode::ReadVRAM => 0xff,
            _ => self.read_unchecked(address),
        }
    }
    pub fn read_unchecked(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7fff => self.cartridge.read(address),
            0x8000..=0x9fff => self.read_vram(address),
            0xa000..=0xbfff => self.cartridge.read(address),
            0xc000..=0xdfff => self.read_wram(address),
            0xe000..=0xfdff => self.read_echo(address),
            0xfe00..=0xfe9f => self.read_oam(address),
            0xfea0..=0xfeff => 0,
            0xff00 => !self.read_io_ports(address),
            0xff01..=0xff0e => self.read_io_ports(address),
            0xff0f => self.read_io_ports(address) | 0b1110_0000,
            0xff10..=0xff40 => self.read_io_ports(address),
            0xff41 => {
                let stat = self.read_io_ports(address) | 0b1000_0000;
                if !self.is_lcd_enabled() {
                    return stat & 0b1111_1000; // Bits 0-2 return '0' when the LCD is off.
                }
                stat
            }
            0xff42..=0xff7f => self.read_io_ports(address),
            0xff80..=0xfffe => self.read_hram(address),
            0xffff => self.ie_register,
        }
    }
    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9fff if self.dma_copy_in_progress => {}
            0x8000..=0x9fff if self.lcd_mode() == LcdMode::ReadVRAM => {}
            0xfe00..=0xfe9f if self.dma_copy_in_progress => {}
            0xa000..=0xbfff if !self.cartridge.ram_enabled() => {}
            0xfe00..=0xfe9f if self.dma_copy_in_progress => {}
            0xfe00..=0xfe9f if self.lcd_mode() == LcdMode::ReadOAM => {}
            0xfe00..=0xfe9f if self.lcd_mode() == LcdMode::ReadVRAM => {}
            // Writes to DIV resets DIV and counter
            0xff04 => {
                self.write_io_ports(0xff03, 0);
                self.write_io_ports(0xff04, 0);
            }
            // Write to TIMA ignored
            0xff05 if self.tima_reloading => {}
            // Write to TMA also loads TIMA
            0xff06 if self.tima_reloading => {
                self.write_unchecked(0xff05, data);
                self.write_unchecked(0xff06, data);
            }
            0xff41 => {
                let stat_register = self.read_unchecked(address) & 0b1000_0111;
                let new_stat_register = data & 0b0111_1000;
                self.write_io_ports(address, stat_register | new_stat_register)
            }
            0xff44 => {
                self.check_ly_eq_lyc();
                self.write_io_ports(address, 0);
            }
            _ => self.write_unchecked(address, data),
        }
    }
    pub fn write_unchecked(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7fff => self.cartridge.write(address, data),
            0x8000..=0x9FFF => self.write_vram(address, data),
            0xa000..=0xbfff => self.cartridge.write(address, data),
            0xc000..=0xdfff => self.write_wram(address, data),
            0xe000..=0xfdff => self.write_echo(address, data),
            0xfe00..=0xfe9f => self.write_oam(address, data),
            0xfea0..=0xfeff => {}
            0xff01 => {
                self.write_io_ports(address, data);
                self.write_io_ports(0xff02, 0x81);
                let c = self.read_io_ports(0xff01) as char;
                let mut out = std::io::stdout();
                print!("{}", c);
                let _ = out.flush();
            }
            0xff40 => {
                let enabling_lcd = get_bit_at(data, 7);
                if enabling_lcd {
                    self.check_ly_eq_lyc();
                }
                self.write_io_ports(address, data);
            }
            0xff45 => {
                self.check_ly_eq_lyc();
                self.write_io_ports(address, data);
            }
            0xff46 => {
                self.start_dma_transfer(data);
                self.write_io_ports(address, data);
            }
            0xff80..=0xfffe => self.write_hram(address, data),
            0xffff => self.ie_register = data,
            _ => self.write_io_ports(address, data),
        }
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if DEBUG_CPU {
            let pc = self.get_pc();
            let sp = self.get_sp();
            let opcode = self.get_byte_debug();
            let n16 = self.get_word_debug();
            let ie = self.read(0xffff);
            let ifl = self.read(0xff0f);
            write!(
                f,
                "PC: {:04X}  SP: {:04X} -> {:02X}{:02X}\n\
                00:{:04X}: | {:02X}{:04X}\n\
                IE: {:02X}|{:b}, IF: {:02X}|{:b}\n\
                period: {:?}\n",
                pc,
                sp,
                self.read(sp + 1),
                self.read(sp),
                pc,
                opcode,
                n16,
                ie,
                ie,
                ifl,
                ifl,
                self.lcd_mode(),
            )
            .unwrap()
        }
        if DEBUG_MEMORY {
            self.cartridge.debug();
        }
        if DEBUG_GPU {
            write!(
                f,
                "GPU: -----------------------------\n\
                LCDC: {:02X}  STAT: {:02X}  LY: {:X} ({})\n\
                LYC {:02X}\n",
                self.read(0xff40),
                self.read(0xff41),
                self.read(0xff44),
                self.read(0xff44),
                self.read(0xff45),
            )
            .unwrap();
        }
        if DEBUG_TIMERS {
            write!(
                f,
                "TIMERS: -----------------------------\n\
                Timers frequency: {}\n\
                Timer enabled: {}\n\
                0xff04 (DIV) Divider counter: {:02X}\n\
                0xff05 (TIMA) Timer counter: {:02X}\n\
                0xff06 (TMA) Timer modulo: {:02X}\n\
                0xff07 (TAC) Timer control: {:02X}\n",
                self.get_tac(),
                self.tac_enabled(),
                self.get_div(),
                self.get_tima(),
                self.get_tma(),
                self.get_tac(),
            )
            .unwrap()
        }
        Ok(())
    }
}
