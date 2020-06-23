use super::memory::Memory;
use super::timers::Timers;
use super::utils::get_bit_at;

pub fn update(timers: &mut Timers, memory: &mut Memory) {
    if timers.master_enabled {
        let request = memory.read(0xff0f);
        let interrupt_enable = memory.read(0xffff);
        if request > 0 {
            for bit in 0..5 {
                if get_bit_at(request, bit) && get_bit_at(interrupt_enable, bit) {
                    timers.master_enabled = false;
                    memory.interrupt_execution(request, bit);
                }
            }
        }
    }
}
