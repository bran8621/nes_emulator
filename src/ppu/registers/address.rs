pub struct AddressRegister{
    value: (u8, u8),
    hi_ptr: bool,
}

impl AddressRegister{
    pub fn new() -> Self{
        Self{
            value: (0, 0),
            hi_ptr: false,
        }
    }
    fn set(&mut self, data: u16){
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }
    
}