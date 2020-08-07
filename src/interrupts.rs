use super::emulator::Emulator;
use super::utils::get_bit_at;

pub fn update(emu: &mut Emulator) {
    if emu.timers.master_enabled {
        let request = emu.memory.read(0xff0f);
        let interrupt_enable = emu.memory.read(0xffff);
        if request > 0 {
            for bit in 0..5 {
                if get_bit_at(request, bit) && get_bit_at(interrupt_enable, bit) {
                    emu.timers.master_enabled = false;
                    emu.memory.interrupt_execution(request, bit);
                }
            }
        }
    }
}
