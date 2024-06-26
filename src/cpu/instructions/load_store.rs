/*
LDA: load accumulator
LDX: load X register
LDY: load Y register
STA: store accumulator
STX: store X register
STY: store Y register
*/

use crate::cpu::CPU;

fn lda(cpu: &mut CPU, mode: &AddressingMode) {
        let mut data: u8 = 0;
        cpu.load_data_and_update_flags(mode, &mut data);
        cpu.set_register_a(data);
}

fn ldx(cpu: &mut CPU, mode: &AddressingMode) {
        let mut data: u8 = 0;
        cpu.load_data_and_update_flags(mode, &mut data);
        cpu.register_x = data;
}

fn ldy(cpu: &mut CPU, mode: &AddressingMode) {
        let mut data: u8 = 0;
        cpu.load_data_and_update_flags(mode, &mut data);
        cpu.register_y = data;
}

fn sta(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(mode);
        cpu.mem_write(addr, cpu.register_a);
}

fn stx(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(mode);
        cpu.mem_write(addr, cpu.register_x);
}

fn sty(cpu: &mut CPU, mode: &AddressingMode) {
        let addr = cpu.get_operand_address(mode);
        cpu.mem_write(addr, cpu.register_y);
}