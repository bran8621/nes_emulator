
use super::CPU;
use super::AddressingMode;
use super::CpuFlags;
use super::Mem;



impl <'a>CPU<'a> {
/*
TAX: transfer accumulator to X
TAY: transfer accumulator to Y
TXA: transfer X to accumulator 
TYA: transfer Y to accumulator
*/

pub fn tax(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flags(self.register_x);
}
pub fn tay(&mut self) {
    self.register_y = self.register_a;
    self.update_zero_and_negative_flags(self.register_y);
}
pub fn txa(&mut self) {
    self.register_a = self.register_x;
    self.update_zero_and_negative_flags(self.register_a);
}
pub fn tya(&mut self) {
    self.register_a = self.register_y;
    self.update_zero_and_negative_flags(self.register_a);
}
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


pub fn branch(&mut self, condition: bool) {
    if condition {
        let jump: i8 = self.mem_read(self.program_counter) as i8;
        let jump_addr = self
            .program_counter
            .wrapping_add(1)
            .wrapping_add(jump as u16);
        self.program_counter = jump_addr;
    }
}
/*
ADC: add with carry
SBC: subtract with carry
CMP: compare accumulator
CPX: compare X register
CPY: compare Y register 
*/

pub fn adc(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(value);
}
pub fn sbc(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(&mode);
        let data = self.mem_read(addr);
        self.add_to_register_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
}
pub fn cmp(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.compare(mode, data);
}
pub fn cpx(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.compare(mode, data);
}
pub fn cpy(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.compare(mode, data);
}

pub fn compare(&mut self, mode: &AddressingMode, compare_with: u8) {
        let (addr, _) = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        
        if data <= compare_with {
            self.status.insert(CpuFlags::CARRY);
        } else {
            self.status.remove(CpuFlags::CARRY);
        }
        self.update_zero_and_negative_flags(compare_with.wrapping_sub(data));
}
/*
ASL: arithmetic shift left
LSR: logical shift right
ROL: rotate left
ROR: rotate right
*/


pub fn asl(&mut self, mode: &AddressingMode) -> u8 {
    let (addr, _) = self.get_operand_address(mode);
    let mut data = self.mem_read(addr);
    if data >> 7 == 1 {
        self.set_carry_flag();
    } else {
        self.clear_carry_flag();
    }
    data = data << 1;
    self.mem_write(addr, data);
    self.update_zero_and_negative_flags(data);
    data
}

pub fn asl_accumulator(&mut self){
    let data: u8 = self.register_a;
    if data >> 7 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }
}

