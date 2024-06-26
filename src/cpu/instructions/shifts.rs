/*
ASL: arithmetic shift left
LSR: logical shift right
ROL: rotate left
ROR: rotate right
*/

use crate::cpu::CPU;

fn asl(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode);
    let mut data: u8 = cpu.mem_read(addr);
}

fn asl_accumulator(cpu: &mut CPU){
    let mut data: u8 = cpu.register_a;
    if data >> 7 == 1{
        cpu.set_carry_flag();
    }else{
        cpu.clear_carry_flag();
    }
}

fn lsr(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode);
    let mut data: u8 = cpu.mem_read(addr);
    if data & 1 == 1{
        cpu.set_carry_flag();
    }else{
        cpu.clear_carry_flag();
    }

    data = data >> 1;
    cpu.mem_write(addr, data);
    cpu.update_zero_and_negative_flags(data);
    data
}

fn lsr_accumulator(cpu: &mut CPU){
    let mut data: u8 = cpu.register_a;
    if data & 1 == 1{
        cpu.set_carry_flag();
    }else{
        cpu.clear_carry_flag();
    }

    data = data >> 1;
    cpu.set_register_a(data);
}

fn rol(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode);
    let mut data: u8 = cpu.mem_read(addr);
    let carry: bool = cpu.status.contains(CpuFlags::CARRY);
    if data >> 7 == 1{
        cpu.set_carry_flag();
    }else{
        cpu.clear_carry_flag();
    }
    data = data << 1;
    if carry{
        data = data | 1;
    }
    cpu.mem_write(addr, data);
    cpu.update_negative_flags(data);
    data
}

fn rol_accumulator(cpu: &mut CPU){
    let mut data: u8 = cpu.register_a;
    let carry: bool = cpu.status.contains(CpuFlags::CARRY);
    if data >> 7 == 1{
        cpu.set_carry_flag();
    }else{
        cpu.clear_carry_flag();
    }
    data = data << 1;
    if carry{
        data = data | 1;
    }
    cpu.set_register_a(data);
}

fn ror(cpu: &mut CPU, mode: &AddressingMode) -> u8{
    let addr: (u16, bool) = cpu.get_operand_address(mode);
    let mut data: u8 = cpu.mem_read(addr);
    let carry: bool = cpu.status.contains(CpuFlags::CARRY);

    if data & 1 == 1{
        cpu.set_carry_flag();
    }else{
        cpu.clear_carry_flag();
    }
    data = data >> 1;
    if carry{
        data = data | 0b1000_0000;
    }
    cpu.mem_write(addr, data);
    cpu.update_negative_flags(data);
    data
}

fn ror_accumulator(cpu: &mut CPU){
    let mut data: u8 = cpu.register_a;
    let carry: bool = cpu.status.contains(CpuFlags::CARRY);

    if data & 1 == 1{
        cpu.set_carry_flag();
    }else{
        cpu.clear_carry_flag();
    }
    data = data >> 1;
    if carry{
        data = data | 0b1000_0000;
    }
    cpu.set_register_a(data);
}