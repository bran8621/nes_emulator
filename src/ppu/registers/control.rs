bitflags! {
    /*VPHB SINN
    V: VBlank interrupt enable (1: enabled) (0: disabled)
    P: PPU master/slave select (1: read backdrop from EXT pins; 0: output color on EXT pins)
    H: sprite height (1: 8 dots; 0: 16 dots)
    B: background pattern table address (1: $0000; 0: $1000)
    S: sprite pattern table address for 8x8 sprites (1: $0000; 0: $1000)
    I: VRAM address increment per CPU read/write of PPUDATA (1: increment by 32; 0: increment by 1)
    NN: Base nametable address (0: $2000; 1: $2400; 2: $2800; 3: $2C00)
    */
    pub struct ControlRegister: u8 {
        const NAMETABLE1 = 0b0000_0001;
        const NAMETABLE2 = 0b0000_0010;
        const VRAM_ADDR_INCREMENT = 0b0000_0100;
        const SPRITE_PATTERN_ADDR = 0b0000_1000;
        const BACKGROUND_PATTERN_ADDR = 0b0001_0000;
        const SPRITE_HEIGHT = 0b0010_0000;
        const MASTER_SLAVE_SELECT = 0b0100_0000;
        const GENERATE_NMI = 0b1000_0000;
    }
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0b0000_0000)
    }
    pub fn nametable_address(&self) -> u16 {
        match self.bits & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => 0
        }
    }

    pub fn vram_addr_inc(&self) -> u8 {
        if self.contains(ControlRegister::VRAM_ADDR_INCREMENT) {
            32
        } else {
            1
        }
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            0x1000
        } else {
            0
        }
    }

    pub fn background_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::BACKGROUND_PATTERN_ADDR) {
            0x1000
        } else {
            0
        }
    }

    pub fn sprite_height(&self) -> u8 {
        if self.contains(ControlRegister::SPRITE_HEIGHT) {
            16
        } else {
            8
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        if self.contains(ControlRegister::MASTER_SLAVE_SELECT) {
            1
        } else {
            0
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        self.contains(ControlRegister::GENERATE_NMI)
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }

}