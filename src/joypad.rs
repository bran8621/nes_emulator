bitflags! {
    pub struct JoypadButton: u8 {
        const A = 0b00000001;
        const B = 0b00000010;
        const SELECT = 0b00000100;
        const START = 0b00001000;
        const UP = 0b00010000;
        const DOWN = 0b00100000;
        const LEFT = 0b01000000;
        const RIGHT = 0b10000000;
    }
}

#[derive(Debug)]
pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: JoypadButton,
}

impl Joypad {
    /// Creates a new Joypad instance.
    pub fn new() -> Self {
        Joypad {
            strobe: false,
            button_index: 0,
            button_status: JoypadButton::empty(),
        }
    }

    /// Writes data to the Joypad.
    pub fn write(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.button_index = 0;
        }
    }

    /// Reads the current button status from the Joypad.
    pub fn read(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }
        let response = (self.button_status.bits() & (1 << self.button_index)) >> self.button_index;
        if !self.strobe && self.button_index <= 7 {
            self.button_index += 1;
        }
        response
    }

    /// Sets the status of a specific button.
    pub fn set_button_status(&mut self, button: JoypadButton, pressed: bool) {
        if pressed {
            self.button_status.insert(button);
        } else {
            self.button_status.remove(button);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_joypad() {
        let mut joypad = Joypad::new();
        joypad.set_button_status(JoypadButton::A, true);
        joypad.set_button_status(JoypadButton::B, true);
        joypad.set_button_status(JoypadButton::SELECT, true);
        joypad.set_button_status(JoypadButton::START, true);
        joypad.set_button_status(JoypadButton::UP, true);
        joypad.set_button_status(JoypadButton::DOWN, true);
        joypad.set_button_status(JoypadButton::LEFT, true);
        joypad.set_button_status(JoypadButton::RIGHT, true);

        joypad.write(0);
        assert_eq!(joypad.read(), 0);

        joypad.write(1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 0);
    }

    #[test]
    fn test_strobe_mode() {
        let mut joypad = Joypad::new();
        joypad.write(1);
        joypad.set_button_status(JoypadButton::A, true);
        for _x in 0..10 {
            assert_eq!(joypad.read(), 1);
        }
    }

    #[test]
    fn test_button_status() {
        let mut joypad = Joypad::new();
        joypad.set_button_status(JoypadButton::A, true);
        assert_eq!(joypad.read(), 0);
        joypad.write(1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 0);
    }

    #[test]
    fn test_strobe_mode_on_off() {
        let mut joypad = Joypad::new();
        joypad.write(0);
        joypad.set_button_status(JoypadButton::RIGHT, true);
        joypad.set_button_status(JoypadButton::LEFT, true);
        joypad.set_button_status(JoypadButton::DOWN, true);
        joypad.set_button_status(JoypadButton::UP, true);

        for _ in 0..=10 {
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 1);

            for _x in 0..10 {
                assert_eq!(joypad.read(), 1);
            }
            joypad.write(1);
            joypad.write(0);
        }
    }
}
