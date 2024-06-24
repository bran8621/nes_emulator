/*
BRK: force an interrupt
NOP: no operation
RTI: return from interrupt
*/

use crate::cpu::CPU;

fn brk(cpu: &mut CPU){
    cpu.program_counter += 1;
    cpu.status.insert(CpuFlags::BREAK);
    cpu.stack_push_u16(cpu.program_counter);
    cpu.stack_push(cpu.status.bits());
    cpu.status.insert(CpuFlags::INTERRUPT_DISABLE);
    cpu.program_counter = cpu.mem_read_u16(0xFFFE);
}
fn rti(cpu: &mut cpu){
    let flags_bits = cpu.stack_pop();
    let flags = CpuFlags::from_bits_truncate(flags_bits);
    cpu.status = flags;
    cpu.status.remove(CpuFlags::BREAK);
    cpu.status.insert(CpuFlags::BREAK2);
    cpu.program_counter = cpu.stack_pop_u16();
}