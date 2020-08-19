use super::emulator::Emulator;
use super::utils::get_bit_at;
use super::utils::*;

pub fn update(emu: &mut Emulator) {
    let request = emu.memory.read(0xff0f);
    let interrupt_enable = emu.memory.read(0xffff);
    if request > 0 && emu.timers.master_enabled {
        for bit in 0..5 {
            emu.timers.is_halted = false;
            if get_bit_at(request, bit) && get_bit_at(interrupt_enable, bit) {
                emu.timers.clear_master_enabled();
                interrupt_execution(emu, request, bit);
                break;
            }
        }
    }
}

pub fn is_interrupt_requested(emu: &Emulator, bit: u8) -> bool {
    let lcd_status = emu.memory.read(0xff41);
    get_bit_at(lcd_status, bit)
}

pub fn interrupt_execution(emu: &mut Emulator, request: u8, interrupt: u8) {
    let pc = emu.memory.get_program_counter();
    emu.push_to_stack_hi(pc);
    let new_pc;
    let interrupt_enable = emu.mem_read(0xffff);
    if !get_bit_at(interrupt_enable, interrupt) {
        new_pc = 0;
    } else {
        emu.push_to_stack_lo(pc);
        let clear_request = clear_bit_at(request, interrupt);
        emu.mem_write(0xff0f, clear_request);
        new_pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => return,
        };
    }
    emu.take_cycle();
    emu.memory.set_program_counter(new_pc);
}

pub fn request_interrupt(emu: &mut Emulator, bit: u8) {
    let interrupt_flags = emu.memory.read(0xff0f);
    let modified_flag = set_bit_at(interrupt_flags, bit);
    emu.memory.write(0xff0f, modified_flag);
    emu.timers.is_halted = false;
}
