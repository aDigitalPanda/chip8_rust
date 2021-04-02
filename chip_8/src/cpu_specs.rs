const REGISTER_SIZE usize = 16;
const STACK_SIZE usize = 16;

use memory;

pub struct Cpu {
    register: [u8; REGISTER_SIZE],
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; STACK_SIZE],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            register: [0; REGISTER_SIZE],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: CHIP8_RAM_OFFSET as u16,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
        }
    }

    //---------
}