pub fn lsr(&mut self, mode: &AddressingMode) -> u8 {
    let (addr, _) = self.get_operand_address(mode);
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

pub fn lsr_accumulator(&mut self){
    let mut data: u8 = self.register_a;
    if data & 1 == 1{
        self.set_carry_flag();
    }else{
        self.clear_carry_flag();
    }
    data = data >> 1;
    self.set_register_a(data);
}

pub fn rol(&mut self, mode: &AddressingMode) -> u8 {
    let (addr, _) = self.get_operand_address(mode);
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

pub fn rol_accumulator(&mut self){
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

pub fn ror(&mut self, mode: &AddressingMode) -> u8{
    let (addr, _): (u16, bool) = self.get_operand_address(mode);
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

pub fn ror_accumulator(&mut self){
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

/*
JMP: jump to anotherlocation
JSR: jump to a subroutine
RTS: return from a subroutine
*/

pub fn jump_absolute(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    self.program_counter = addr;
}

pub fn jump_indirect(&mut self) {
    let mem_address = self.mem_read_u16(self.program_counter);
    let indirect_ref = if mem_address & 0x00FF == 0x00FF {
        let lo = self.mem_read(mem_address);
        let hi = self.mem_read(mem_address & 0xFF00);
        (hi as u16) << 8 | lo as u16
    } else {
        self.mem_read_u16(mem_address)
    };
    self.program_counter = indirect_ref; 
}

pub fn jsr(&mut self){
    self.stack_push_u16(self.program_counter+1);
    let target_address = self.mem_read_u16(self.program_counter);
    self.program_counter = target_address;
}

pub fn rts(&mut self){
    self.program_counter = self.stack_pop_u16() + 1;
}

/*
LDA: load accumulator
LDX: load X register
LDY: load Y register
STA: store accumulator
STX: store X register
STY: store Y register
*/

pub fn lda(&mut self, mode: &AddressingMode) {
    let (addr, page_cross) = self.get_operand_address(mode);
    let value = self.mem_read(addr);
    self.set_register_a(value);
    if page_cross {
        self.bus.tick(1);
    }
}

pub fn ldx(&mut self, mode: &AddressingMode) {
    let (addr, page_cross) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.register_x = data;
    self.update_zero_and_negative_flags(data);
    if page_cross{
        self.bus.tick(1);
    }

}

pub fn ldy(&mut self, mode: &AddressingMode) {
    let (addr, page_cross) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.register_y = data;
    self.update_zero_and_negative_flags(data);
    if page_cross{
        self.bus.tick(1);
    }
}

pub fn sta(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
}

pub fn stx(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
}

pub fn sty(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
}

/*
TSX: transfer stack pointer to X
TXS: transfer X to stack pointer
PHA: push accumulator on stack
PHP: push processor status on stack
PLA: pull accumulator from stack
PLP: pull process status from stack
*/


pub fn php(&mut self) {
        let mut flags = self.status.clone();
        flags.insert(CpuFlags::BREAK);
        flags.insert(CpuFlags::BREAK2);
        self.stack_push(flags.bits());
}

pub fn pla(&mut self) {
        let data = self.stack_pop();
        self.set_register_a(data);
}

pub fn plp(&mut self) {
        let flags = self.stack_pop();
        self.status = CpuFlags::from_bits_truncate(flags);
        self.status.remove(CpuFlags::BREAK);
        self.status.insert(CpuFlags::BREAK2);
}

pub fn pha(&mut self) {
        self.stack_push(self.register_a);
}

pub fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
}

pub fn txs(&mut self) {
        self.stack_pointer = self.register_x;
}

/* 
INC: increment a memory location
INX: incrememnt the X register
INY: incrememnt the Y register
DEC: decrement a memory location
DEX: decrememnt the X register
DEY: decrememnt the Y register
*/

pub fn inx(&mut self){
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
}

pub fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
}

pub fn inc(&mut self, mode: &AddressingMode) -> u8 {
        let (addr, _) = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        data = data.wrapping_add(1);
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
        data
}

pub fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
}

pub fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
}

pub fn dec(&mut self, mode: &AddressingMode) -> u8 {
        let (addr, _) = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        data = data.wrapping_sub(1);
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
        data
}

/*
AND: logical AND
EOR: exclusive OR
ORA: logical inclusive OR
BIT: bit test
*/

pub fn and(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.register_a &= data;
    self.update_zero_and_negative_flags(self.register_a);
}

pub fn eor(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.register_a ^= data;
    self.update_zero_and_negative_flags(self.register_a);
}

pub fn ora(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.register_a |= data;
    self.update_zero_and_negative_flags(self.register_a);
}

pub fn bit(&mut self, mode: &AddressingMode) {
    let (addr, _) =self.get_operand_address(mode);
    let data =self.mem_read(addr);
    let and =self.register_a & data;
    if and == 0 {
       self.status.insert(CpuFlags::ZERO);
    } else {
       self.status.remove(CpuFlags::ZERO);
    }
   self.status.set(CpuFlags::NEGATIVE, data & 0b10000000 > 0);
   self.status.set(CpuFlags::OVERFLOW, data & 0b01000000 > 0);
}

/*
Stack operations
*/

pub fn stack_push(&mut self, data: u8) {
    self.mem_write(0x100 + self.stack_pointer as u16, data);
    self.stack_pointer = self.stack_pointer.wrapping_sub(1);
}

pub fn stack_push_u16(&mut self, data: u16) {
    self.stack_push(((data >> 8) & 0xFF) as u8);
    self.stack_push((data & 0xFF) as u8);
}

pub fn stack_pop(&mut self) -> u8 {
    self.stack_pointer = self.stack_pointer.wrapping_add(1);
    self.mem_read(0x100 + self.stack_pointer as u16)
}

pub fn stack_pop_u16(&mut self) -> u16 {
    let lo = self.stack_pop() as u16;
    let hi = self.stack_pop() as u16;
    (hi << 8) | lo
}



/*
CLC: clear carry flag
CLD: clear decimal mode flag
CLI: clear interrupt disable flag
CLV: clear overflow flag
SEC: set carry flag
SED: set decimal mode flag
SEI: set interrupt disable flag
*/


/*
BRK: force an interrupt
NOP: no operation
RTI: return from interrupt
*/

pub fn brk(&mut self){
    self.program_counter += 1;
    self.status.insert(CpuFlags::BREAK);
    self.stack_push_u16(self.program_counter);
    self.stack_push(self.status.bits());
    self.status.insert(CpuFlags::INTERRUPT_DISABLE);
    self.program_counter = self.mem_read_u16(0xFFFE);
}
pub fn rti(&mut self){
    let flags_bits = self.stack_pop();
    let flags = CpuFlags::from_bits_truncate(flags_bits);
    self.status = flags;
    self.status.remove(CpuFlags::BREAK);
    self.status.insert(CpuFlags::BREAK2);
    self.program_counter = self.stack_pop_u16();
}



pub fn dcp(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let mut data = self.mem_read(addr);
    data = data.wrapping_sub(1);
    self.mem_write(addr, data);
    // self._update_zero_and_negative_flags(data);
    if data <= self.register_a {
        self.status.insert(CpuFlags::CARRY);
    }

    self.update_zero_and_negative_flags(self.register_a.wrapping_sub(data));
}

pub fn rla(&mut self, mode: &AddressingMode) {
    let data = self.rol(mode);
    self.and_with_register_a(data);
}

pub fn slo(&mut self, mode: &AddressingMode) {
    let data = self.asl(mode);
    self.or_with_register_a(data);
}

pub fn sre(&mut self, mode: &AddressingMode) {
    let data = self.lsr(mode);
    self.xor_with_register_a(data);
}

pub fn skb(&mut self) {
    self.program_counter += 2;
}



pub fn axs(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    let x_and_a = self.register_x & self.register_a;
    let result = x_and_a.wrapping_sub(data);

    if data <= x_and_a {
        self.status.insert(CpuFlags::CARRY);
    }
    self.update_zero_and_negative_flags(result);

    self.register_x = result;
}

pub fn arr(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.and_with_register_a(data);
    self.ror_accumulator();
    
    let result = self.register_a;
    let bit_5 = (result >> 5) & 1;
    let bit_6 = (result >> 6) & 1;

    if bit_6 == 1 {
        self.status.insert(CpuFlags::CARRY)
    } else {
        self.status.remove(CpuFlags::CARRY)
    }

    if bit_5 ^ bit_6 == 1 {
        self.status.insert(CpuFlags::OVERFLOW);
    } else {
        self.status.remove(CpuFlags::OVERFLOW);
    }

    self.update_zero_and_negative_flags(result);
}

pub fn unofficial_sbc(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.sub_from_register_a(data);
}

pub fn anc(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.and_with_register_a(data);
    if self.status.contains(CpuFlags::NEGATIVE) {
        self.status.insert(CpuFlags::CARRY);
    } else {
        self.status.remove(CpuFlags::CARRY);
    }
}

pub fn alr(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.and_with_register_a(data);
    self.lsr_accumulator();
}

pub fn nop_read(&mut self, mode: &AddressingMode) {
    let (_, page_cross) = self.get_operand_address(mode);
    if page_cross{
        self.bus.tick(1);
    }

}

pub fn rra(&mut self, mode: &AddressingMode) {
    let data = self.ror(mode);
    self.add_to_register_a(data);
}

pub fn isb(&mut self, mode: &AddressingMode) {
    let data = self.inc(mode);
    self.sub_from_register_a(data);
}

pub fn nop(&mut self) {
    /* do nothing */
}

pub fn lax(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.set_register_a(data);
    self.register_x = self.register_a;
}

pub fn sax(&mut self, mode: &AddressingMode) {
    let data = self.register_a & self.register_x;
    let (addr, _) = self.get_operand_address(mode);
    self.mem_write(addr, data);
}

pub fn lxa(&mut self, mode: &AddressingMode) {
    self.lda(mode);
    self.tax();
}

pub fn xaa(&mut self, mode: &AddressingMode) {
    self.register_a = self.register_x;
    self.update_zero_and_negative_flags(self.register_a);
    let (addr, _) = self.get_operand_address(mode);
    let data = self.mem_read(addr);
    self.and_with_register_a(data);
}

pub fn las(&mut self, mode: &AddressingMode) {
    let (addr, _) = self.get_operand_address(mode);
    let mut data = self.mem_read(addr);
    data = data & self.stack_pointer;
    self.register_a = data;
    self.register_x = data;
    self.stack_pointer = data;
    self.update_zero_and_negative_flags(data);
}

pub fn tas(&mut self) {
    let data = self.register_a & self.register_x;
    self.stack_pointer = data;
    let mem_address =
        self.mem_read_u16(self.program_counter) + self.register_y as u16;

    let data = ((mem_address >> 8) as u8 + 1) & self.stack_pointer;
    self.mem_write(mem_address, data)
}

pub fn ahx_indirect_y(&mut self) {
    let pos: u8 = self.mem_read(self.program_counter);
    let mem_address = self.mem_read_u16(pos as u16) + self.register_y as u16;
    let data = self.register_a & self.register_x & (mem_address >> 8) as u8;
    self.mem_write(mem_address, data)
}

pub fn ahx_absolute_y(&mut self) {
    let mem_address =
        self.mem_read_u16(self.program_counter) + self.register_y as u16;

    let data = self.register_a & self.register_x & (mem_address >> 8) as u8;
    self.mem_write(mem_address, data)
}

pub fn shx(&mut self) {
    let base_address = self.mem_read_u16(self.program_counter);
    let mut mem_address = base_address.wrapping_add(self.register_y as u16);

    // If page crossing is detected
    if (base_address & 0xFF00) != (mem_address & 0xFF00) {
        // Handle page crossing by ANDing the address with the high byte of X shifted
        mem_address &= (self.register_x as u16) << 8;
    }

    let high_byte_plus_one = ((mem_address >> 8) as u8).wrapping_add(1);
    let data = self.register_x & high_byte_plus_one;

    self.mem_write(mem_address, data);
}

pub fn shy(&mut self) {
    let mem_address =
        self.mem_read_u16(self.program_counter) + self.register_x as u16;
    let data = self.register_y & ((mem_address >> 8) as u8 + 1);
    self.mem_write(mem_address, data)
}

pub fn default(&mut self) {
    todo!();
}

//CPU helpers

pub fn set_carry_flag(&mut self) {
    self.status.insert(CpuFlags::CARRY);
}

pub fn clear_carry_flag(&mut self) {
    self.status.remove(CpuFlags::CARRY);
}

pub fn set_register_a(&mut self, data: u8) {
    self.register_a = data;
}

pub fn set_register_x(&mut self, data: u8) {
    self.register_x = data;
}

pub fn set_register_y(&mut self, data: u8) {
    self.register_y = data;
}

pub fn add_to_register_a(&mut self, data: u8) {
    let result = self.register_a as u16 + data as u16;
    self.status.set(CpuFlags::CARRY, result > 0xFF);
    self.status.set(CpuFlags::OVERFLOW, (self.register_a ^ data) & (data ^ result as u8) & 0x80 > 0);
    self.register_a = result as u8;
    self.update_zero_and_negative_flags(self.register_a);
}

pub fn sub_from_register_a(&mut self, data: u8) {
    let borrow = if self.status.contains(CpuFlags::CARRY) { 0 } else { 1 };
    let result = self.register_a as i16 - data as i16 - borrow as i16;
    self.status.set(CpuFlags::CARRY, result >= 0);
    self.status.set(CpuFlags::NEGATIVE, result < 0);
    self.status.set(CpuFlags::ZERO, result as u8 == 0);
    self.status.set(CpuFlags::OVERFLOW, (self.register_a ^ data) & (self.register_a ^ result as u8) & 0x80 > 0);
    self.register_a = result as u8;
}

pub fn and_with_register_a(&mut self, data: u8) {
    self.register_a &= data;
    self.update_zero_and_negative_flags(self.register_a);
}

pub fn xor_with_register_a(&mut self, data: u8) {
    self.register_a ^= data;
    self.update_zero_and_negative_flags(self.register_a);
}

pub fn or_with_register_a(&mut self, data: u8) {
    self.register_a |= data;
    self.update_zero_and_negative_flags(self.register_a);
}

pub fn update_zero_and_negative_flags(&mut self, data: u8) {
    self.status.set(CpuFlags::ZERO, data == 0);
    self.status.set(CpuFlags::NEGATIVE, data & 0x80 > 0);
}

pub fn update_negative_flags(&mut self, data: u8) {
    self.status.set(CpuFlags::NEGATIVE, data & 0x80 > 0);
}

}