mod keyboard;
mod display;
mod instructions;
mod cpu_specs;

use keyboard::Keyboard8;
use minifb::{Key, Window, WindowOptions, KeyRepeat};
use display::Display;
use cpu_specs::Cpu;
use std::fs;

fn main() {
    let mut window = match Window::new("Test", 640, 320, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    
    let program = fs::read("snake.ch8").expect("Failure");


    let mut cpu = Cpu::new(program);
    let mut display = Display::new();
    let display_buffer = display.get_buffer();
    let mut buffer: Vec<u32> = vec![0; 640 * 320];



    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for (id, i) in buffer.iter_mut().enumerate() {
        //     if id % 2 == 0 {
        //         *i = *i;
        //     } else {
        //         *i = *i;
        //     } // write something more funny here! (FF,FF,FF) = 0xFFFFFF
        // }

        cpu.instruction_cycle(&window);


        //handle key press
        let key_pressed = window.get_keys_pressed(KeyRepeat::Yes);
        let key = match key_pressed {
            Some(keys) => if !keys.is_empty() {
                Some(keys[0])
            } else {
                None
            },
            _ => None,
        };
        let chip8_key = key.map(Keyboard8::from_key).flatten();
        cpu.key_handle(chip8_key);
        //-----------------

        //timer decrement
        cpu.delay_timer();
        cpu.sound_timer();
        //----------------

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
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
        window
            .update_with_buffer(&buffer, 640, 320)
            .unwrap();
    }
}
