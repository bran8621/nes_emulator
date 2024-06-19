/*
BCC: branch if carry flag clear
BCS: branch if carry flag set
BEQ: branch if zero flag set
BMI: branch if negative flat set
BNE: branch if zero flag clear
BPL: branch if negative flag clear
BVC: branch if overflow flag clear
BVS: branch if overflow flag set
*/




fn branch(&mut self, condition: bool) {
    if condition {
        let jump: i8 = self.mem_read(self.program_counter) as i8;
        let jump_addr = self
            .program_counter
            .wrapping_add(1)
            .wrapping_add(jump as u16);

        self.program_counter = jump_addr;
    }
}
