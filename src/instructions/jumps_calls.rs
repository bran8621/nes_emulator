/*
JMP: jump to anotherlocation
JSR: jump to a subroutine
RTS: return from a subroutine
*/

fn jump_absolute(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    self.program_counter = addr;
}

fn jump_indirect(&mut self, mode: &AddressingMode) {
    let addr = self.mem_read_u16(self.program_counter);
    let indirect_ref:u16 = if mem_address & 0x00FF == 0x00FF {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr & 0xFF00);
        (hi as u16) << 8 | lo as u16
    } else {
        self.mem_read_u16(addr)
    };
    self.program_counter = indirect_ref;
}

fn jsr(&mut self){
    self.stack_push_u16(self.program+counter+1);
    let target_address = self.mem_read_u16(self.program_counter);
    self.program_counter = target_address;
}

fn rts(&mut self){
    self.program_counter = self.stack_pop_u16() + 1;
}