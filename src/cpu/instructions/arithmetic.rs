/*
ADC: add with carry
SBC: subtract with carry
CMP: compare accumulator
CPX: compare X register
CPY: compare Y register 
*/

use crate::cpu::cpu;


fn adc(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(mode);
        let value = cpu.mem_read(addr);
        cpu.add_to_register_a(value);
}

fn sbc(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(&mode);
        let data = cpu.mem_read(addr);
        cpu.add_to_register_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
}

fn cmp(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(mode);
        let data = cpu.mem_read(addr);
        cpu.compare(mode, data);
}

fn cpx(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(mode);
        let data = cpu.mem_read(addr);
        cpu.compare(mode, data);
}

fn cpy(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(mode);
        let data = cpu.mem_read(addr);
        cpu.compare(mode, data);
}


fn compare(cpu: &mut CPU, mode: &AddressingMode, compare_with: u8) {
        let addr = cpu.get_operand_address(mode);
        let data = cpu.mem_read(addr);
        
        if data <= compare_with {
            cpu.status.insert(CpuFlags::CARRY);
        } else {
            cpu.status.remove(CpuFlags::CARRY);
        }

        cpu.update_zero_and_negative_flags(compare_with.wrapping_sub(data));
}