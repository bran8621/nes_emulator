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
    pub virtual_ram : [u8; 2048],

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