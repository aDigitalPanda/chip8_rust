mod keyboard;
mod memory;

use keyboard::Keyboard8;
use minifb::Key;

fn main() {
    let key = Keyboard8::from_key(Key::A);
    println!("{:#X?}", key.unwrap() as u8);
}
