
use crate::cartridge::Rom;
use crate::ppu::NesPPU;
use crate::ppu::PPU;

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
    rom: Rom,
    ppu: NesPPU,
}


impl Bus{
    pub fn new(rom: Rom) -> Self{
        let ppu = NesPPU::new(rom.chr_rom, rom.mirroring);

        Bus{
            cpu_vram: [0; 2048],
            rom: rom.prg_rom,
            ppu,
        }
    }
    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}


impl Mem for Bus{
    fn mem_read(&self, addr: u16) -> u8{
        match addr{
            RAM..=RAM_MIRRORS_END => {
                let  mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => 
            {
                panic!("Attempt to read from write-only PPU address {:x}", addr)
            }
            
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_read(mirror_down_addr)
            }

            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("Attempted to read from write only address {:x}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8){
        match addr{
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            0x2000 => self.ppu.write_to_ctrl(data),
            0x2001 => self.ppu.write_to_mask(data),
            0x2003 => self.ppu.write_to_oam_addr(data),
            0x2004 => self.ppu.write_to_oam_data(data),
            0x2005 => self.ppu.write_to_scroll(data),
            0x2006 => self.ppu.write_to_ppu_addr(data),
            0x2007 => self.ppu.write_to_data(data),
            0x4014 => self.ppu.write_oam_dma(data),
            0x2002 => panic!("Attempt to write to PPU status register {:x}", addr),
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_write(mirror_down_addr, data);
            }
            _ => {
                println!("Attempted to write to read only address {:x}", addr);
            }
        }
    }
}
