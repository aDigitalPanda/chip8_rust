mod keyboard;
mod display;
mod memory;

use keyboard::Keyboard8;
use minifb::{Key, Window, WindowOptions};
use display::Display;
use memory::Memory;

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

    let mut memory = Memory::new();
    let mut display = Display::new();

    let mem = memory.get_memory();
    display.draw(10, 1, &mem[5..10]);
    display.draw(15, 1, &mem[10..15]);
    display.draw(20, 1, &mem[15..20]);
    display.draw(25, 1, &mem[60..65]);

    display.draw(10, 7, &mem[20..25]);
    display.draw(15, 7, &mem[25..30]);
    display.draw(20, 7, &mem[30..35]);
    display.draw(25, 7, &mem[65..70]);

    display.draw(10, 13, &mem[35..40]);
    display.draw(15, 13, &mem[40..45]);
    display.draw(20, 13, &mem[45..50]);
    display.draw(25, 13, &mem[70..75]);

    display.draw(10, 19, &mem[50..55]);
    display.draw(15, 19, &mem[0..5]);
    display.draw(20, 19, &mem[55..60]);
    display.draw(25, 19, &mem[75..80]);

    let display_buffer = display.get_buffer();

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
