/*
BRK: force an interrupt
NOP: no operation
RTI: return from interrupt
*/

use crate::cpu::CPU;

fn brk(cpu: &mut CPU){
    self.program_counter += 1;
    self.status.insert(CpuFlags::BREAK);
    self.stack_push_u16(self.program_counter);
    self.stack_push(self.status.bits());
    self.status.insert(CpuFlags::INTERRUPT_DISABLE);
    self.program_counter = self.mem_read_u16(0xFFFE);
}
fn rti(&mut self){
    let flags_bits = self.stack_pop();
    let flags = CpuFlags::from_bits_truncate(flags_bits);
    self.status = flags;
    self.status.remove(CpuFlags::BREAK);
    self.status.insert(CpuFlags::BREAK2);
    self.program_counter = self.stack_pop_u16();
}