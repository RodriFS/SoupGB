use super::constants::*;
use super::memory::Memory;
use super::registers::Flags;
use super::registers::Registers;

fn print_instruction(instruction: u8, code: u16) {
    match instruction {
        0x00 => println!("NOP"),                           // 0x00
        0x01 => println!("LD BC, 0x{:X}", code),           // 0x01
        0x02 => println!("LD (BC), A"),                    // 0x02
        0x03 => println!("INC BC"),                        // 0x03
        0x04 => println!("INC B"),                         // 0x04
        0x05 => println!("DEC B"),                         // 0x05
        0x06 => println!("LD B, 0x{:X}", code),            // 0x06
        0x07 => println!("RLCA"),                          // 0x07
        0x08 => println!("LD (0x{:X}), SP", code),         // 0x08
        0x09 => println!("ADD HL, BC"),                    // 0x09
        0x0a => println!("LD A, (BC)"),                    // 0x0a
        0x0b => println!("DEC BC"),                        // 0x0b
        0x0c => println!("INC C"),                         // 0x0c
        0x0d => println!("DEC C"),                         // 0x0d
        0x0e => println!("LD C, 0x{:X}", code),            // 0x0e
        0x0f => println!("RRCA"),                          // 0x0f
        0x10 => println!("STOP"),                          // 0x10
        0x11 => println!("LD DE, 0x{:X}", code),           // 0x11
        0x12 => println!("LD (DE), A"),                    // 0x12
        0x13 => println!("INC DE"),                        // 0x13
        0x14 => println!("INC D"),                         // 0x14
        0x15 => println!("DEC D"),                         // 0x15
        0x16 => println!("LD D, 0x{:X}", code),            // 0x16
        0x17 => println!("RLA"),                           // 0x17
        0x18 => println!("JR 0x{:X}", code),               // 0x18
        0x19 => println!("ADD HL, DE"),                    // 0x19
        0x1a => println!("LD A, (DE)"),                    // 0x1a
        0x1b => println!("DEC DE"),                        // 0x1b
        0x1c => println!("INC E"),                         // 0x1c
        0x1d => println!("DEC E"),                         // 0x1d
        0x1e => println!("LD E, 0x{:X}", code),            // 0x1e
        0x1f => println!("RRA"),                           // 0x1f
        0x20 => println!("JR NZ, 0x{:X}", code),           // 0x20
        0x21 => println!("LD HL, 0x{:X}", code),           // 0x21
        0x22 => println!("LDI (HL), A"),                   // 0x22
        0x23 => println!("INC HL"),                        // 0x23
        0x24 => println!("INC H"),                         // 0x24
        0x25 => println!("DEC H"),                         // 0x25
        0x26 => println!("LD H, 0x{:X}", code),            // 0x26
        0x27 => println!("DAA"),                           // 0x27
        0x28 => println!("JR Z, 0x{:X}", code),            // 0x28
        0x29 => println!("ADD HL, HL"),                    // 0x29
        0x2a => println!("LDI A, (HL)"),                   // 0x2a
        0x2b => println!("DEC HL"),                        // 0x2b
        0x2c => println!("INC L"),                         // 0x2c
        0x2d => println!("DEC L"),                         // 0x2d
        0x2e => println!("LD L, 0x{:X}", code),            // 0x2e
        0x2f => println!("CPL"),                           // 0x2f
        0x30 => println!("JR NC, 0x{:X}", code),           // 0x30
        0x31 => println!("LD SP, 0x{:X}", code),           // 0x31
        0x32 => println!("LDD (HL), A"),                   // 0x32
        0x33 => println!("INC SP"),                        // 0x33
        0x34 => println!("INC (HL)"),                      // 0x34
        0x35 => println!("DEC (HL)"),                      // 0x35
        0x36 => println!("LD (HL), 0x{:X}", code),         // 0x36
        0x37 => println!("SCF"),                           // 0x37
        0x38 => println!("JR C, 0x{:X}", code),            // 0x38
        0x39 => println!("ADD HL, SP"),                    // 0x39
        0x3a => println!("LDD A, (HL)"),                   // 0x3a
        0x3b => println!("DEC SP"),                        // 0x3b
        0x3c => println!("INC A"),                         // 0x3c
        0x3d => println!("DEC A"),                         // 0x3d
        0x3e => println!("LD A, 0x{:X}", code),            // 0x3e
        0x3f => println!("CCF"),                           // 0x3f
        0x40 => println!("LD B, B"),                       // 0x40
        0x41 => println!("LD B, C"),                       // 0x41
        0x42 => println!("LD B, D"),                       // 0x42
        0x43 => println!("LD B, E"),                       // 0x43
        0x44 => println!("LD B, H"),                       // 0x44
        0x45 => println!("LD B, L"),                       // 0x45
        0x46 => println!("LD B, (HL)"),                    // 0x46
        0x47 => println!("LD B, A"),                       // 0x47
        0x48 => println!("LD C, B"),                       // 0x48
        0x49 => println!("LD C, C"),                       // 0x49
        0x4a => println!("LD C, D"),                       // 0x4a
        0x4b => println!("LD C, E"),                       // 0x4b
        0x4c => println!("LD C, H"),                       // 0x4c
        0x4d => println!("LD C, L"),                       // 0x4d
        0x4e => println!("LD C, (HL)"),                    // 0x4e
        0x4f => println!("LD C, A"),                       // 0x4f
        0x50 => println!("LD D, B"),                       // 0x50
        0x51 => println!("LD D, C"),                       // 0x51
        0x52 => println!("LD D, D"),                       // 0x52
        0x53 => println!("LD D, E"),                       // 0x53
        0x54 => println!("LD D, H"),                       // 0x54
        0x55 => println!("LD D, L"),                       // 0x55
        0x56 => println!("LD D, (HL)"),                    // 0x56
        0x57 => println!("LD D, A"),                       // 0x57
        0x58 => println!("LD E, B"),                       // 0x58
        0x59 => println!("LD E, C"),                       // 0x59
        0x5a => println!("LD E, D"),                       // 0x5a
        0x5b => println!("LD E, E"),                       // 0x5b
        0x5c => println!("LD E, H"),                       // 0x5c
        0x5d => println!("LD E, L"),                       // 0x5d
        0x5e => println!("LD E, (HL)"),                    // 0x5e
        0x5f => println!("LD E, A"),                       // 0x5f
        0x60 => println!("LD H, B"),                       // 0x60
        0x61 => println!("LD H, C"),                       // 0x61
        0x62 => println!("LD H, D"),                       // 0x62
        0x63 => println!("LD H, E"),                       // 0x63
        0x64 => println!("LD H, H"),                       // 0x64
        0x65 => println!("LD H, L"),                       // 0x65
        0x66 => println!("LD H, (HL)"),                    // 0x66
        0x67 => println!("LD H, A"),                       // 0x67
        0x68 => println!("LD L, B"),                       // 0x68
        0x69 => println!("LD L, C"),                       // 0x69
        0x6a => println!("LD L, D"),                       // 0x6a
        0x6b => println!("LD L, E"),                       // 0x6b
        0x6c => println!("LD L, H"),                       // 0x6c
        0x6d => println!("LD L, L"),                       // 0x6d
        0x6e => println!("LD L, (HL)"),                    // 0x6e
        0x6f => println!("LD L, A"),                       // 0x6f
        0x70 => println!("LD (HL), B"),                    // 0x70
        0x71 => println!("LD (HL), C"),                    // 0x71
        0x72 => println!("LD (HL), D"),                    // 0x72
        0x73 => println!("LD (HL), E"),                    // 0x73
        0x74 => println!("LD (HL), H"),                    // 0x74
        0x75 => println!("LD (HL), L"),                    // 0x75
        0x76 => println!("HALT"),                          // 0x76
        0x77 => println!("LD (HL), A"),                    // 0x77
        0x78 => println!("LD A, B"),                       // 0x78
        0x79 => println!("LD A, C"),                       // 0x79
        0x7a => println!("LD A, D"),                       // 0x7a
        0x7b => println!("LD A, E"),                       // 0x7b
        0x7c => println!("LD A, H"),                       // 0x7c
        0x7d => println!("LD A, L"),                       // 0x7d
        0x7e => println!("LD A, (HL)"),                    // 0x7e
        0x7f => println!("LD A, A"),                       // 0x7f
        0x80 => println!("ADD A, B"),                      // 0x80
        0x81 => println!("ADD A, C"),                      // 0x81
        0x82 => println!("ADD A, D"),                      // 0x82
        0x83 => println!("ADD A, E"),                      // 0x83
        0x84 => println!("ADD A, H"),                      // 0x84
        0x85 => println!("ADD A, L"),                      // 0x85
        0x86 => println!("ADD A, (HL)"),                   // 0x86
        0x87 => println!("ADD A"),                         // 0x87
        0x88 => println!("ADC B"),                         // 0x88
        0x89 => println!("ADC C"),                         // 0x89
        0x8a => println!("ADC D"),                         // 0x8a
        0x8b => println!("ADC E"),                         // 0x8b
        0x8c => println!("ADC H"),                         // 0x8c
        0x8d => println!("ADC L"),                         // 0x8d
        0x8e => println!("ADC (HL)"),                      // 0x8e
        0x8f => println!("ADC A"),                         // 0x8f
        0x90 => println!("SUB B"),                         // 0x90
        0x91 => println!("SUB C"),                         // 0x91
        0x92 => println!("SUB D"),                         // 0x92
        0x93 => println!("SUB E"),                         // 0x93
        0x94 => println!("SUB H"),                         // 0x94
        0x95 => println!("SUB L"),                         // 0x95
        0x96 => println!("SUB (HL)"),                      // 0x96
        0x97 => println!("SUB A"),                         // 0x97
        0x98 => println!("SBC B"),                         // 0x98
        0x99 => println!("SBC C"),                         // 0x99
        0x9a => println!("SBC D"),                         // 0x9a
        0x9b => println!("SBC E"),                         // 0x9b
        0x9c => println!("SBC H"),                         // 0x9c
        0x9d => println!("SBC L"),                         // 0x9d
        0x9e => println!("SBC (HL)"),                      // 0x9e
        0x9f => println!("SBC A"),                         // 0x9f
        0xa0 => println!("AND B"),                         // 0xa0
        0xa1 => println!("AND C"),                         // 0xa1
        0xa2 => println!("AND D"),                         // 0xa2
        0xa3 => println!("AND E"),                         // 0xa3
        0xa4 => println!("AND H"),                         // 0xa4
        0xa5 => println!("AND L"),                         // 0xa5
        0xa6 => println!("AND (HL)"),                      // 0xa6
        0xa7 => println!("AND A"),                         // 0xa7
        0xa8 => println!("XOR B"),                         // 0xa8
        0xa9 => println!("XOR C"),                         // 0xa9
        0xaa => println!("XOR D"),                         // 0xaa
        0xab => println!("XOR E"),                         // 0xab
        0xac => println!("XOR H"),                         // 0xac
        0xad => println!("XOR L"),                         // 0xad
        0xae => println!("XOR (HL)"),                      // 0xae
        0xaf => println!("XOR A"),                         // 0xaf
        0xb0 => println!("OR B"),                          // 0xb0
        0xb1 => println!("OR C"),                          // 0xb1
        0xb2 => println!("OR D"),                          // 0xb2
        0xb3 => println!("OR E"),                          // 0xb3
        0xb4 => println!("OR H"),                          // 0xb4
        0xb5 => println!("OR L"),                          // 0xb5
        0xb6 => println!("OR (HL)"),                       // 0xb6
        0xb7 => println!("OR A"),                          // 0xb7
        0xb8 => println!("CP B"),                          // 0xb8
        0xb9 => println!("CP C"),                          // 0xb9
        0xba => println!("CP D"),                          // 0xba
        0xbb => println!("CP E"),                          // 0xbb
        0xbc => println!("CP H"),                          // 0xbc
        0xbd => println!("CP L"),                          // 0xbd
        0xbe => println!("CP (HL)"),                       // 0xbe
        0xbf => println!("CP A"),                          // 0xbf
        0xc0 => println!("RET NZ"),                        // 0xc0
        0xc1 => println!("POP BC"),                        // 0xc1
        0xc2 => println!("JP NZ, 0x{:X}", code),           // 0xc2
        0xc3 => println!("JP 0x{:X}", code),               // 0xc3
        0xc4 => println!("CALL NZ, 0x{:X}", code),         // 0xc4
        0xc5 => println!("PUSH BC"),                       // 0xc5
        0xc6 => println!("ADD A, 0x{:X}", code),           // 0xc6
        0xc7 => println!("RST 0x00"),                      // 0xc7
        0xc8 => println!("RET Z"),                         // 0xc8
        0xc9 => println!("RET"),                           // 0xc9
        0xca => println!("JP Z, 0x{:X}", code),            // 0xca
        0xcb => println!("CB {:X}", code),                 // 0xcb
        0xcc => println!("CALL Z, 0x{:X}", code),          // 0xcc
        0xcd => println!("CALL 0x{:X}", code),             // 0xcd
        0xce => println!("ADC 0x{:X}", code),              // 0xce
        0xcf => println!("RST 0x08"),                      // 0xcf
        0xd0 => println!("RET NC"),                        // 0xd0
        0xd1 => println!("POP DE"),                        // 0xd1
        0xd2 => println!("JP NC, 0x{:X}", code),           // 0xd2
        0xd3 => println!("UNKNOWN"),                       // 0xd3
        0xd4 => println!("CALL NC, 0x{:X}", code),         // 0xd4
        0xd5 => println!("PUSH DE"),                       // 0xd5
        0xd6 => println!("SUB 0x{:X}", code),              // 0xd6
        0xd7 => println!("RST 0x10"),                      // 0xd7
        0xd8 => println!("RET C"),                         // 0xd8
        0xd9 => println!("RETI"),                          // 0xd9
        0xda => println!("JP C, 0x{:X}", code),            // 0xda
        0xdb => println!("UNKNOWN"),                       // 0xdb
        0xdc => println!("CALL C, 0x{:X}", code),          // 0xdc
        0xdd => println!("UNKNOWN"),                       // 0xdd
        0xde => println!("SBC 0x{:X}", code),              // 0xde
        0xdf => println!("RST 0x18"),                      // 0xdf
        0xe0 => println!("LD (0xFF00 + 0x{:X}), A", code), // 0xe0
        0xe1 => println!("POP HL"),                        // 0xe1
        0xe2 => println!("LD (0xFF00 + C), A"),            // 0xe2
        0xe3 => println!("UNKNOWN"),                       // 0xe3
        0xe4 => println!("UNKNOWN"),                       // 0xe4
        0xe5 => println!("PUSH HL"),                       // 0xe5
        0xe6 => println!("AND 0x{:X}", code),              // 0xe6
        0xe7 => println!("RST 0x20"),                      // 0xe7
        0xe8 => println!("ADD SP,0x{:X}", code),           // 0xe8
        0xe9 => println!("JP HL"),                         // 0xe9
        0xea => println!("LD (0x{:X}), A", code),          // 0xea
        0xeb => println!("UNKNOWN"),                       // 0xeb
        0xec => println!("UNKNOWN"),                       // 0xec
        0xed => println!("UNKNOWN"),                       // 0xed
        0xee => println!("XOR 0x{:X}", code),              // 0xee
        0xef => println!("RST 0x28"),                      // 0xef
        0xf0 => println!("LD A, (0xFF00 + 0x{:X})", code), // 0xf0
        0xf1 => println!("POP AF"),                        // 0xf1
        0xf2 => println!("LD A, (0xFF00 + C)"),            // 0xf2
        0xf3 => println!("DI"),                            // 0xf3
        0xf4 => println!("UNKNOWN"),                       // 0xf4
        0xf5 => println!("PUSH AF"),                       // 0xf5
        0xf6 => println!("OR 0x{:X}", code),               // 0xf6
        0xf7 => println!("RST 0x30"),                      // 0xf7
        0xf8 => println!("LD HL, SP+0x{:X}", code),        // 0xf8
        0xf9 => println!("LD SP, HL"),                     // 0xf9
        0xfa => println!("LD A, (0x{:X})", code),          // 0xfa
        0xfb => println!("EI"),                            // 0xfb
        0xfc => println!("UNKNOWN"),                       // 0xfc
        0xfd => println!("UNKNOWN"),                       // 0xfd
        0xfe => println!("CP 0x{:X}", code),               // 0xfe
        0xff => println!("RST 0x38"),                      // 0xff
    }
}

