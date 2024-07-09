pub struct AddressRegister {
    value: u16,
    hi_ptr: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        Self {
            value: 0,
            hi_ptr: false,
        }
    }

    pub fn set(&mut self, data: u16) {
        self.value = data & 0x3fff;
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value = (self.value & 0x00ff) | ((data as u16) << 8);
        } else {
            self.value = (self.value & 0xff00) | (data as u16);
        }

        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        self.value = self.value.wrapping_add(inc as u16) & 0x3fff;
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}
