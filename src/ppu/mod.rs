use crate::cartridge::Mirroring;
use crate::ppu::registers::*;

pub mod registers;

pub struct NesPPU{
    pub chr_ram: Vec<u8>,
    pub mirroring: Mirroring,
    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    pub scroll: ScrollRegister,
    pub address: AddressRegister,
    pub video_ram : [u8; 2048],

    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    pub paletter_table: [u8; 32],

    internal_data_buffer: u8,
}

pub trait PPU{
    fn write_to_ctrl(&mut self, value: u8);
    fn write_to_mask(&mut self, value: u8);
    fn write_to_status(&mut self, value: u8);
    fn write_to_oam_addr(&mut self, value: u8);
    fn write_to_oam_data(&mut self, value: u8);
    fn write_to_ppu_addr(&mut self, value: u8);
    fn write_oam_dma(&mut self, value: &[u8; 256]);
    fn write_to_data(&mut self, value: u8);

    fn read_status(&mut self) -> u8;
    fn read_oam_data(&mut self) -> u8;
    fn read_data(&mut self) -> u8;
}

impl NesPPU{
    pub fn new_empty_rom() -> Self {
        NesPPU(vec![0; 2048], Mirroring::Vertical)
    }

    pub fn new(chr_rom:Vec<u8>, mirroring: Mirroring) -> Self {
        NesPPU{
            chr_ram: chr_rom,
            mirroring: mirroring,
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            scroll: ScrollRegister::new(),
            address: AddressRegister::new(),
            internal_data_buffer: 0,
            oam_addr: 0,
            oam_data: [0; 256],
            paletter_table: [0; 32],
            video_ram: [0; 2048],
        }
    }

    pub fn mirror_vram_address(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b1011_1111_1111_1111;
        let vram_index = mirrored_vram - 0x2000;

        let name_table = vram_index/0x400;
        match (self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) | (Mirroring::Horizontal, 3) => vram_index - 0x800,
            (Mirroring::Horizantal,2) | (Mirroring::Horizontal, 1) => vram_index - 0x400,
            _ => vram_index
        }
    }
    fn incrememnt_vram_address(&mut self) {
        self.address.increment(self.ctrl.vram_addr_inc());
    }
}



