/*
ADC: add with carry
SBC: subtract with carry
COM: compare accumulator
CPX: compare X register
CPY: compare Y register 
*/

use crate::cpu::cpu;

pub fn adc(cpu: &mut cpu){
    
}

pub fn sbc(cpu: &mut cpu){

}

pub fn com(cpu: &mut cpu){

}

pub fn cpx(cpu: &mut cpu){

}fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(value);
}

fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let data = self.mem_read(addr);
        self.add_to_register_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
}
