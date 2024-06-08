/*
LDA: load accumulator
LDX: load X register
LDY: load Y register
STA: store accumulator
STX: store X register
STY: store Y register
*/

fn lda(&mut self, mode: &AddressingMode) {
        let mut data: u8 = 0;
        self.load_data_and_update_flags(mode, &mut data);
        self.set_register_a(data);
    }fn ldx(&mut self, mode: &AddressingMode) {
        let mut data: u8 = 0;
        self.load_data_and_update_flags(mode, &mut data);
        self.register_x = data;
    }fn ldy(&mut self, mode: &AddressingMode) {
        let mut data: u8 = 0;
        self.load_data_and_update_flags(mode, &mut data);
        self.register_y = data;
    }fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }
