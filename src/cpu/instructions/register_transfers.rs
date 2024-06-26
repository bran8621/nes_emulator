/*
TAX: transfer accumulator to X
TAY: transfer accumulator to Y
TXA: transfer X to accumulator 
TYA: transfer Y to accumulator
*/

use crate::cpu::CPU;

fn tax(cpu: &mut CPU) {
    cpu.register_x = cpu.register_a;
    cpu.update_zero_and_negative_flags(cpu.register_x);
}

fn tay(cpu: &mut CPU) {
    cpu.register_y = cpu.register_a;
    cpu.update_zero_and_negative_flags(cpu.register_y);
}

fn txa(cpu: &mut CPU) {
    cpu.register_a = cpu.register_x;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

fn tya(cpu: &mut CPU) {
    cpu.register_a = cpu.register_y;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

