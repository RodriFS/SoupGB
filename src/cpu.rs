use super::constants::*;
use super::interrupts::Interrupts;
use super::memory::Memory;
use super::utils::{get_bit_at, swap_nibbles, test_flag_add, test_flag_add_16, test_flag_sub};
use byteorder::{BigEndian, ByteOrder};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    N8,
    N16,
    SP,
}

pub enum Flags {
    Z,
    N,
    H,
    C,
}

pub struct Cpu {
    is_halted: bool,
    memory: Rc<RefCell<Memory>>,
    interrupts: Rc<RefCell<Interrupts>>,
    frame_cycles: u32,
    total_cycles: u32,
    instructions: [&'static str; 0x100],
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl Cpu {
    pub fn new(memory: Rc<RefCell<Memory>>, interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Self {
            is_halted: false,
            memory,
            interrupts,
            frame_cycles: 0,
            total_cycles: 0,
            a: 0x11,
            f: 0xB0,
            b: 0x00,
            c: 0x0d,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            instructions: [
                "NOP",                   // 0x00
                "LD BC, 0x{}",           // 0x01
                "LD (BC), A",            // 0x02
                "INC BC",                // 0x03
                "INC B",                 // 0x04
                "DEC B",                 // 0x05
                "LD B, 0x{}",            // 0x06
                "RLCA",                  // 0x07
                "LD (0x{}), SP",         // 0x08
                "ADD HL, BC",            // 0x09
                "LD A, (BC)",            // 0x0a
                "DEC BC",                // 0x0b
                "INC C",                 // 0x0c
                "DEC C",                 // 0x0d
                "LD C, 0x{}",            // 0x0e
                "RRCA",                  // 0x0f
                "STOP",                  // 0x10
                "LD DE, 0x{}",           // 0x11
                "LD (DE), A",            // 0x12
                "INC DE",                // 0x13
                "INC D",                 // 0x14
                "DEC D",                 // 0x15
                "LD D, 0x{}",            // 0x16
                "RLA",                   // 0x17
                "JR 0x{}",               // 0x18
                "ADD HL, DE",            // 0x19
                "LD A, (DE)",            // 0x1a
                "DEC DE",                // 0x1b
                "INC E",                 // 0x1c
                "DEC E",                 // 0x1d
                "LD E, 0x{}",            // 0x1e
                "RRA",                   // 0x1f
                "JR NZ, 0x{}",           // 0x20
                "LD HL, 0x{}",           // 0x21
                "LDI (HL), A",           // 0x22
                "INC HL",                // 0x23
                "INC H",                 // 0x24
                "DEC H",                 // 0x25
                "LD H, 0x{}",            // 0x26
                "DAA",                   // 0x27
                "JR Z, 0x{}",            // 0x28
                "ADD HL, HL",            // 0x29
                "LDI A, (HL)",           // 0x2a
                "DEC HL",                // 0x2b
                "INC L",                 // 0x2c
                "DEC L",                 // 0x2d
                "LD L, 0x{}",            // 0x2e
                "CPL",                   // 0x2f
                "JR NC, 0x{}",           // 0x30
                "LD SP, 0x{}",           // 0x31
                "LDD (HL), A",           // 0x32
                "INC SP",                // 0x33
                "INC (HL)",              // 0x34
                "DEC (HL)",              // 0x35
                "LD (HL), 0x{}",         // 0x36
                "SCF",                   // 0x37
                "JR C, 0x{}",            // 0x38
                "ADD HL, SP",            // 0x39
                "LDD A, (HL)",           // 0x3a
                "DEC SP",                // 0x3b
                "INC A",                 // 0x3c
                "DEC A",                 // 0x3d
                "LD A, 0x{}",            // 0x3e
                "CCF",                   // 0x3f
                "LD B, B",               // 0x40
                "LD B, C",               // 0x41
                "LD B, D",               // 0x42
                "LD B, E",               // 0x43
                "LD B, H",               // 0x44
                "LD B, L",               // 0x45
                "LD B, (HL)",            // 0x46
                "LD B, A",               // 0x47
                "LD C, B",               // 0x48
                "LD C, C",               // 0x49
                "LD C, D",               // 0x4a
                "LD C, E",               // 0x4b
                "LD C, H",               // 0x4c
                "LD C, L",               // 0x4d
                "LD C, (HL)",            // 0x4e
                "LD C, A",               // 0x4f
                "LD D, B",               // 0x50
                "LD D, C",               // 0x51
                "LD D, D",               // 0x52
                "LD D, E",               // 0x53
                "LD D, H",               // 0x54
                "LD D, L",               // 0x55
                "LD D, (HL)",            // 0x56
                "LD D, A",               // 0x57
                "LD E, B",               // 0x58
                "LD E, C",               // 0x59
                "LD E, D",               // 0x5a
                "LD E, E",               // 0x5b
                "LD E, H",               // 0x5c
                "LD E, L",               // 0x5d
                "LD E, (HL)",            // 0x5e
                "LD E, A",               // 0x5f
                "LD H, B",               // 0x60
                "LD H, C",               // 0x61
                "LD H, D",               // 0x62
                "LD H, E",               // 0x63
                "LD H, H",               // 0x64
                "LD H, L",               // 0x65
                "LD H, (HL)",            // 0x66
                "LD H, A",               // 0x67
                "LD L, B",               // 0x68
                "LD L, C",               // 0x69
                "LD L, D",               // 0x6a
                "LD L, E",               // 0x6b
                "LD L, H",               // 0x6c
                "LD L, L",               // 0x6d
                "LD L, (HL)",            // 0x6e
                "LD L, A",               // 0x6f
                "LD (HL), B",            // 0x70
                "LD (HL), C",            // 0x71
                "LD (HL), D",            // 0x72
                "LD (HL), E",            // 0x73
                "LD (HL), H",            // 0x74
                "LD (HL), L",            // 0x75
                "HALT",                  // 0x76
                "LD (HL), A",            // 0x77
                "LD A, B",               // 0x78
                "LD A, C",               // 0x79
                "LD A, D",               // 0x7a
                "LD A, E",               // 0x7b
                "LD A, H",               // 0x7c
                "LD A, L",               // 0x7d
                "LD A, (HL)",            // 0x7e
                "LD A, A",               // 0x7f
                "ADD A, B",              // 0x80
                "ADD A, C",              // 0x81
                "ADD A, D",              // 0x82
                "ADD A, E",              // 0x83
                "ADD A, H",              // 0x84
                "ADD A, L",              // 0x85
                "ADD A, (HL)",           // 0x86
                "ADD A",                 // 0x87
                "ADC B",                 // 0x88
                "ADC C",                 // 0x89
                "ADC D",                 // 0x8a
                "ADC E",                 // 0x8b
                "ADC H",                 // 0x8c
                "ADC L",                 // 0x8d
                "ADC (HL)",              // 0x8e
                "ADC A",                 // 0x8f
                "SUB B",                 // 0x90
                "SUB C",                 // 0x91
                "SUB D",                 // 0x92
                "SUB E",                 // 0x93
                "SUB H",                 // 0x94
                "SUB L",                 // 0x95
                "SUB (HL)",              // 0x96
                "SUB A",                 // 0x97
                "SBC B",                 // 0x98
                "SBC C",                 // 0x99
                "SBC D",                 // 0x9a
                "SBC E",                 // 0x9b
                "SBC H",                 // 0x9c
                "SBC L",                 // 0x9d
                "SBC (HL)",              // 0x9e
                "SBC A",                 // 0x9f
                "AND B",                 // 0xa0
                "AND C",                 // 0xa1
                "AND D",                 // 0xa2
                "AND E",                 // 0xa3
                "AND H",                 // 0xa4
                "AND L",                 // 0xa5
                "AND (HL)",              // 0xa6
                "AND A",                 // 0xa7
                "XOR B",                 // 0xa8
                "XOR C",                 // 0xa9
                "XOR D",                 // 0xaa
                "XOR E",                 // 0xab
                "XOR H",                 // 0xac
                "XOR L",                 // 0xad
                "XOR (HL)",              // 0xae
                "XOR A",                 // 0xaf
                "OR B",                  // 0xb0
                "OR C",                  // 0xb1
                "OR D",                  // 0xb2
                "OR E",                  // 0xb3
                "OR H",                  // 0xb4
                "OR L",                  // 0xb5
                "OR (HL)",               // 0xb6
                "OR A",                  // 0xb7
                "CP B",                  // 0xb8
                "CP C",                  // 0xb9
                "CP D",                  // 0xba
                "CP E",                  // 0xbb
                "CP H",                  // 0xbc
                "CP L",                  // 0xbd
                "CP (HL)",               // 0xbe
                "CP A",                  // 0xbf
                "RET NZ",                // 0xc0
                "POP BC",                // 0xc1
                "JP NZ, 0x{}",           // 0xc2
                "JP 0x{}",               // 0xc3
                "CALL NZ, 0x{}",         // 0xc4
                "PUSH BC",               // 0xc5
                "ADD A, 0x{}",           // 0xc6
                "RST 0x00",              // 0xc7
                "RET Z",                 // 0xc8
                "RET",                   // 0xc9
                "JP Z, 0x{}",            // 0xca
                "CB {}",                 // 0xcb
                "CALL Z, 0x{}",          // 0xcc
                "CALL 0x{}",             // 0xcd
                "ADC 0x{}",              // 0xce
                "RST 0x08",              // 0xcf
                "RET NC",                // 0xd0
                "POP DE",                // 0xd1
                "JP NC, 0x{}",           // 0xd2
                "UNKNOWN",               // 0xd3
                "CALL NC, 0x{}",         // 0xd4
                "PUSH DE",               // 0xd5
                "SUB 0x{}",              // 0xd6
                "RST 0x10",              // 0xd7
                "RET C",                 // 0xd8
                "RETI",                  // 0xd9
                "JP C, 0x{}",            // 0xda
                "UNKNOWN",               // 0xdb
                "CALL C, 0x{}",          // 0xdc
                "UNKNOWN",               // 0xdd
                "SBC 0x{}",              // 0xde
                "RST 0x18",              // 0xdf
                "LD (0xFF00 + 0x{}), A", // 0xe0
                "POP HL",                // 0xe1
                "LD (0xFF00 + C), A",    // 0xe2
                "UNKNOWN",               // 0xe3
                "UNKNOWN",               // 0xe4
                "PUSH HL",               // 0xe5
                "AND 0x{}",              // 0xe6
                "RST 0x20",              // 0xe7
                "ADD SP,0x{}",           // 0xe8
                "JP HL",                 // 0xe9
                "LD (0x{}), A",          // 0xea
                "UNKNOWN",               // 0xeb
                "UNKNOWN",               // 0xec
                "UNKNOWN",               // 0xed
                "XOR 0x{}",              // 0xee
                "RST 0x28",              // 0xef
                "LD A, (0xFF00 + 0x{})", // 0xf0
                "POP AF",                // 0xf1
                "LD A, (0xFF00 + C)",    // 0xf2
                "DI",                    // 0xf3
                "UNKNOWN",               // 0xf4
                "PUSH AF",               // 0xf5
                "OR 0x{}",               // 0xf6
                "RST 0x30",              // 0xf7
                "LD HL, SP+0x{}",        // 0xf8
                "LD SP, HL",             // 0xf9
                "LD A, (0x{})",          // 0xfa
                "EI",                    // 0xfb
                "UNKNOWN",               // 0xfc
                "UNKNOWN",               // 0xfd
                "CP 0x{}",               // 0xfe
                "RST 0x38",              // 0xff
            ],
        }
    }

    fn get_flag(&self, flag: Flags) -> u8 {
        match flag {
            Flags::Z => get_bit_at(self.f, 7) as u8,
            Flags::N => get_bit_at(self.f, 6) as u8,
            Flags::H => get_bit_at(self.f, 5) as u8,
            Flags::C => get_bit_at(self.f, 4) as u8,
        }
    }
    fn set_flag(&mut self, flag: Flags, value: bool) {
        let mask = match flag {
            Flags::Z => 0x80,
            Flags::N => 0x40,
            Flags::H => 0x20,
            Flags::C => 0x10,
        };
        if value {
            self.f |= mask;
        } else {
            self.f &= !(mask);
        };
    }

    fn get_next_16(&self) -> u16 {
        let c = self.memory.borrow().get_program_counter() as usize;
        self.memory.borrow_mut().increment_program_counter(2);
        let mem = self.memory.borrow();
        let address = mem.read_range(c..(c + 2));
        BigEndian::read_u16(&[address[0], address[1]])
    }
    fn get_next_8(&self) -> u8 {
        let data = self.read_memory_at_current_location();
        self.memory.borrow_mut().increment_program_counter(1);
        data
    }
    fn get_next_16_debug(&self) -> u16 {
        let c = self.memory.borrow().get_program_counter() as usize;
        let mem = self.memory.borrow();
        let address = mem.read_range((c + 1)..(c + 3));
        BigEndian::read_u16(&[address[0], address[1]])
    }

    fn get_instruction_at(&self, address: u8) -> &str {
        self.instructions[address as usize]
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.memory.borrow_mut().write(address, data);
    }
    fn mem_write_sp(&mut self, address: u16) {
        self.memory.borrow_mut().set_stack_pointer(address)
    }
    fn mem_write_pc(&mut self, address: u16) {
        self.memory.borrow_mut().set_program_counter(address);
    }
    fn mem_push_stack(&mut self, address: u16) {
        self.memory.borrow_mut().push_to_stack(address);
    }
    fn mem_pop_stack(&self) -> u16 {
        self.memory.borrow_mut().pop_from_stack()
    }
    fn mem_read_sp(&self) -> u16 {
        self.memory.borrow().get_stack_pointer()
    }
    fn mem_read_pc(&self) -> u16 {
        self.memory.borrow().get_program_counter()
    }
    fn mem_read(&self, address: u16) -> u8 {
        self.memory.borrow().read(address)
    }
    fn get_reg_u16(&self, reg: Reg) -> u16 {
        let address = match reg {
            Reg::AF => [self.a, self.f],
            Reg::BC => [self.b, self.c],
            Reg::DE => [self.d, self.e],
            Reg::HL => [self.h, self.l],
            _ => panic!("Unsupported fn get_reg_u16"),
        };
        BigEndian::read_u16(&address)
    }

    fn ld_nn_n(&mut self, reg: Reg) -> u32 {
        let next_8 = self.get_next_8();
        match reg {
            Reg::B => self.b = next_8,
            Reg::C => self.c = next_8,
            Reg::D => self.d = next_8,
            Reg::E => self.e = next_8,
            Reg::H => self.h = next_8,
            Reg::L => self.l = next_8,
            _ => panic!("Unsupported fn ld_nn_n"),
        }
        8
    }
    fn ld_n_nn(&mut self, n: Reg) -> u32 {
        let data = self.get_next_16().swap_bytes();
        match n {
            Reg::BC => {
                let split = data.to_be_bytes();
                self.b = split[0];
                self.c = split[1];
            }
            Reg::DE => {
                let split = data.to_be_bytes();
                self.d = split[0];
                self.e = split[1];
            }
            Reg::HL => {
                let split = data.to_be_bytes();
                self.h = split[0];
                self.l = split[1];
            }
            Reg::SP => {
                self.mem_write_sp(data);
            }
            _ => panic!("Unsupported fn ld_n_nn"),
        }
        12
    }
    fn ld_r1_r2(&mut self, r1: Reg, r2: u8) -> u32 {
        match r1 {
            Reg::A => {
                self.a = r2;
                4
            }
            Reg::B => {
                self.b = r2;
                4
            }
            Reg::C => {
                self.c = r2;
                4
            }
            Reg::D => {
                self.d = r2;
                4
            }
            Reg::E => {
                self.e = r2;
                4
            }
            Reg::H => {
                self.h = r2;
                4
            }
            Reg::L => {
                self.l = r2;
                4
            }
            _ => panic!("Unsupported fn ld_r1_r2"),
        }
    }
    fn ld_r1_hl(&mut self, r1: Reg) -> u32 {
        let data = self.memory.borrow().read(self.get_reg_u16(Reg::HL));
        match r1 {
            Reg::A => self.a = data,
            Reg::B => self.b = data,
            Reg::C => self.c = data,
            Reg::D => self.d = data,
            Reg::E => self.e = data,
            Reg::H => self.h = data,
            Reg::L => self.l = data,
            _ => panic!("Unsupported fn ld_r1_hl"),
        }
        8
    }
    fn ld_hl_r2(&mut self, r2: Reg) -> u32 {
        let data = match r2 {
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            _ => panic!("Unsupported fn ld_hl_r2"),
        };
        self.memory
            .borrow_mut()
            .write(self.get_reg_u16(Reg::HL), data);
        8
    }
    fn ld_a_n(&mut self, reg: Reg) -> u32 {
        let address = match reg {
            Reg::BC => self.get_reg_u16(Reg::BC),
            Reg::DE => self.get_reg_u16(Reg::DE),
            Reg::HL => self.get_reg_u16(Reg::HL),
            Reg::N16 => self.get_next_16(),
            _ => panic!("Unsupported fn ld_a_n"),
        };
        let data = self.mem_read(address);
        self.memory.borrow_mut().increment_program_counter(1);
        self.a = data;
        if reg == Reg::N16 {
            return 16;
        }
        8
    }
    fn ld_n_a(&mut self, reg: Reg) -> u32 {
        let address = match reg {
            Reg::BC => self.get_reg_u16(Reg::BC),
            Reg::DE => self.get_reg_u16(Reg::DE),
            Reg::HL => self.get_reg_u16(Reg::HL),
            Reg::N16 => self.get_next_16(),
            _ => panic!("Unsupported fn ld_n_a"),
        };
        self.mem_write(address, self.a); // To swap or not to swap, that's the question
        if reg == Reg::N16 {
            return 16;
        }
        8
    }
    fn push_nn(&mut self, reg: Reg) -> u32 {
        let address = match reg {
            Reg::AF => self.get_reg_u16(Reg::AF),
            Reg::BC => self.get_reg_u16(Reg::BC),
            Reg::DE => self.get_reg_u16(Reg::DE),
            Reg::HL => self.get_reg_u16(Reg::HL),
            _ => panic!("Unsupported fn push_nn"),
        };
        println!("Pushing address to stack (push_nn): {:04X}", address);
        self.mem_push_stack(address);
        16
    }
    fn pop_nn(&mut self, reg: Reg) -> u32 {
        let stack = self.mem_pop_stack();
        println!("Popping address from stack (pop_nn): {:04X}", stack);
        let bytes = stack.to_be_bytes();
        match reg {
            Reg::AF => {
                self.a = bytes[0];
                self.f = bytes[1];
            }
            Reg::BC => {
                self.b = bytes[0];
                self.c = bytes[1];
            }
            Reg::DE => {
                self.d = bytes[0];
                self.e = bytes[1];
            }
            Reg::HL => {
                self.h = bytes[0];
                self.l = bytes[1];
            }
            _ => panic!("Unsupported fn pop_nn"),
        }
        12
    }
    fn add_a_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn add_a_n"),
        };
        self.set_flag(Flags::Z, test_flag_add(self.a, data, Flags::Z));
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, test_flag_add(self.a, data, Flags::H));
        self.set_flag(Flags::C, test_flag_add(self.a, data, Flags::C));
        self.a = self.a.wrapping_add(data);
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn addc_a_n(&mut self, reg: Reg) -> u32 {
        let carry = self.get_flag(Flags::C);
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn add_a_n"),
        };
        self.set_flag(Flags::Z, test_flag_add(self.a, data + carry, Flags::Z));
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, test_flag_add(self.a, data + carry, Flags::H));
        self.set_flag(Flags::C, test_flag_add(self.a, data + carry, Flags::C));
        self.a = self.a.wrapping_add(data + carry);
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn sub_a_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn add_a_n"),
        };
        self.set_flag(Flags::Z, test_flag_sub(self.a, data, Flags::Z));
        self.set_flag(Flags::N, true);
        self.set_flag(Flags::H, test_flag_sub(self.a, data, Flags::H));
        self.set_flag(Flags::C, test_flag_sub(self.a, data, Flags::C));
        self.a = self.a.wrapping_sub(data);
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn subc_a_n(&mut self, reg: Reg) -> u32 {
        let carry = self.get_flag(Flags::C);
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn add_a_n"),
        };
        self.set_flag(Flags::Z, test_flag_sub(self.a, data + carry, Flags::Z));
        self.set_flag(Flags::N, true);
        self.set_flag(Flags::H, test_flag_sub(self.a, data + carry, Flags::H));
        self.set_flag(Flags::C, test_flag_sub(self.a, data + carry, Flags::C));
        self.a = self.a.wrapping_sub(data + carry);
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn and_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn and_n"),
        };
        let result = data & self.a;
        self.set_flag(Flags::Z, result == 0);
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, true);
        self.set_flag(Flags::C, false);
        self.a = result;
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn or_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn or_n"),
        };
        let result = data | self.a;
        self.set_flag(Flags::Z, result == 0);
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, false);
        self.set_flag(Flags::C, false);
        self.a = result;
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn xor_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn xor_n"),
        };
        let result = data ^ self.a;
        self.set_flag(Flags::Z, result == 0);
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, false);
        self.set_flag(Flags::C, false);
        self.a = result;
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn cp_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            Reg::N8 => self.get_next_8(),
            _ => panic!("Unsupported fn cp_n"),
        };
        self.set_flag(Flags::Z, test_flag_sub(self.a, data, Flags::Z));
        self.set_flag(Flags::N, true);
        self.set_flag(Flags::H, test_flag_sub(self.a, data, Flags::H));
        self.set_flag(Flags::C, test_flag_sub(self.a, data, Flags::C));
        self.memory.borrow_mut().increment_program_counter(1);
        if reg == Reg::HL || reg == Reg::N8 {
            return 8;
        }
        4
    }
    fn inc_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => {
                let old_value = self.a;
                self.a = self.a.wrapping_add(1);
                old_value
            }
            Reg::B => {
                let old_value = self.b;
                self.b = self.b.wrapping_add(1);
                old_value
            }
            Reg::C => {
                let old_value = self.c;
                self.c = self.c.wrapping_add(1);
                old_value
            }
            Reg::D => {
                let old_value = self.d;
                self.d = self.d.wrapping_add(1);
                old_value
            }
            Reg::E => {
                let old_value = self.e;
                self.e = self.e.wrapping_add(1);
                old_value
            }
            Reg::H => {
                let old_value = self.h;
                self.h = self.h.wrapping_add(1);
                old_value
            }
            Reg::L => {
                let old_value = self.l;
                self.l = self.l.wrapping_add(1);
                old_value
            }
            Reg::HL => {
                let address = self.get_reg_u16(Reg::HL);
                let old_value = self.mem_read(address);
                self.mem_write(address, old_value.wrapping_add(1));
                old_value
            }
            _ => panic!("Unsupported fn inc_n"),
        };
        self.set_flag(Flags::Z, test_flag_add(data, 1, Flags::Z));
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, test_flag_add(data, 1, Flags::H));
        if reg == Reg::HL {
            return 12;
        }
        4
    }
    fn dec_n(&mut self, reg: Reg) -> u32 {
        let data = match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            Reg::HL => self.mem_read(self.get_reg_u16(Reg::HL)),
            _ => panic!("Unsupported fn dec_n"),
        };
        self.set_flag(Flags::Z, test_flag_sub(data, 1, Flags::Z));
        self.set_flag(Flags::N, true);
        self.set_flag(Flags::H, test_flag_sub(data, 1, Flags::H));
        if reg == Reg::HL {
            return 12;
        }
        4
    }
    fn add_hl_n(&mut self, reg: Reg) -> u32 {
        let hl = self.get_reg_u16(Reg::HL);
        let data = match reg {
            Reg::BC => self.get_reg_u16(Reg::BC),
            Reg::DE => self.get_reg_u16(Reg::DE),
            Reg::HL => hl,
            Reg::SP => self.mem_read_sp(),
            _ => panic!("Unsupported fn add_hl_n"),
        };
        let result = hl.wrapping_add(data);
        self.set_flag(Flags::Z, test_flag_add_16(hl, data, Flags::Z));
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, test_flag_add_16(hl, data, Flags::H));
        self.set_flag(Flags::C, test_flag_add_16(hl, data, Flags::C));
        let bytes = result.to_be_bytes();
        self.h = bytes[0];
        self.l = bytes[1];
        8
    }
    fn swap_n(&mut self, reg: Reg) -> u32 {
        let result = match reg {
            Reg::A => {
                self.a = swap_nibbles(self.a);
                self.a
            }
            Reg::B => {
                self.b = swap_nibbles(self.b);
                self.b
            }
            Reg::C => {
                self.c = swap_nibbles(self.c);
                self.c
            }
            Reg::D => {
                self.d = swap_nibbles(self.d);
                self.d
            }
            Reg::E => {
                self.e = swap_nibbles(self.e);
                self.e
            }
            Reg::H => {
                self.h = swap_nibbles(self.h);
                self.h
            }
            Reg::L => {
                self.l = swap_nibbles(self.l);
                self.l
            }
            Reg::HL => {
                let address = self.get_reg_u16(Reg::HL);
                let data = swap_nibbles(self.mem_read(address));
                self.mem_write(address, data);
                data
            }
            _ => panic!("Unsupported swap_n"),
        };
        if result == 0 {
            self.set_flag(Flags::Z, true);
        }
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::C, false);
        self.set_flag(Flags::H, false);

        if reg == Reg::HL {
            return 16;
        }
        8
    }
    fn jr_cc_n(&mut self, condition: bool) -> u32 {
        if condition {
            let address = self.get_next_8() as i8;
            self.mem_write_pc(self.mem_read_pc().wrapping_add(address as u16));
        }
        8
    }
    fn ret_cc(&mut self, condition: bool) -> u32 {
        if condition {
            let address = self.mem_pop_stack();
            println!("Popping address from stack (ret_cc): {:04X}", address);
            self.mem_write_pc(address);
        }
        8
    }
    fn jp_cc_nn(&mut self, condition: bool) -> u32 {
        if condition {
            let address = self.get_next_16().swap_bytes();
            self.mem_write_pc(address);
        }
        12
    }
    fn call_cc_nn(&mut self, condition: bool) -> u32 {
        if condition {
            let address = self.get_next_16().swap_bytes();
            let next_pc = self.mem_read_pc();
            println!("Pushing address to stack (call_cc_nn): {:04X}", next_pc);
            self.mem_push_stack(next_pc);
            self.mem_write_pc(address);
        }
        12
    }
    fn rst_n(&mut self, new_address: u16) -> u32 {
        let current_address = self.mem_read_pc();
        println!("Pushing address to stack (rst_n): {:04X}", current_address);
        self.mem_push_stack(current_address);
        self.mem_write_pc(new_address);
        32
    }
    fn di(&mut self) -> u32 {
        self.interrupts.borrow_mut().clear_master_enabled();
        4
    }
    fn ei(&mut self) -> u32 {
        self.interrupts.borrow_mut().set_master_enabled_on();
        4
    }
    fn cb(&mut self) -> u32 {
        let address = self.get_next_8();
        println!("Executing callback opcode {:x}", address);
        self.execute_opcode(address, true);
        4
    }
    fn cpu_res_b_a(&mut self, bit: u8) -> u32 {
        self.a &= !(bit);
        8
    }
    fn execute_opcode(&mut self, opcode: u8, is_callback: bool) -> u32 {
        if is_callback {
            self.memory.borrow_mut().increment_program_counter(1);
            return match opcode {
                0x30 if is_callback => self.swap_n(Reg::B),
                0x31 if is_callback => self.swap_n(Reg::C),
                0x32 if is_callback => self.swap_n(Reg::D),
                0x33 if is_callback => self.swap_n(Reg::E),
                0x34 if is_callback => self.swap_n(Reg::H),
                0x35 if is_callback => self.swap_n(Reg::L),
                0x36 if is_callback => self.swap_n(Reg::HL),
                0x37 if is_callback => self.swap_n(Reg::A),
                0x87 if is_callback => self.cpu_res_b_a(0x1),
                // END OF CALLBACKS //
                _ => {
                    println!(" Callback Not implemented: {:x}", opcode);
                    panic!("Not implemented");
                }
            };
        }
        match opcode {
            0x00 => 4,
            0x01 => self.ld_n_nn(Reg::BC),
            0x02 => self.ld_n_a(Reg::BC),
            0x03 => {
                let address = self.get_reg_u16(Reg::BC).wrapping_add(1).to_be_bytes();
                self.b = address[0];
                self.c = address[1];
                8
            }
            0x04 => self.inc_n(Reg::B),
            0x05 => self.dec_n(Reg::B),
            0x06 => self.ld_nn_n(Reg::B),
            0x07 => {
                let result = self.a << 1;
                let to_carry = self.a >> 7;
                self.set_flag(Flags::Z, result == 0);
                self.set_flag(Flags::N, false);
                self.set_flag(Flags::H, false);
                self.set_flag(Flags::C, to_carry == 1);
                4
            }
            0x08 => {
                let address = self.get_next_16();
                let stack_pointer = self.memory.borrow().get_stack_pointer().to_be_bytes();
                self.memory.borrow_mut().write(address, stack_pointer[0]);
                self.memory
                    .borrow_mut()
                    .write(address.wrapping_add(1), stack_pointer[1]);
                20
            }
            0x09 => self.add_hl_n(Reg::BC),
            0x0a => self.ld_a_n(Reg::BC),
            0x0b => {
                let address = self.get_reg_u16(Reg::BC).wrapping_sub(1).to_be_bytes();
                self.b = address[0];
                self.c = address[1];
                8
            }
            0x0c => self.inc_n(Reg::C),
            0x0d => self.dec_n(Reg::C),
            0x0e => self.ld_nn_n(Reg::C),
            0x0f => {
                let result = self.a >> 1;
                let to_carry = self.a & 0x1;
                self.set_flag(Flags::Z, result == 0);
                self.set_flag(Flags::N, false);
                self.set_flag(Flags::H, false);
                self.set_flag(Flags::C, to_carry == 1);
                4
            }
            0x10 => 4,
            0x11 => self.ld_n_nn(Reg::DE),
            0x12 => self.ld_n_a(Reg::DE),
            0x13 => {
                let address = self.get_reg_u16(Reg::DE).wrapping_add(1).to_be_bytes();
                self.d = address[0];
                self.e = address[1];
                8
            }
            0x14 => self.inc_n(Reg::D),
            0x15 => self.dec_n(Reg::D),
            0x16 => self.ld_nn_n(Reg::D),
            0x17 => {
                let result = (self.a << 1).wrapping_add(self.get_flag(Flags::C));
                let to_carry = self.a >> 7;
                self.set_flag(Flags::Z, result == 0);
                self.set_flag(Flags::N, false);
                self.set_flag(Flags::H, false);
                self.set_flag(Flags::C, to_carry == 1);
                4
            }
            0x18 => self.jr_cc_n(true),
            0x19 => self.add_hl_n(Reg::DE),
            0x1a => self.ld_a_n(Reg::DE),
            0x1b => {
                let address = self.get_reg_u16(Reg::DE).wrapping_sub(1).to_be_bytes();
                self.d = address[0];
                self.e = address[1];
                8
            }
            0x1c => self.inc_n(Reg::E),
            0x1d => self.dec_n(Reg::E),
            0x1e => self.ld_nn_n(Reg::E),
            0x1f => {
                let result = (self.get_flag(Flags::C) << 7).wrapping_add(self.a >> 1);
                let to_carry = self.a >> 7;
                self.set_flag(Flags::Z, result == 0);
                self.set_flag(Flags::N, false);
                self.set_flag(Flags::H, false);
                self.set_flag(Flags::C, to_carry == 1);
                4
            }
            0x20 => self.jr_cc_n(self.get_flag(Flags::Z) == 0),
            0x21 => self.ld_n_nn(Reg::HL),
            0x22 => {
                let mut address = self.get_reg_u16(Reg::HL);
                self.memory.borrow_mut().write(address, self.a);
                address = address.wrapping_add(1);
                let decrement_address = address.to_be_bytes();
                self.h = decrement_address[0];
                self.l = decrement_address[1];
                8
            }
            0x23 => {
                let address = self.get_reg_u16(Reg::HL).wrapping_add(1).to_be_bytes();
                self.h = address[0];
                self.l = address[1];
                8
            }
            0x24 => self.inc_n(Reg::H),
            0x25 => self.dec_n(Reg::H),
            0x26 => self.ld_nn_n(Reg::H),
            0x27 => {
                let mut correction = 0;

                let mut set_flag_c = 0;
                let flag_h = self.get_flag(Flags::H);
                let flag_n = self.get_flag(Flags::N);
                let flag_c = self.get_flag(Flags::C);
                let flag_z = self.get_flag(Flags::Z);
                if flag_h == 1 || (flag_n == 0 && (self.a & 0xf) > 9) {
                    correction |= 0x6;
                }

                if flag_c == 1 || (!flag_n == 0 && self.a > 0x99) {
                    correction |= 0x60;
                    set_flag_c = flag_c;
                }

                self.a += flag_n;
                if self.a == 1 {
                    self.a -= correction;
                } else {
                    self.a += correction;
                }
                self.a &= 0xff;

                let mut set_flag_z = 0;
                if self.a == 0 {
                    set_flag_z = flag_z;
                };

                self.f &= !(flag_h | flag_z | flag_c);
                self.f |= set_flag_c | set_flag_z;

                self.set_flag(Flags::Z, set_flag_z == 1);
                self.set_flag(Flags::H, false);
                self.set_flag(Flags::C, set_flag_c == 1);
                4
            }
            0x28 => self.jr_cc_n(self.get_flag(Flags::Z) == 1),
            0x29 => self.add_hl_n(Reg::HL),
            0x2a => {
                let mut address = self.get_reg_u16(Reg::HL);
                let data = self.memory.borrow().read(address);
                self.a = data;
                address = address.wrapping_add(1);
                let incremented_address = address.to_be_bytes();
                self.h = incremented_address[0];
                self.l = incremented_address[1];
                8
            }
            0x2b => {
                let address = self.get_reg_u16(Reg::HL).wrapping_sub(1).to_be_bytes();
                self.h = address[0];
                self.l = address[1];
                8
            }
            0x2c => self.inc_n(Reg::L),
            0x2d => self.dec_n(Reg::L),
            0x2e => self.ld_nn_n(Reg::L),
            0x2f => {
                self.a = !self.a;
                self.set_flag(Flags::H, true);
                self.set_flag(Flags::N, true);
                4
            }
            0x30 => self.jr_cc_n(self.get_flag(Flags::N) == 0),
            0x31 => self.ld_n_nn(Reg::SP),
            0x32 => {
                let mut address = self.get_reg_u16(Reg::HL);
                self.memory.borrow_mut().write(address, self.a);
                address = address.wrapping_sub(1);
                let decrement_address = address.to_be_bytes();
                self.h = decrement_address[0];
                self.l = decrement_address[1];
                8
            }
            0x33 => {
                let address = self.mem_read_sp().wrapping_add(1);
                self.mem_write_sp(address);
                8
            }
            0x34 => self.inc_n(Reg::HL),
            0x35 => self.dec_n(Reg::HL),
            0x36 => {
                let data = self.get_next_8();
                self.memory
                    .borrow_mut()
                    .write(self.get_reg_u16(Reg::HL), data);
                12
            }
            0x37 => {
                self.set_flag(Flags::C, true);
                self.set_flag(Flags::N, false);
                self.set_flag(Flags::H, false);
                4
            }
            0x38 => self.jr_cc_n(self.get_flag(Flags::N) == 1),
            0x39 => self.add_hl_n(Reg::SP),
            0x3a => {
                let mut address = self.get_reg_u16(Reg::HL);
                let data = self.memory.borrow().read(address);
                self.a = data;
                address = address.wrapping_sub(1);
                let decrement_address = address.to_be_bytes();
                self.h = decrement_address[0];
                self.l = decrement_address[1];
                8
            }
            0x3b => {
                let address = self.mem_read_sp().wrapping_sub(1);
                self.mem_write_sp(address);
                8
            }
            0x3c => self.inc_n(Reg::A),
            0x3d => self.dec_n(Reg::A),
            0x3e => self.ld_r1_r2(Reg::A, self.get_next_8()),
            0x3f => {
                self.set_flag(Flags::C, self.get_flag(Flags::C) == 0);
                self.set_flag(Flags::N, false);
                self.set_flag(Flags::H, false);
                4
            }
            0x40 => 4,
            0x41 => self.ld_r1_r2(Reg::B, self.c),
            0x42 => self.ld_r1_r2(Reg::B, self.d),
            0x43 => self.ld_r1_r2(Reg::B, self.e),
            0x44 => self.ld_r1_r2(Reg::B, self.h),
            0x45 => self.ld_r1_r2(Reg::B, self.l),
            0x46 => self.ld_r1_hl(Reg::B),
            0x47 => self.ld_r1_r2(Reg::B, self.a),
            0x48 => self.ld_r1_r2(Reg::C, self.b),
            0x49 => 4,
            0x4a => self.ld_r1_r2(Reg::C, self.d),
            0x4b => self.ld_r1_r2(Reg::C, self.e),
            0x4c => self.ld_r1_r2(Reg::C, self.h),
            0x4d => self.ld_r1_r2(Reg::C, self.l),
            0x4e => self.ld_r1_hl(Reg::C),
            0x4f => self.ld_r1_r2(Reg::C, self.a),
            0x50 => self.ld_r1_r2(Reg::D, self.b),
            0x51 => self.ld_r1_r2(Reg::D, self.c),
            0x52 => 4,
            0x53 => self.ld_r1_r2(Reg::D, self.e),
            0x54 => self.ld_r1_r2(Reg::D, self.h),
            0x55 => self.ld_r1_r2(Reg::D, self.l),
            0x56 => self.ld_r1_hl(Reg::D),
            0x57 => self.ld_r1_r2(Reg::D, self.a),
            0x58 => self.ld_r1_r2(Reg::E, self.b),
            0x59 => self.ld_r1_r2(Reg::E, self.c),
            0x5a => self.ld_r1_r2(Reg::E, self.d),
            0x5b => 4,
            0x5c => self.ld_r1_r2(Reg::E, self.h),
            0x5d => self.ld_r1_r2(Reg::E, self.l),
            0x5e => self.ld_r1_hl(Reg::E),
            0x5f => self.ld_r1_r2(Reg::E, self.a),
            0x60 => self.ld_r1_r2(Reg::H, self.b),
            0x61 => self.ld_r1_r2(Reg::H, self.c),
            0x62 => self.ld_r1_r2(Reg::H, self.d),
            0x63 => self.ld_r1_r2(Reg::H, self.e),
            0x64 => 4,
            0x65 => self.ld_r1_r2(Reg::H, self.l),
            0x66 => self.ld_r1_hl(Reg::H),
            0x67 => self.ld_r1_r2(Reg::H, self.a),
            0x68 => self.ld_r1_r2(Reg::L, self.b),
            0x69 => self.ld_r1_r2(Reg::L, self.c),
            0x6a => self.ld_r1_r2(Reg::L, self.d),
            0x6b => self.ld_r1_r2(Reg::L, self.e),
            0x6c => self.ld_r1_r2(Reg::L, self.h),
            0x6d => 4,
            0x6e => self.ld_r1_hl(Reg::L),
            0x6f => self.ld_r1_r2(Reg::L, self.a),
            0x70 => self.ld_hl_r2(Reg::B),
            0x71 => self.ld_hl_r2(Reg::C),
            0x72 => self.ld_hl_r2(Reg::D),
            0x73 => self.ld_hl_r2(Reg::E),
            0x74 => self.ld_hl_r2(Reg::H),
            0x75 => self.ld_hl_r2(Reg::L),
            0x76 => {
                self.is_halted = true;
                4
            }
            0x77 => self.ld_n_a(Reg::HL),
            0x78 => self.ld_r1_r2(Reg::A, self.b),
            0x79 => self.ld_r1_r2(Reg::A, self.c),
            0x7a => self.ld_r1_r2(Reg::A, self.d),
            0x7b => self.ld_r1_r2(Reg::A, self.e),
            0x7c => self.ld_r1_r2(Reg::A, self.h),
            0x7d => self.ld_r1_r2(Reg::A, self.l),
            0x7e => self.ld_r1_hl(Reg::A),
            0x7f => 4,
            0x80 => self.add_a_n(Reg::B),
            0x81 => self.add_a_n(Reg::C),
            0x82 => self.add_a_n(Reg::D),
            0x83 => self.add_a_n(Reg::E),
            0x84 => self.add_a_n(Reg::H),
            0x85 => self.add_a_n(Reg::L),
            0x86 => self.add_a_n(Reg::HL),
            0x87 => self.add_a_n(Reg::A),
            0x88 => self.addc_a_n(Reg::B),
            0x89 => self.addc_a_n(Reg::C),
            0x8a => self.addc_a_n(Reg::D),
            0x8b => self.addc_a_n(Reg::E),
            0x8c => self.addc_a_n(Reg::H),
            0x8d => self.addc_a_n(Reg::L),
            0x8e => self.addc_a_n(Reg::HL),
            0x8f => self.addc_a_n(Reg::A),
            0x90 => self.sub_a_n(Reg::B),
            0x91 => self.sub_a_n(Reg::C),
            0x92 => self.sub_a_n(Reg::D),
            0x93 => self.sub_a_n(Reg::E),
            0x94 => self.sub_a_n(Reg::H),
            0x95 => self.sub_a_n(Reg::L),
            0x96 => self.sub_a_n(Reg::HL),
            0x97 => self.sub_a_n(Reg::A),
            0x98 => self.subc_a_n(Reg::B),
            0x99 => self.subc_a_n(Reg::C),
            0x9a => self.subc_a_n(Reg::D),
            0x9b => self.subc_a_n(Reg::E),
            0x9c => self.subc_a_n(Reg::H),
            0x9d => self.subc_a_n(Reg::L),
            0x9e => self.subc_a_n(Reg::HL),
            0x9f => self.subc_a_n(Reg::A),
            0xa0 => self.and_n(Reg::B),
            0xa1 => self.and_n(Reg::C),
            0xa2 => self.and_n(Reg::D),
            0xa3 => self.and_n(Reg::E),
            0xa4 => self.and_n(Reg::H),
            0xa5 => self.and_n(Reg::L),
            0xa6 => self.and_n(Reg::HL),
            0xa7 => self.and_n(Reg::A),
            0xa8 => self.xor_n(Reg::B),
            0xa9 => self.xor_n(Reg::C),
            0xaa => self.xor_n(Reg::D),
            0xab => self.xor_n(Reg::E),
            0xac => self.xor_n(Reg::H),
            0xad => self.xor_n(Reg::L),
            0xae => self.xor_n(Reg::HL),
            0xaf => self.xor_n(Reg::A),
            0xb0 => self.or_n(Reg::B),
            0xb1 => self.or_n(Reg::C),
            0xb2 => self.or_n(Reg::D),
            0xb3 => self.or_n(Reg::E),
            0xb4 => self.or_n(Reg::H),
            0xb5 => self.or_n(Reg::L),
            0xb6 => self.or_n(Reg::HL),
            0xb7 => self.or_n(Reg::A),
            0xb8 => self.cp_n(Reg::B),
            0xb9 => self.cp_n(Reg::C),
            0xba => self.cp_n(Reg::D),
            0xbb => self.cp_n(Reg::E),
            0xbc => self.cp_n(Reg::H),
            0xbd => self.cp_n(Reg::L),
            0xbe => self.cp_n(Reg::HL),
            0xbf => self.cp_n(Reg::A),
            0xc0 => self.ret_cc(self.get_flag(Flags::Z) == 0),
            0xc1 => self.pop_nn(Reg::BC),
            0xc2 => self.jp_cc_nn(self.get_flag(Flags::Z) == 0),
            0xc3 => self.jp_cc_nn(true),
            0xc4 => self.call_cc_nn(self.get_flag(Flags::Z) == 0),
            0xc5 => self.push_nn(Reg::BC),
            0xc6 => self.add_a_n(Reg::N8),
            0xc7 => self.rst_n(0x0000),
            0xc8 => self.ret_cc(self.get_flag(Flags::Z) == 1),
            0xc9 => self.ret_cc(true),
            0xca => self.jp_cc_nn(self.get_flag(Flags::Z) == 1),
            0xcb => self.cb(),
            0xcc => self.call_cc_nn(self.get_flag(Flags::Z) == 1),
            0xcd => self.call_cc_nn(true),
            0xce => self.addc_a_n(Reg::N8),
            0xcf => self.rst_n(0x0008),
            0xd0 => self.ret_cc(self.get_flag(Flags::C) == 0),
            0xd1 => self.pop_nn(Reg::DE),
            0xd2 => self.jp_cc_nn(self.get_flag(Flags::C) == 0),
            //0xd3
            0xd4 => self.call_cc_nn(self.get_flag(Flags::C) == 0),
            0xd5 => self.push_nn(Reg::DE),
            0xd6 => self.sub_a_n(Reg::N8),
            0xd7 => self.rst_n(0x0010),
            0xd8 => self.ret_cc(self.get_flag(Flags::C) == 1),
            0xd9 => {
                let address = self.mem_pop_stack();
                println!("Popping address from stack (0xdn opcode): {:04X}", address);
                self.mem_write_pc(address);
                self.interrupts.borrow_mut().set_master_enabled_on();
                8
            }
            0xda => self.jp_cc_nn(self.get_flag(Flags::C) == 1),
            //0xdb
            0xdc => self.call_cc_nn(self.get_flag(Flags::C) == 1),
            //0xdd
            0xde => self.subc_a_n(Reg::N8),
            0xdf => self.rst_n(0x0018),
            0xe0 => {
                let address = self.get_next_8();
                self.memory
                    .borrow_mut()
                    .write(0xff00 | address as u16, self.a);
                self.memory.borrow_mut().increment_program_counter(1);
                12
            }
            0xe1 => self.pop_nn(Reg::HL),
            0xe2 => {
                self.memory
                    .borrow_mut()
                    .write((0xff00 as u16).wrapping_add(self.c as u16), self.a);
                8
            }
            //0xe3
            //0xe4
            0xe5 => self.push_nn(Reg::HL),
            0xe6 => self.and_n(Reg::N8),
            0xe7 => self.rst_n(0x0020),
            0xe8 => {
                let data = self.get_next_8() as u16;
                let address = self.mem_read_sp();
                self.set_flag(Flags::Z, false);
                self.set_flag(Flags::N, false);
                self.set_flag(Flags::H, test_flag_add_16(address, data, Flags::H));
                self.set_flag(Flags::C, test_flag_add_16(address, data, Flags::C));
                self.mem_write_sp(address.wrapping_add(data));
                16
            }
            0xe9 => {
                let address = self.get_reg_u16(Reg::HL);
                self.mem_write_pc(address);
                4
            }
            0xea => self.ld_n_a(Reg::N16),
            //0xeb
            //0xec
            //0xed
            0xee => self.xor_n(Reg::N8),
            0xef => self.rst_n(0x0028),
            0xf0 => {
                let address = self.get_next_8();
                self.a = self
                    .memory
                    .borrow()
                    .read((0xff00 as u16).wrapping_add(address as u16));
                self.memory.borrow_mut().increment_program_counter(1);
                4
            }
            0xf1 => self.pop_nn(Reg::AF),
            0xf2 => {
                let data = self
                    .memory
                    .borrow()
                    .read((0xff00 as u16).wrapping_add(self.c as u16));
                self.a = data;
                8
            }
            0xf3 => self.di(),
            //0xf4
            0xf5 => self.push_nn(Reg::AF),
            0xf6 => self.or_n(Reg::N8),
            0xf7 => self.rst_n(0x0030),
            0xf8 => {
                let address = self.get_next_8();
                let sp = self.memory.borrow().get_stack_pointer();
                let effective_address = sp.wrapping_add(address as u16);
                let split_address = effective_address.to_be_bytes();
                self.h = split_address[0];
                self.l = split_address[1];
                self.set_flag(Flags::H, (sp & 0x000f) + (address as u16 & 0x000f) > 0x000f);
                self.set_flag(Flags::C, (sp & 0x00ff) + (address as u16 & 0x00ff) > 0x00ff);
                self.set_flag(Flags::Z, false);
                self.set_flag(Flags::N, false);
                12
            }
            0xf9 => {
                let address = self.get_reg_u16(Reg::HL);
                self.memory.borrow_mut().set_stack_pointer(address);
                8
            }
            0xfa => self.ld_a_n(Reg::N16),
            0xfb => self.ei(),
            //0xfc
            //0xfd
            0xfe => self.cp_n(Reg::N8),
            0xff => self.rst_n(0x0038),
            c => {
                println!("Not implemented: {:x}", c);
                6
            }
        }
    }

    fn read_memory_at_current_location(&self) -> u8 {
        self.memory
            .borrow()
            .read(self.memory.borrow().get_program_counter())
    }

    pub fn update(&mut self) -> u32 {
        self.frame_cycles = 0;
        while self.frame_cycles < MAXCYCLES {
            self.print_debug_info();
            let opcode = self.get_next_8();
            let cycles: u32 = self.execute_opcode(opcode, false);
            self.frame_cycles += cycles;
        }
        self.total_cycles += self.frame_cycles;
        self.frame_cycles
    }

    fn print_debug_info(&self) {
        if DEBUG {
            // A: 77  F: 80  (AF: 7780)
            println!(
                "A: {:02X}  F: {:02X}  (AF: {:04X})",
                self.a,
                self.f,
                self.get_reg_u16(Reg::AF)
            );
            // B: 00  C: 00  (BC: 0000)
            println!(
                "B: {:02X}  C: {:02X}  (BC: {:04X})",
                self.b,
                self.c,
                self.get_reg_u16(Reg::BC)
            );
            // D: 00  E: 08  (DE: 0008)
            println!(
                "D: {:02X}  E: {:02X}  (DE: {:04X})",
                self.d,
                self.e,
                self.get_reg_u16(Reg::DE)
            );
            // H: 00  L: 7C  (HL: 007C)
            println!(
                "H: {:02X}  L: {:02X}  (HL: {:04X})",
                self.h,
                self.l,
                self.get_reg_u16(Reg::HL)
            );
            // PC: 0453  SP: DFFF
            println!(
                "PC: {:04X}  SP: {:04X}",
                self.mem_read_pc(),
                self.mem_read_sp(),
            );
            // F: [Z---]
            // ROM: 01  RAM: 00  WRAM: 01  VRAM: 00
            // IE: 00  IF: E0  IME: 0
            // LCDC: 91  STAT: 87  LY: 00
            // Next video mode: 9
            // 00:0453:  218F0B	ld hl, $B8F
            let opcode = self.read_memory_at_current_location();
            println!(
                "00:{:04X}: {:02X}{:04X}      {} -> {:03X}",
                self.mem_read_pc(),
                opcode,
                self.get_next_16_debug(),
                self.get_instruction_at(opcode),
                self.get_next_16_debug().swap_bytes()
            );
            // print!("\x1B[2J\x1B[1;1H");
            // println!("CPU: -------------------------------");
            // println!(
            //     "A: {:02X}, B: {:02X}, C: {:02X}, D: {:02X}",
            //     self.a, self.b, self.c, self.d
            // );

            // println!(
            //     "E: {:x}, F: {:x}, H: {:x}, L: {:x}",
            //     self.e, self.f, self.h, self.l
            // );
            // println!(
            //     "PC: {:x}, SP: {:x}",
            //     self.memory.borrow().get_program_counter(),
            //     self.memory.borrow().get_stack_pointer()
            // );
            // println!(
            //     "Z: {}, N: {}, H: {}, C: {}",
            //     self.get_flag(Flags::Z),
            //     self.get_flag(Flags::N),
            //     self.get_flag(Flags::H),
            //     self.get_flag(Flags::C)
            // );
            // let opcode = self.read_memory_at_current_location();
            // println!("Next instruction to execute: {:x}", opcode);
            // println!(
            //     "Disassembled instruction: \n     {}",
            //     self.get_instruction_at(opcode)
            // );

            // println!("Total Cycles: {}", self.total_cycles);
            // if self.memory.borrow().get_program_counter() == 0x2817 {
            //     println!("WE HAVE VISUALS");
            //     println!("WE HAVE VISUALS");
            //     println!("WE HAVE VISUALS");
            //     println!("WE HAVE VISUALS");
            //     println!("WE HAVE VISUALS");
            //     panic!();
            // }
            // if self.total_cycles > 7_480_232 {
            //     panic!("UI should be loaded by now");
            // }
        }
        if STEPS {
            use std::io::{stdin, stdout, Read, Write};
            let mut stdout = stdout();
            stdout.write_all(b"Press Enter to continue...").unwrap();
            stdout.flush().unwrap();
            stdin().read_exact(&mut [0]).unwrap();
        }
    }
}
