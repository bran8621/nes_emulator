/*
ASL: arithmetic shift left
LSR: logical shift right
ROL: rotate left
ROR: rotate right
*/


fn asl(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let mut data: u8 = self.mem_read(addr);
}

fn asl_accumulator(&mut self){
    let mut data: u8 = self.register_a;
    if data >> 7 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }
}

fn lsr(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let mut data: u8 = self.mem_read(addr);
    if data & 1 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }

    data = data >> 1;
    self.mem_write(addr, data);
    self.update_zero_and_negative_flags(data);
    data
}

fn lsr_accumulator(&mut self){
    let mut data: u8 = self.register_a;
    if data & 1 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }

    data = data >> 1;
    self.set_register_a(data);
}

fn rol(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let mut data: u8 = self.mem_read(addr);
    let carry: bool = self.status.contains(CpuFlags::CARRY);
    if data >> 7 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }
    data = data << 1;
    if carry{
        data = data | 1;
    }
    self.mem_write(addr, data);
    self.update_negative_flags(data);
    data
}

fn rol_accumulator(&mut self){
    let mut data: u8 = self.register_a;
    let carry: bool = self.status.contains(CpuFlags::CARRY);
    if data >> 7 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }
    data = data << 1;
    if carry{
        data = data | 1;
    }
    self.set_register_a(data);
}

fn ror(&mut self, mode: &AddressingMode) -> u8{
    let addr: (u16, bool) = self.get_operand_address(mode);
    let mut data: u8 = self.mem_read(addr);
    let carry: bool = self.status.contains(CpuFlags::CARRY);

    if data & 1 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }
    data = data >> 1;
    if carry{
        data = data | 0b1000_0000;
    }
    self.mem_write(addr, data);
    self.update_negative_flags(data);
    data
}

fn ror_accumulator(&mut self){
    let mut data: u8 = self.register_a;
    let carry: bool = self.status.contains(CpuFlags::CARRY);

    if data & 1 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }
    data = data >> 1;
    if carry{
        data = data | 0b1000_0000;
    }
    self.set_register_a(data);
}