bitflags! {
    /*
    VSO 
    V: Vertical blank has started (0: not in VBLANK; 1: in VBLANK)
    S: Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps a nonzero background pixel.  Cleared at dot 1 (V-Blank NMI).
    O" Sprite overflow.  Set when sprite 0 hits sprite 1.  Cleared at dot 1 (V-Blank NMI).
    */

    pub struct StatusRegister: u8 {
        const NOTUSED = 0b0000_0001;
        const NOTUSED2 = 0b0000_0010;
        const NOTUSED3 = 0b0000_0100;
        const NOTUSED4 = 0b0000_1000;
        const NOTUSED5 = 0b0001_0000;
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_ZERO_HIT = 0b0100_0000;
        const VERTICAL_BLANK = 0b1000_0000;
    }
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister::from_bits_truncate(0b0000_0000)
    }

    pub fn set_vblank_status(&mut self, value: bool) {
        self.set(StatusRegister::VERTICAL_BLANK, value);
    }

    pub fn set_sprite_zero_hit(&mut self, value: bool) {
        self.set(StatusRegister::SPRITE_ZERO_HIT, value);
    }

    pub fn set_sprite_overflow(&mut self, value: bool) {
        self.set(StatusRegister::SPRITE_OVERFLOW, value);
    }

    pub fn is_in_vertical_blank(&self) -> bool {
        self.contains(StatusRegister::VERTICAL_BLANK)
    }

    pub fn is_sprite_zero_hit(&self) -> bool {
        self.contains(StatusRegister::SPRITE_ZERO_HIT)
    }

    pub fn is_sprite_overflow(&self) -> bool {
        self.contains(StatusRegister::SPRITE_OVERFLOW)
    }

    fn set(&mut self, flag: StatusRegister, value: bool) {
        if value {
            self.insert(flag);
        } else {
            self.remove(flag);
        }
    }

    fn reset(&mut self, flag: StatusRegister) {
        self.remove(flag);
    }

    pub fn snapshot(&self) -> u8 {
        self.bits
    }
}