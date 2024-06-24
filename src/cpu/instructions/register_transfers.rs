/*
TAX: transfer accumulator to X
TAY: transfer accumulator to Y
TXA: transfer X to accumulator 
TYA: transfer Y to accumulator
*/

fn tax(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flags(self.register_x);
}

fn tay(&mut self) {
    self.register_y = self.register_a;
    self.update_zero_and_negative_flags(self.register_y);
}

fn txa(&mut self) {
    self.register_a = self.register_x;
    self.update_zero_and_negative_flags(self.register_a);
}

fn tya(&mut self) {
    self.register_a = self.register_y;
    self.update_zero_and_negative_flags(self.register_a);
}

