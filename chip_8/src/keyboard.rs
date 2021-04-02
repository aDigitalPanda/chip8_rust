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
}
