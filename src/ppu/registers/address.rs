const ADDRESS_MASK: u16 = 0b0011_1111_1111_1111;
#[derive(Debug, Default)]
pub struct AddressRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        Self::default()
    }

    // Sets the address register with a 16-bit value
    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    // Updates the address register with an 8-bit value
    // Alternates between setting the high and low byte
    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        // Mirror down address above 0x3FFF
        
        if self.get() > ADDRESS_MASK {
            self.set(self.get() & ADDRESS_MASK);
        }

        self.hi_ptr = !self.hi_ptr;
    }

    // Increments the address register by the given amount
    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }

        // Mirror down address above 0x3FFF
        if self.get() > ADDRESS_MASK {
            self.set(self.get() & ADDRESS_MASK);
        }
    }

    // Resets the latch, so the next update sets the high byte
    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    // Gets the current 16-bit address
    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }
}
