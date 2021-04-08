use minifb::Key;

#[derive(Debug, Clone, Copy)]
pub enum Keyboard8 {
    Key1 = 0x0,
    Key2 = 0x1,
    Key3 = 0x2,
    Key4 = 0x3,
    Key5 = 0x4,
    Key6 = 0x5,
    Key7 = 0x6,
    Key8 = 0x7,
    Key9 = 0x8,
    Key10 = 0x9,
    Key11 = 0xA,
    Key12 = 0xB,
    Key13 = 0xC,
    Key14 = 0xD,
    Key15 = 0xE,
    Key16 = 0xF,
}

impl Keyboard8 {
    // Converts a Key Input to a from the Keyboard the the specific Chip8-Key
    // Layout:
    //       1 2 3 4            1 2 3 C 
    //       Q W E R     ->     4 5 6 D
    //       A S D F            7 8 9 E
    //       Y X C V            A 0 B F
    // parameter: key as Key from minifb
    // return: Option<Keyboard8>
    pub fn from_key(key: Key) -> Option<Keyboard8> {
        match key {
            Key::Key1 => Some(Keyboard8::Key2),
            Key::Key2 => Some(Keyboard8::Key3),
            Key::Key3 => Some(Keyboard8::Key4),
            Key::Key4 => Some(Keyboard8::Key13),
            Key::Q => Some(Keyboard8::Key5),
            Key::W => Some(Keyboard8::Key6),
            Key::E => Some(Keyboard8::Key7),
            Key::R => Some(Keyboard8::Key14),
            Key::A => Some(Keyboard8::Key8),
            Key::S => Some(Keyboard8::Key9),
            Key::D => Some(Keyboard8::Key10),
            Key::F => Some(Keyboard8::Key15),
            Key::Y => Some(Keyboard8::Key11),
            Key::X => Some(Keyboard8::Key1),
            Key::C => Some(Keyboard8::Key12),
            Key::V => Some(Keyboard8::Key16),
            _ => None,
        }
    }
    // Converts Chip8-Key to the corresponding Key
    // parameter: Keyboard8
    // return: Option<Key> from minifb
    pub fn to_key(key: Keyboard8) -> Option<Key> {
        match key {
            Keyboard8::Key2 => Some(Key::Key1),
            Keyboard8::Key3 => Some(Key::Key2),
            Keyboard8::Key4 => Some(Key::Key3),
            Keyboard8::Key13 => Some(Key::Key4),
            Keyboard8::Key5 => Some(Key::Q),
            Keyboard8::Key6 => Some(Key::W),
            Keyboard8::Key7 => Some(Key::E),
            Keyboard8::Key14 => Some(Key::R),
            Keyboard8::Key8 => Some(Key::A),
            Keyboard8::Key9 => Some(Key::S),
            Keyboard8::Key10 => Some(Key::D),
            Keyboard8::Key15 => Some(Key::F),
            Keyboard8::Key11 => Some(Key::Y),
            Keyboard8::Key1 => Some(Key::X),
            Keyboard8::Key12 => Some(Key::C),
            Keyboard8::Key16 => Some(Key::V),
        }
    }
    // Used to get the Key-Value from an u8
    // paramter: u8
    // return: Option<Keyboard8>
    pub fn new_key(value: u8) -> Option<Keyboard8> {
        match value {
            0x0 => Some(Keyboard8::Key1),
            0x1 => Some(Keyboard8::Key2),
            0x2 => Some(Keyboard8::Key3),
            0x3 => Some(Keyboard8::Key4),
            0x4 => Some(Keyboard8::Key5),
            0x5 => Some(Keyboard8::Key6),
            0x6 => Some(Keyboard8::Key7),
            0x7 => Some(Keyboard8::Key8),
            0x8 => Some(Keyboard8::Key9),
            0x9 => Some(Keyboard8::Key10),
            0xA => Some(Keyboard8::Key11),
            0xB => Some(Keyboard8::Key12),
            0xC => Some(Keyboard8::Key13),
            0xD => Some(Keyboard8::Key14),
            0xE => Some(Keyboard8::Key15),
            0xF => Some(Keyboard8::Key16),
            _ => None,
        }
    }
}
