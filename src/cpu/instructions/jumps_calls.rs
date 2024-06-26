/*
JMP: jump to anotherlocation
JSR: jump to a subroutine
RTS: return from a subroutine
*/

use crate::cpu::CPU;

fn jump_absolute(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode);
    cpu.program_counter = addr;
}

fn jump_indirect(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = cpu.mem_read_u16(cpu.program_counter);
    let indirect_ref:u16 = if mem_address & 0x00FF == 0x00FF {
        let lo = cpu.mem_read(addr);
        let hi = cpu.mem_read(addr & 0xFF00);
        (hi as u16) << 8 | lo as u16
    } else {
        cpu.mem_read_u16(addr)
    };
    cpu.program_counter = indirect_ref;
}

fn jsr(cpu: &mut CPU){
    cpu.stack_push_u16(cpu.program+counter+1);
    let target_address = cpu.mem_read_u16(cpu.program_counter);
    cpu.program_counter = target_address;
}

fn rts(cpu: &mut CPU){
    cpu.program_counter = cpu.stack_pop_u16() + 1;
}