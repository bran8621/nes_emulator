use crate::cpu::Mem;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
// const APU_AND_IOP: u16 = 0x4000;
// const APU_AND_IOP_MIRRORS_END: u16 = 0x401F;
// const CARTRIDGE_RAM: u16 = 0x6000;
// const CARTRIDGE_RAM_MIRRORS_END: u16 = 0x7FFF;

pub struct Bus{
    cpu_vram: [u8; 2048],
}

impl Bus{
    pub fn new() -> Self{
        Bus{
            cpu_vram: [0; 2048],
        }
    }
}

impl Mem for Bus{
    fn mem_read(&self, addr: u16) -> u8{
        match addr{
            RAM..=RAM_MIRRORS_END => {
                let  mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b0000_0111_1111_1111;
                todo!("Implement the PPU")
            }
            _ => {
                println!("Attempted to read from write only address {:x}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8){
        match addr{
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!("Implement the PPU")
            }
            _ => {
                println!("Attempted to write to read only address {:x}", addr);
            }
        }
    }
}