/*
TSX: transfer stack pointer to X
TXS: transfer X to stack pointer
PHA: push accumulator on stack
PHP: push processor status on stack
PLA: pull accumulator from stack
PLP: pull process status from stack
*/

fn php(&mut self) {
        let mut flags = self.status.clone();
        flags.insert(CpuFlags::BREAK);
        flags.insert(CpuFlags::BREAK2);
        self.stack_push(flags.bits());
}

fn pla(&mut self) {
        let data = self.stack_pop();
        self.set_register_a(data);
}

fn plp(&mut self) {
        let flags = self.stack_pop();
        self.status = CpuFlags::from_bits_truncate(flags);
        self.status.remove(CpuFlags::BREAK);
        self.status.insert(CpuFlags::BREAK2);
}
