use super::constants::*;
use super::interrupts::Interrupts;
use super::memory::Memory;
use super::utils::get_bit_at;
use byteorder::{ByteOrder, LittleEndian};
use std::cell::RefCell;
use std::rc::Rc;

enum Flags {
    Z,
    N,
    H,
    C,
}

pub struct Cpu {
    memory: Rc<RefCell<Memory>>,
    interrupts: Rc<RefCell<Interrupts>>,
    frame_cycles: u32,
    total_cycles: u32,
    stack_pointer: u16,
    program_counter: u16,
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
            memory,
            interrupts,
            frame_cycles: 0,
            total_cycles: 0,
            stack_pointer: 0xFFFE,
            program_counter: 0x100,
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
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
        let c = self.get_counter() as usize;
        let mem = self.memory.borrow();
        let address = mem.read_range(c..(c + 2));
        let data = [address[0], address[1]];
        LittleEndian::read_u16(&data)
    }
    fn get_next_8(&self) -> i8 {
        self.read_memory_at_current_location() as i8
    }

    fn get_instruction_at(&self, address: u8) -> &str {
        self.instructions[address as usize]
    }

    fn cpu_nop(&mut self) -> u32 {
        4
    }
    fn cpu_jp_16(&mut self) -> u32 {
        if self.get_flag(Flags::N) == 0 {
            let address = self.get_next_16();
            self.program_counter = address;
        }
        16
    }
    fn cpu_cp_8(&mut self) -> u32 {
        if self.get_flag(Flags::N) == 0 {
            let address = self.get_next_8();
            self.set_flag(Flags::Z, self.a == address as u8);
            self.set_flag(Flags::N, true);
            self.set_flag(Flags::H, (self.a & 0x0f) < (address as u8 & 0x0f));
            self.set_flag(Flags::C, self.a < address as u8);
            self.program_counter += 1;
        } else {
            self.program_counter += 1;
        }
        8
    }
    fn cpu_jr_cc(&mut self, condition: bool) -> u32 {
        if condition {
            let address = self.get_next_8();
            self.program_counter = self.program_counter.wrapping_add(address as u16);
            self.program_counter += 1;
            3
        } else {
            self.program_counter += 1;
            2
        }
    }
    fn cpu_jr(&mut self) -> u32 {
        let address = self.get_next_8();
        println!("{:x}", address);
        self.program_counter += address as u16;
        self.program_counter += 1;
        3
    }
    fn cpu_xor(&mut self, value: u8) -> u32 {
        let result = value as u16 ^ self.a as u16;
        self.set_flag(Flags::Z, result == 0);
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::H, false);
        self.set_flag(Flags::C, false);
        self.a = 0;
        1
    }
    fn cpu_ld_16_a(&mut self) -> u32 {
        let address = self.get_next_16();
        self.memory.borrow_mut().write(address, self.a);
        self.program_counter += 2;
        4
    }
    fn cpu_di(&mut self) -> u32 {
        self.interrupts.borrow_mut().clear_master_enabled();
        1
    }
    fn cpu_ld_8_a(&mut self, offset: u16) -> u32 {
        let address = self.get_next_8();
        self.memory
            .borrow_mut()
            .write(offset.wrapping_add(address as u16), self.a);
        self.program_counter += 1;
        4
    }
    fn cpu_ld_a_8(&mut self, offset: u16) -> u32 {
        let address = self.get_next_8();
        self.a = self
            .memory
            .borrow()
            .read(offset.wrapping_add(address as u16));
        self.program_counter += 1;
        12
    }
    fn cpu_call_16(&mut self) -> u32 {
        let address = self.get_next_16();
        self.program_counter += 2;
        self.stack_pointer -= 2;
        self.program_counter = address as u16;
        6
    }
    fn cpu_ld_b_a(&mut self) -> u32 {
        self.b = self.a;
        4
    }
    fn cpu_cb(&mut self) -> u32 {
        let address = self.get_next_8();
        println!("Executing callback opcode {:x}", address);
        self.execute_opcode(address as u8);
        4
    }
    fn cpu_res_b_a(&mut self, bit: u8) -> u32 {
        self.a &= !(bit);
        8
    }

    // fn add_a(&mut self) -> u32 {
    //     let result = self.af.0 * 2;
    //     self.set_flag(Flags::Z, self.af.0 == result);
    //     self.set_flag(Flags::N, false);
    //     self.set_flag(Flags::H, (self.af.0 & 0x0f) < (result & 0x0f));
    //     self.set_flag(Flags::C, self.af.0 < result);
    //     4
    // }

    fn execute_opcode(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x00 => self.cpu_nop(),
            0xc3 => self.cpu_jp_16(),
            0xfe => self.cpu_cp_8(),
            0x28 => {
                let z = self.get_flag(Flags::Z);
                self.cpu_jr_cc(z == 1)
            }
            0xaf => self.cpu_xor(self.a),
            0x18 => self.cpu_jr(),
            0xea => self.cpu_ld_16_a(),
            0xf3 => self.cpu_di(),
            0xe0 => self.cpu_ld_8_a(0xff00),
            0x3e => self.cpu_ld_a_8(0),
            0xcd => self.cpu_call_16(),
            0xf0 => self.cpu_ld_a_8(0xff00),
            0x47 => self.cpu_ld_b_a(),
            0xcb => self.cpu_cb(),
            0x87 => self.cpu_res_b_a(0x1),
            0x20 => {
                let f = self.get_flag(Flags::Z);
                self.cpu_jr_cc(f == 0)
            }
            // 0x87 => self.add_a(),
            _ => unimplemented!(),
        }
    }

    fn read_memory_at_current_location(&self) -> u8 {
        self.memory.borrow().read(self.get_counter())
    }

    fn execute_next_opcode(&mut self) -> u32 {
        let opcode = self.memory.borrow().read(self.get_counter());
        self.program_counter += 1;
        self.execute_opcode(opcode)
    }

    fn get_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn update(&mut self) -> u32 {
        self.frame_cycles = 0;
        while self.frame_cycles < MAXCYCLES {
            self.print_debug_info();
            let cycles: u32 = self.execute_next_opcode();
            self.frame_cycles += cycles;
        }
        self.total_cycles += self.frame_cycles;
        self.frame_cycles
    }

    fn print_debug_info(&self) {
        println!("CPU: -------------------------------");
        println!(
            "A: {:x}, B: {:x}, C: {:x}, D: {:x}",
            self.a, self.b, self.c, self.d
        );
        println!(
            "E: {:x}, F: {:x}, H: {:x}, L: {:x}",
            self.e, self.f, self.h, self.l
        );
        println!(
            "PC: {:x}, SP: {:x}",
            self.program_counter, self.stack_pointer
        );
        println!(
            "Z: {}, N: {}, H: {}, C: {}",
            self.get_flag(Flags::Z),
            self.get_flag(Flags::N),
            self.get_flag(Flags::H),
            self.get_flag(Flags::C)
        );
        let opcode = self.read_memory_at_current_location();
        println!("Next instruction to execute: {:x}", opcode);
        println!(
            "Disassembled instruction: \n     {}",
            self.get_instruction_at(opcode)
        );
        println!("Total Cycles: {}", self.total_cycles);
        if self.program_counter == 0x2817 {
            println!("WE HAVE VISUALS");
            println!("WE HAVE VISUALS");
            println!("WE HAVE VISUALS");
            println!("WE HAVE VISUALS");
            println!("WE HAVE VISUALS");
            panic!();
        }
    }
}
