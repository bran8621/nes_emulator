/*
AND: logical AND
EOR: exclusive OR
ORA: logical inclusive OR
BIT: bit test
*/fn and(&mut self, mode: &AddressingMode) {
        self.perform_operation_and_update_flags(mode, |a, b| a & b);
    }fn eor(&mut self, mode: &AddressingMode) {
        self.perform_operation_and_update_flags(mode, |a, b| a ^ b);
    }fn ora(&mut self, mode: &AddressingMode) {
        self.perform_operation_and_update_flags(mode, |a, b| a | b);
    }fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let and = self.register_a & data;
        if and == 0 {
            self.status.insert(CpuFlags::ZERO);
        } else {
            self.status.remove(CpuFlags::ZERO);
        }

        self.status.set(CpuFlags::NEGATIVE, data & 0b10000000 > 0);
        self.status.set(CpuFlags::OVERFLOW, data & 0b01000000 > 0);
    }
