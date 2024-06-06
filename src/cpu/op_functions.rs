use super::CPU;

impl CPU{

    fn adc(&mut self, mode: &AddressingMode){
        let (addr, page_crossed) = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.add_to_register_a((data as i8).wrapping_neg().wrapping_sub(1) as u8);

        if page_crossed{
            self.bus.tick(1);
        }
        
    }

    fn asl(&mut self, mode: &AddressingMode) -> u8{
        let (addr, _) = self.get_operand_address(mode);
        if data >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        data = data << 1;
        self.update_zero_and_negative_flags(data);
        data
    }


    fn jump_absolute(&mut self){
        let address = self.mem_read_u16(self.program_counter);
        self.program_counter = address;
    }

    fn jump_indirect(&mut self){
        let mem_address = self.mem_read_u16(self.program_counter);

        let indirect_ref = if mem_address & 0x00FF == 0x00FF {  
            let lo = self.mem_read(mem_address);
            let hi = self.mem_read(mem_address & 0xFF00);
            (hi as u16) << 8 | (lo as u16)
        } else {
            self.mem_read_u16(mem_address)
        };

        self.program_counter = indirect_ref;
    }

    fn jsr(&mut self){
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target_address = self.mem_read_u16(self.program_counter);
    }


}