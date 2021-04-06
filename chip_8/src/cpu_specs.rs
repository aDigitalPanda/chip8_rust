use crate::instructions::{Instruction, Input};
use crate::display::Display;

use rand::{thread_rng, Rng};

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
    display: Display,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut memory = [0; CHIP8_RAM_SIZE];
        let mut display = Display::new();

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
            display: display,
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

    fn instruction(&mut self, instruction: &Instruction) -> u16 {
        match *instruction {
            Instruction::SystemJump(address) => self.program_counter,
            Instruction::Clear => { 
                self.display.clear();
                self.program_counter
            },
            Instruction::Return => {
                if self.stack_pointer > 0 {
                    let address = self.stack[(self.stack_pointer - 1) as usize];
                    self.stack_pointer -= 1;
                    return address + 2;
                }
                self.program_counter
            },
            Instruction::Jump(address) => address,
            Instruction::CallSub(address) => {
                self.stack_pointer += 1;
                self.stack[(self.stack_pointer - 1) as usize] = self.program_counter;
                address
            },
            Instruction::SkipEq(register, byte) => {
                if self.get_reg(register) == byte {
                    self.program_counter + 4
                } else {
                    self.program_counter + 2
                }
            },
            Instruction::SkipNeq(register, byte) => {
                if self.get_reg(register) != byte {
                    self.program_counter + 4
                } else {
                    self.program_counter + 2
                }
            },
            Instruction::SkipEqRegister(register1, register2) => {
                if self.get_reg(register1) == self.get_reg(register2) {
                    self.program_counter + 4
                } else {
                    self.program_counter + 2
                }
            },
            Instruction::ByteInRegister(register, byte) => {
                self.set_reg(byte, register);
                self.program_counter + 2
            },
            Instruction::Add(register, byte) => {
                let value = self.get_reg(register);
                self.set_reg(byte + value, register);
                self.program_counter + 2
            },
            Instruction::RegisterInRegister(register1, register2) => {
                self.set_reg(self.get_reg(register2), register1);
                self.program_counter + 2
            }, 
            Instruction::Or(register1, register2) => {
                let value = self.get_reg(register1) | self.get_reg(register2);
                self.set_reg(value, register1);
                self.program_counter + 2
            }, 
            Instruction::And(register1, register2) => {
                let value = self.get_reg(register1) & self.get_reg(register2);
                self.set_reg(value, register1);
                self.program_counter + 2
            }, 
            Instruction::Xor(register1, register2) => {
                let value = self.get_reg(register1) ^ self.get_reg(register2);
                self.set_reg(value, register1);
                self.program_counter + 2
            }, 
            Instruction::AddRegister(register1, register2) => {
                let result = self.get_reg(register1) as u16 + self.get_reg(register2) as u16;
                if result > 0xFF {
                    self.register[0xf] = 1;
                } else {
                    self.register[0xf] = 0;
                }
                self.set_reg((result & 0xFF) as u8, register1);
                self.program_counter + 2
            },
            Instruction::SubRegister(register1, register2) => {
                if self.get_reg(register1) > self.get_reg(register2) {
                    self.register[0xf] = 1;
                } else {
                    self.register[0xf] = 0;
                }
                self.set_reg(self.get_reg(register1) - self.get_reg(register2), register1);
                self.program_counter + 2
            },
            Instruction::ShiftRight(register) => {
                let lsb = self.get_reg(register) & 0x1;
                if lsb == 1 {
                    self.register[0xf] = 1;
                } else {
                    self.register[0xf] = 0;
                }
                self.set_reg(self.get_reg(register) / 2, register);
                self.program_counter + 2
            },
            Instruction::ReverseSubRegister(register1, register2) => {
                if self.get_reg(register2) > self.get_reg(register1) {
                    self.register[0xf] = 1;
                } else {
                    self.register[0xf] = 0;
                }
                self.set_reg(self.get_reg(register2) - self.get_reg(register1), register1);
                self.program_counter + 2
            },
            Instruction::ShiftLeft(register) => {
                let msb = (self.get_reg(register) >> 7) & 0x1;
                if msb == 1 {
                    self.register[0xf] = 1;
                } else {
                    self.register[0xf] = 0;
                }
                self.set_reg(self.get_reg(register) * 2, register);
                self.program_counter + 2
            },
            Instruction::SkipNeqRegister(register1, register2) => {
                if self.get_reg(register1) != self.get_reg(register2) {
                    self.program_counter + 4
                } else {
                    self.program_counter + 2
                }
            },
            Instruction::SetI(address) => {
                self.i_reg = address;
                self.program_counter + 2 
            },
            Instruction::JumpPlus(address) => address + self.register[0x0] as u16,
            Instruction::RandomAnd(register, byte) => {
                let mut rng = thread_rng();
                let rand: u8 = rng.gen();
                self.set_reg(rand & byte, register);
                self.program_counter + 2
            },
            Instruction::Draw(register1, register2, byte) => {
                let x = self.get_reg(register1);
                let y = self.get_reg(register2);
                let from = self.i_reg as usize;
                let to = from + (byte as usize); 
                self.register[0xF] = self.display.draw(x, y, &self.memory[from..to]) as u8;
                self.program_counter + 2
            },
            //Instruction::SkipIfPressed(register) => 
            //Instruction::SkipIfNotPressed(register) => 
            Instruction::GetDelayTimerInRegister(register) => {
                self.set_reg(self.delay_timer, register);
                self.program_counter + 2
            },
            //Instruction::WaitAndStoreKey(register) => 
            Instruction::SetDelayTimer(register) => {
                self.delay_timer = self.get_reg(register);
                self.program_counter + 2
            },
            Instruction::SetSoundTimer(register) => {
                self.sound_timer = self.get_reg(register);
                self.program_counter + 2
            },
            Instruction::AddI(register) => {
                self.i_reg = self.i_reg + self.get_reg(register) as u16;
                self.program_counter + 2
            },
            Instruction::SpriteLocation(register) => {
                let value = self.get_reg(register);
                let location = (value * 5) as u16;
                self.i_reg = location;
                self.program_counter + 2
            },
            Instruction::BCDStore(register) => {
                let value = self.get_reg(register);
                let ones = value % 10;
                let tens = (value / 10) % 10;
                let hundreds = (value / 100) % 10;
                let location = self.i_reg as usize;
                self.memory[location] = hundreds;
                self.memory[location + 1] = tens;
                self.memory[location + 2] = ones;
                self.program_counter + 2
            },
            Instruction::StoreRegister(register) => {
                let location = self.i_reg as usize;
                for i in 0..=register {
                    self.memory[location + i as usize] = self.register[i as usize];
                }
                self.program_counter + 2
            },
            Instruction::LoadRegister(register) => {
                let location = self.i_reg as usize;
                for i in 0..=register {
                    self.register[i as usize] = self.memory[location + i as usize];
                }
                self.program_counter + 2
            },
            
            //just a placeholder
            _ => self.program_counter,
        }
    }

    fn get_reg(&self, pos: u8) -> u8 {
        self.register[pos as usize] as u8
    }

    fn set_reg(&mut self, value: u8, pos: u8) {
        self.register[pos as usize] = value;
    }
}