pub fn steps() {
    if STEPS {
        use std::io::{stdin, stdout, Read, Write};
        let mut stdout = stdout();
        stdout.write_all(b"Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        stdin().read_exact(&mut [0]).unwrap();
    }
}

pub fn print_debug_registers_info(registers: &Registers) {
    if !DEBUG_CPU {
        return;
    }
    let flags = (
        registers.get_flag(Flags::Z),
        registers.get_flag(Flags::N),
        registers.get_flag(Flags::H),
        registers.get_flag(Flags::C),
    );
    let (z, n, h, c) = flags;
    let reg = (
        registers.get_af(),
        registers.get_bc(),
        registers.get_de(),
        registers.get_hl(),
    );
    let (af, bc, de, hl) = reg;
    let [reg_a, reg_f] = af.to_be_bytes();
    let [reg_b, reg_c] = bc.to_be_bytes();
    let [reg_d, reg_e] = de.to_be_bytes();
    let [reg_h, reg_l] = hl.to_be_bytes();
    println!("\nCPU: -----------------------------");
    println!("A: {:02X}  F: {:02X}  (AF: {:04X})", reg_a, reg_f, af);
    println!("B: {:02X}  C: {:02X}  (BC: {:04X})", reg_b, reg_c, bc);
    println!("D: {:02X}  E: {:02X}  (DE: {:04X})", reg_d, reg_e, de);
    println!("H: {:02X}  L: {:02X}  (HL: {:04X})", reg_h, reg_l, hl);
    println!("Z: {}, N: {}, H: {}, C: {}", z, n, h, c);
}

pub fn print_debug_memory_info(memory: &Memory) {
    if DEBUG_CPU {
        let opcode = memory.get_byte_debug();
        let n16 = memory.get_word_debug();
        let pc = memory.get_program_counter();
        let sp = memory.get_stack_pointer();
        println!("PC: {:04X}  SP: {:04X}", pc, sp);
        println!("00:{:04X}: | {:02X}{:04X}", pc, opcode, n16);
        print_instruction(opcode, n16.swap_bytes());
    }
    if DEBUG_MEMORY {
        let cromb = memory.memory_bank;
        let cramb = memory.get_bank2_as_low();
        let mbt = memory.memory_bank_type.clone();
        let ire = memory.is_ram_enabled;
        let bm = memory.banking_mode.clone();
        println!("MEMORY: -----------------------------");
        println!("Current ROM bank: {}", cromb);
        println!("Current RAM bank: {}", cramb);
        println!("memory bank type: {:?}", mbt);
        println!("is_ram_enabled: {}", ire);
        println!("banking_mode: {:?}", bm);
    }
    if DEBUG_GPU {
        let lcdc = memory.read(0xff40);
        let lcd_stat = memory.read(0xff41);
        let ly = memory.read(0xff44);
        println!("GPU: -----------------------------");
        println!("LCDC: {:02X}  STAT: {:02X}  LY: {}", lcdc, lcd_stat, ly);
    }
    if DEBUG_TIMERS {
        let cf = memory.get_tac();
        let te = memory.get_is_clock_enabled();
        let dr = memory.get_div();
        let tima = memory.get_tima();
        let tma = memory.get_tma();
        let tac = memory.get_tac();
        println!("TIMERS: -----------------------------");
        println!("Timers frequency: {}", cf);
        println!("Timer enabled: {}", te);
        println!("0xff04 (DIV) Divider counter: {:02X}", dr);
        println!("0xff05 (TIMA) Timer counter: {:02X}", tima);
        println!("0xff06 (TMA) Timer modulo: {:02X}", tma);
        println!("0xff07 (TAC) Timer control: {:02X}", tac);
    }
}
