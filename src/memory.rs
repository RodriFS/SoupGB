use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub enum MBC {
    NONE,
    MB1,
    MB2,
}

#[derive(Debug)]
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
    link_port: Vec<u8>,
}

impl Memory {
    pub fn new(cartridge_memory: Vec<u8>) -> Self {
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
        internal_memory[0x0000..0x7FFF].clone_from_slice(&cartridge_memory[0x0000..0x7FFF]);

        let memory_bank_type = match cartridge_memory[0x147] {
            1 | 2 | 3 => MBC::MB1,
            5 | 6 => MBC::MB2,
            _ => MBC::NONE,
        };
        Self {
            memory_bank_type,
            current_rom_bank: 1,
            cartridge_memory,
            internal_memory,
            ram_memory: [0; 0x8000],
            current_ram_bank: 0,
            is_ram_enabled: false,
            banking_mode: Bmode::ROM,
            stack_pointer: 0xfffe,
            program_counter: 0x100,
            link_port: Vec::new(),
        }
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
        // From memory bank
        if (address >= 0x4000) && (address <= 0x7FFF) {
            let mb_address = address - 0x4000;
            return self
                .cartridge_memory
                .get((mb_address + (self.current_rom_bank as u16 * 0x4000)) as usize)
                .unwrap()
                .to_owned();
        }
        // from RAM
        else if (address >= 0xA000) && (address <= 0xBFFF) {
            let ram_address = address - 0xA000;
            return self.ram_memory
                [(ram_address + (self.current_ram_bank as u16 * 0x2000)) as usize];
        }

        self.internal_memory[address as usize]
    }

    pub fn read_range(&self, range: std::ops::Range<usize>) -> &[u8] {
        self.internal_memory.get(range).unwrap()
    }

    pub fn push_to_stack(&mut self, data: u16) {
        self.decrement_stack_counter(2);
        let bytes = data.to_be_bytes();
        self.write(self.stack_pointer, bytes[0]);
        self.write(self.stack_pointer.wrapping_add(1), bytes[1]);
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let byte1 = self.read(self.stack_pointer);
        let byte2 = self.read(self.stack_pointer.wrapping_add(1));
        let data = BigEndian::read_u16(&[byte1, byte2]);
        self.increment_stack_counter(2);
        data
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

    pub fn increment_stack_counter(&mut self, increment: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_add(increment);
    }

    pub fn decrement_stack_counter(&mut self, decrement: u16) {
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

    pub fn write(&mut self, address: u16, data: u8) {
        if address < 0x8000 {
            match self.memory_bank_type {
                MBC::NONE if address > 0x8000 => {
                    panic!("Trying to write to address greater than 0x8000")
                }
                MBC::MB1 | MBC::MB2 if address <= 0x7fff => {
                    //panic!("Can't write, the cartridge data is there")
                }
                MBC::MB1 | MBC::MB2 if address <= 0x1fff => match address & 0xf {
                    0x00 => self.set_is_ram_enabled(true),
                    0x0a => self.set_is_ram_enabled(false),
                    _ => {}
                },
                MBC::MB1 | MBC::MB2 if (address >= 0x2000) && (address <= 0x3fff) => {
                    let lower_bits = (address & 0xf) as u8;
                    let upper_bits = self.current_rom_bank & 0xe0;
                    let mut next_rom_bank = upper_bits | lower_bits;
                    if next_rom_bank == 0 {
                        next_rom_bank += 1;
                    }
                    self.set_rom_bank(next_rom_bank);
                }
                MBC::MB1 if (address >= 0x4000) && (address <= 0x5fff) => match self.banking_mode {
                    Bmode::ROM => {
                        let upper_bits = (address & 0x1f) as u8;
                        let lower_bits = self.current_rom_bank & 0xe1;
                        let mut next_rom_bank = upper_bits | lower_bits;
                        if next_rom_bank == 0 {
                            next_rom_bank += 1;
                        }
                        self.set_rom_bank(next_rom_bank);
                    }
                    Bmode::RAM => {
                        self.set_ram_bank((address & 0x3) as u8);
                    }
                },
                MBC::MB1 if (address >= 0x6000) && (address <= 0x7FFF) => match address & 0x3 {
                    0x00 => self.set_banking_mode(Bmode::ROM),
                    0x01 => self.set_banking_mode(Bmode::RAM),
                    _ => panic!("Unsupported banking mode"),
                },
                _ => panic!("MBC case not supported"),
            }
        } else if self.is_ram_enabled && (address >= 0xa000) && (address <= 0xbfff) {
            let bank_address = address - 0xa000;
            self.set_ram(bank_address + self.current_ram_bank as u16 * 0x2000, data);
        } else if (address >= 0xe000) && (address <= 0xfdff) {
            self.set_rom(address, data);
            self.write(address - 0x2000, data);
        } else if (address >= 0xfea0) && (address <= 0xfefe) {
        } else if address == 0xff01 {
            println!("{:x}", data);
            self.link_port.push(data);
            self.set_rom(address, data);
            self.set_rom(0xff02, 0x81);
        } else if address == 0xff04 {
            self.set_rom(0xff04, 0);
        } else if address == 0xff44 {
            self.set_rom(address, 0);
        } else {
            self.set_rom(address, data);
        }
    }
}
