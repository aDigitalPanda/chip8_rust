mod keyboard;
mod display;
mod instructions;
mod cpu_specs;

use keyboard::Keyboard8;
use minifb::{Key, Window, WindowOptions, KeyRepeat};
use cpu_specs::Cpu;
use std::fs;
use std::env;

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 320;
const WINDOW_SCALE: usize = WINDOW_HEIGHT / 32;
const GAME_DIR: &str = "c8games/";

fn main() {
    // get the programm from command line
    let args: Vec<String> = env::args().collect();
    let file_location = GAME_DIR.to_owned() + &args[1];


    let mut window = match Window::new(&args[1], WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    let program = fs::read(file_location).expect("Failure");
    let mut cpu = Cpu::new(program); 
    let mut buffer: Vec<u32> = vec![0; WINDOW_HEIGHT * WINDOW_WIDTH];

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Run Program and update buffer
        cpu.instruction_cycle(&window);
        let display_buffer = cpu.display.get_buffer();
        //---------------------------

        // handle key press
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

        // timer decrement
        cpu.delay_timer();
        cpu.sound_timer();
        //----------------
    
        // Fill Buffer for the drawing with respect of the scaling
        for y in 0..WINDOW_HEIGHT {
            let y_coord = y / WINDOW_SCALE;
            let buffer_offset = y * WINDOW_WIDTH;
            for x in 0..WINDOW_WIDTH {
                let x_coord = x / WINDOW_SCALE;
                let color = if display_buffer[y_coord][x_coord] {
                    0xFFFFFF
                } else {
                    0x000000
                };
                buffer[buffer_offset + x] = color;
            }
        }
        //------------------------------

        // update the window with our buffer 
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
