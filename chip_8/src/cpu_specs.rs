const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const CHIP8_RAM_SIZE: usize = 4096;
const CHIP8_RAM_OFFSET: usize = 0x200;
const SPRITE: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, /* 0 */
    0x20, 0x60, 0x20, 0x20, 0x70, /* 1 */
    0xF0, 0x10, 0xF0, 0x80, 0xF0, /* 2 */
    0xF0, 0x10, 0xF0, 0x10, 0xF0, /* 3 */
    0x90, 0x90, 0xF0, 0x10, 0x10, /* 4 */
    0xF0, 0x80, 0xF0, 0x10, 0xF0, /* 5 */
    0xF0, 0x80, 0xF0, 0x90, 0xF0, /* 6 */
    0xF0, 0x10, 0x20, 0x40, 0x40, /* 7 */
    0xF0, 0x90, 0xF0, 0x90, 0xF0, /* 8 */
    0xF0, 0x90, 0xF0, 0x10, 0xF0, /* 9 */
    0xF0, 0x90, 0xF0, 0x90, 0x90, /* a */
    0xE0, 0x90, 0xE0, 0x90, 0xE0, /* b */
    0xF0, 0x80, 0x80, 0x80, 0xF0, /* c */
    0xE0, 0x90, 0x90, 0x90, 0xE0, /* d */
    0xF0, 0x80, 0xF0, 0x80, 0xF0, /* e */
    0xF0, 0x80, 0xF0, 0x80, 0x80, /* f */
];

pub type Mem = [u8; CHIP8_RAM_SIZE];


pub struct Cpu {
    register: [u8; REGISTER_SIZE],
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; STACK_SIZE],
    memory: Mem,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut memory = [0; CHIP8_RAM_SIZE];

        for (i, value) in SPRITE.iter().enumerate() {
            memory[i] = *value;
        }

        Cpu {
            register: [0; REGISTER_SIZE],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: CHIP8_RAM_OFFSET as u16,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            memory: memory,
        }
    }

    pub fn delay_timer(&mut self) {
       if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
       }
    }

    pub fn sound_timer(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer = self.sound_timer - 1;
            if self.sound_timer == 0 {
                //make beep
            }
       }
    }
    //---------
}