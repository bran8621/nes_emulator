/*
TSX: transfer stack pointer to X
TXS: transfer X to stack pointer
PHA: push accumulator on stack
PHP: push processor status on stack
PLA: pull accumulator from stack
PLP: pull process status from stack
*/

use crate::cpu::CpuFlags;
use crate::cpu::CPU;


fn php(cpu: &mut CPU) {
        let mut flags = cpu.status.clone();
        flags.insert(CpuFlags::BREAK);
        flags.insert(CpuFlags::BREAK2);
        cpu.stack_push(flags.bits());
}

fn pla(cpu: &mut CPU) {
        let data = cpu.stack_pop();
        cpu.set_register_a(data);
}

fn plp(cpu: &mut CPU) {
        let flags = cpu.stack_pop();
        cpu.status = CpuFlags::from_bits_truncate(flags);
        cpu.status.remove(CpuFlags::BREAK);
        cpu.status.insert(CpuFlags::BREAK2);
}

fn pha(cpu: &mut CPU) {
        cpu.stack_push(cpu.register_a);
}

fn tsx(cpu: &mut CPU) {
        cpu.register_x = cpu.stack_pointer;
        cpu.update_zero_and_negative_flags(cpu.register_x);
}

fn txs(cpu: &mut CPU) {
        cpu.stack_pointer = cpu.register_x;
}



