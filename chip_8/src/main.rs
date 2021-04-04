mod keyboard;
mod display;
mod instructions;
mod cpu_specs;

use keyboard::Keyboard8;
use minifb::{Key, Window, WindowOptions};
use display::Display;


fn main() {
    let key = Keyboard8::from_key(Key::A);
    println!("{:#X?}", key.unwrap() as u8);

    let mut buffer: Vec<u32> = vec![0; 640 * 320];

    let mut window = match Window::new("Test", 640, 320, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    }; 

    //Test
    let test = (0xB34A >> 8) & 0xF;
    println!("{}", test);
    //.....

    let mut display = Display::new();
    let display_buffer = display.get_buffer();

    //convert display_buffer to window_buffer
    for y in 0..320 {
        let y_coord = y / 10;
        let buffer_offset = y * 640;
        for x in 0..640 {
            let x_coord = x / 10;
            let color = if display_buffer[y_coord][x_coord] {
                0xFFFFFF
            } else {
                0x000000
            };
            buffer[buffer_offset + x] = color;
        }
    }

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (id, i) in buffer.iter_mut().enumerate() {
            if id % 2 == 0 {
                *i = *i;
            } else {
                *i = *i;
            } // write something more funny here! (FF,FF,FF) = 0xFFFFFF
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, 640, 320)
            .unwrap();
    }
}
