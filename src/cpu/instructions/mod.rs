pub mod arithmetic;
pub mod branch;
pub mod increment_decrement;
pub mod jumps_calls;
pub mod load_store;
pub mod logical;
pub mod register_transfers;
pub mod shifts;
pub mod stack;
pub mod status_flag;
pub mod system_functions;

use crate::cpu::CPU;

pub struct InstructionSet;

impl InstructionSet{
    pub fn new() -> Self{
        InstructionSet{}
    }

    //Arithmetic instructions
    pub fn adc(&self, cpu: &mut CPU, addr: u16){
        arithmetic::adc(cpu, addr);
    }

    pub fn sbc(&self, cpu: &mut CPU, addr: u16){
        arithmetic::sbc(cpu, addr);
    }

    pub fn cmp(&self, cpu: &mut CPU, addr: u16){
        arithmetic::cmp(cpu, addr);
    }

    pub fn cpx(&self, cpu: &mut CPU, addr: u16){
        arithmetic::cpx(cpu, addr);
    }

    pub fn cpy(&self, cpu: &mut CPU, addr: u16){
        arithmetic::cpy(cpu, addr);
    }

    //increment and decrement instructions

    pub fn inx(&self, cpu: &mut CPU){
        increment_decrement::inx(cpu);
    }

    pub fn iny(&self, cpu: &mut CPU){
        increment_decrement::iny(cpu);
    }

    pub fn inc(&self, cpu: &mut CPU, addr: u16){
        increment_decrement::inc(cpu, addr);
    }

    pub fn dex(&self, cpu: &mut CPU){
        increment_decrement::dex(cpu);
    }

    pub fn dey(&self, cpu: &mut CPU){
        increment_decrement::dey(cpu);
    }

    pub fn dec(&self, cpu: &mut CPU, addr: u16){
        increment_decrement::dec(cpu, addr);
    }

    //jump and call instructions
    pub fn jump_absolute(&self, cpu: &mut CPU, addr: u16){
        jumps_calls::jump_absolute(cpu, addr);
    }

    pub fn jump_indirect(&self, cpu: &mut CPU, addr: u16){
        jumps_calls::jump_indirect(cpu, addr);
    }

    pub fn jsr(&self, cpu: &mut CPU, addr: u16){
        jumps_calls::jsr(cpu, addr);
    }

    pub fn rts(&self, cpu: &mut CPU){
        jumps_calls::rts(cpu);
    }

    //load and store instructions
    pub fn lda(&self, cpu: &mut CPU, addr: u16){
        load_store::lda(cpu, addr);
    }

    pub fn ldx(&self, cpu: &mut CPU, addr: u16){
        load_store::ldx(cpu, addr);
    }

    pub fn ldy(&self, cpu: &mut CPU, addr: u16){
        load_store::ldy(cpu, addr);
    }

    pub fn sta(&self, cpu: &mut CPU, addr: u16){
        load_store::sta(cpu, addr);
    }

    pub fn stx(&self, cpu: &mut CPU, addr: u16){
        load_store::stx(cpu, addr);
    }

    pub fn sty(&self, cpu: &mut CPU, addr: u16){
        load_store::sty(cpu, addr);
    }

    //logical instructions
    pub fn and(&self, cpu: &mut CPU, mode: &AddressingMode){
        logical::and(cpu, mode);
    }

    pub fn eor(&self, cpu: &mut CPU, mode: &AddressingMode){
        logical::eor(cpu, mode);
    }

    pub fn ora(&self, cpu: &mut CPU, mode: &AddressingMode){
        logical::ora(cpu, mode);
    }

    pub fn bit(&self, cpu: &mut CPU, mode: &AddressingMode){
        logical::bit(cpu, mode);
    }

    //register transfer instructions
    pub fn tax(&self, cpu: &mut CPU){
        register_transfers::tax(cpu);
    }

    pub fn tay(&self, cpu: &mut CPU){
        register_transfers::tay(cpu);
    }

    pub fn txa(&self, cpu: &mut CPU){
        register_transfers::txa(cpu);
    }

    pub fn tya(&self, cpu: &mut CPU){
        register_transfers::tya(cpu);
    }

    //shift instructions
    pub fn asl(&self, cpu: &mut CPU, addr: u16){
        shifts::asl(cpu, addr);
    }

    pub fn asl_accumulator(&self, cpu: &mut CPU){
        shifts::asl_accumulator(cpu);
    }

    pub fn lsr(&self, cpu: &mut CPU, addr: u16){
        shifts::lsr(cpu, addr);
    }

    pub fn lsr_accumulator(&self, cpu: &mut CPU){
        shifts::lsr_accumulator(cpu);
    }

    pub fn rol(&self, cpu: &mut CPU, addr: u16){
        shifts::rol(cpu, addr);
    }

    pub fn rol_accumulator(&self, cpu: &mut CPU){
        shifts::rol_accumulator(cpu);
    }

    pub fn ror(&self, cpu: &mut CPU, addr: u16){
        shifts::ror(cpu, addr);
    }

    pub fn ror_accumulator(&self, cpu: &mut CPU){
        shifts::ror_accumulator(cpu);
    }

    //stack instructions
    pub fn pha(&self, cpu: &mut CPU){
        stack::pha(cpu);
    }

    pub fn php(&self, cpu: &mut CPU){
        stack::php(cpu);
    }

    pub fn pla(&self, cpu: &mut CPU){
        stack::pla(cpu);
    }

    pub fn plp(&self, cpu: &mut CPU){
        stack::plp(cpu);
    }

    pub fn tsx(&self, cpu: &mut CPU){
        stack::tsx(cpu);
    }

    pub fn txs(&self, cpu: &mut CPU){
        stack::txs(cpu);
    }

    //status flag instructions

    //system function instructions
    pub fn brk(&self, cpu: &mut CPU){
        system_functions::brk(cpu);
    }

    pub fn rti(&self,cpu: &mut CPU){
        system_functions::rti(cpu);
    }
}