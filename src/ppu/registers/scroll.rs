pub struct ScrollRegister {
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub latch: bool,
}

impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            scroll_x: 0,
            scroll_y: 0,
            latch: false,
        }
    }

    pub fn write(&mut self, data: u8) {
        if self.latch {
            self.scroll_y = data;
        } else {
            self.scroll_x = data;
        }
        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }

    pub fn get_scroll_x(&self) -> u8 {
        self.scroll_x
    }

    pub fn get_scroll_y(&self) -> u8 {
        self.scroll_y
    }
}
