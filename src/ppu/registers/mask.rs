bitflags! {
    /* BGRs bMmG 
    B: emphasize blue
    G: emphasize green
    R: emphasize red
    s: show sprites
    b: show background
    M: 1: show sprites in leftmost 8 pixels of screen 0: hide
    m: 1: show background in leftmost 8 pixels of screen 0: hide
    G: greyscale (0: normal color, 1: produce a greyscale display)
    */

    pub struct MaskRegister: u8{
        const GREYSCALE = 0b0000_0001;
        const LEFTMOST_8PXL_BACKGROUND = 0b0000_0010;
        const LEFTMOST_8PXL_SPRITES = 0b0000_0100;
        const SHOW_BACKGROUND = 0b0000_1000;
        const SHOW_SPRITES = 0b0001_0000;
        const EMPHASIZE_RED = 0b0010_0000;
        const EMPHASIZE_GREEN = 0b0100_0000;
        const EMPHASIZE_BLUE = 0b1000_0000;
    }
}

#[derive(Debug)]
pub enum Color{
    Red, 
    Green,
    Blue
}

impl MaskRegister{
    //Creates a new MaskRegister with all flags cleared
    pub fn new() -> self::MaskRegister{
        MaskRegister::from_bits_truncate(0b0000_0000)
    }

    fn is_flag_set(&self, flag: MaskRegister) -> bool{
        self.contains(flag)
    }

    //Checks if the greyscale flag is set
    pub fn is_greyscale(&self) -> bool{
        self.is_flag_set(MaskRegister::GREYSCALE)
    }

    //Checks if the leftmost 8pxl background flag is set
    pub fn leftmost_8pxl_background(&self) -> bool{
        self.is_flag_set(MaskRegister::LEFTMOST_8PXL_BACKGROUND)
    }

    //Checks if the leftmost 8pxl sprite flag is set
    pub fn  leftmost_8pxl_sprite(&self) -> bool{
        self.is_flag_set(MaskRegister::LEFTMOST_8PXL_SPRITES   )
    }

    //Checks if the show background flag is set
    pub fn show_background(&self) -> bool{
        self.is_flag_set(MaskRegister::SHOW_BACKGROUND)
    }

    //Checks if the show sprites flag is set
    pub fn show_sprites(&self) -> bool{
        self.is_flag_set(MaskRegister::SHOW_SPRITES)
    }

    //Checks if the emphasize flags are set and returns a vector of the colors to emphasize
    pub fn emphasized_colors(&self) -> Vec<Color> {
        [MaskRegister::EMPHASIZE_RED, MaskRegister::EMPHASIZE_GREEN, MaskRegister::EMPHASIZE_BLUE]
            .iter()
            .filter_map(|&flag| {
                if self.is_flag_set(flag) {
                    Some(match flag {
                        MaskRegister::EMPHASIZE_RED => Color::Red,
                        MaskRegister::EMPHASIZE_GREEN => Color::Green,
                        MaskRegister::EMPHASIZE_BLUE => Color::Blue,
                        _ => unreachable!(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn update(&mut self, data: u8){
        self.bits = data;
    }
}

impl Default for MaskRegister{
    fn default() -> Self {
        MaskRegister::from_bits_truncate(0b0000_0000)  
    }
}