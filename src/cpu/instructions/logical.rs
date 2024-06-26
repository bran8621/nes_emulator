/*
AND: logical AND
EOR: exclusive OR
ORA: logical inclusive OR
BIT: bit test
*/

use crate::cpu::CPU;

fn and(cpu: &mut CPU, mode: &AddressingMode) {
       cpu.perform_operation_and_update_flags(mode, |a, b| a & b);
}

fn eor(cpu: &mut CPU, mode: &AddressingMode) {
       cpu.perform_operation_and_update_flags(mode, |a, b| a ^ b);
}

fn ora(cpu: &mut CPU, mode: &AddressingMode) {
   cpu.perform_operation_and_update_flags(mode, |a, b| a | b);
}

fn bit(cpu: &mut CPU, mode: &AddressingMode) {
    let addr =cpu.get_operand_address(mode);
    let data =cpu.mem_read(addr);
    let and =cpu.register_a & data;
    if and == 0 {
       cpu.status.insert(CpuFlags::ZERO);
    } else {
       cpu.status.remove(CpuFlags::ZERO);
    }

   cpu.status.set(CpuFlags::NEGATIVE, data & 0b10000000 > 0);
   cpu.status.set(CpuFlags::OVERFLOW, data & 0b01000000 > 0);
}
