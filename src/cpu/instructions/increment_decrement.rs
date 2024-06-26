/* 
INC: increment a memory location
INX: incrememnt the X register
INY: incrememnt the Y register
DEC: decrement a memory location
DEX: decrememnt the X register
DEY: decrememnt the Y register
*/

use crate::cpu::CPU;

fn inx(cpu: &mut CPU){
        cpu.register_x = cpu.register_x.wrapping_add(1);
        cpu.update_zero_and_negative_flags(cpu.register_x);
}
fn iny(cpu: &mut CPU) {
        cpu.register_y = cpu.register_y.wrapping_add(1);
        cpu.update_zero_and_negative_flags(cpu.register_y);
}

fn inc(cpu: &mut CPU, mode: &AddressingMode) -> u8 {
        let addr = cpu.get_operand_address(mode);
        let mut data = cpu.mem_read(addr);
        data = data.wrapping_add(1);
        cpu.mem_write(addr, data);
        cpu.update_zero_and_negative_flags(data);
        data
}

fn dex(cpu: &mut CPU) {
        cpu.register_x = cpu.register_x.wrapping_sub(1);
        cpu.update_zero_and_negative_flags(cpu.register_x);
}
fn dey(cpu: &mut CPU) {
        cpu.register_y = cpu.register_y.wrapping_sub(1);
        cpu.update_zero_and_negative_flags(cpu.register_y);
}

fn dec(cpu: &mut CPU, mode: &AddressingMode) -> u8 {
        let addr = cpu.get_operand_address(mode);
        let mut data = cpu.mem_read(addr);
        data = data.wrapping_sub(1);
        cpu.mem_write(addr, data);
        cpu.update_zero_and_negative_flags(data);
        data
}