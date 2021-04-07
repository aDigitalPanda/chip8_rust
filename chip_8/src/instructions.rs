pub type Address = u16;
pub type Register = u8;
pub type Byte = u8;

#[derive(Debug)]
pub enum Instruction {
    SystemJump(Address),
    Clear,
    Return,
    Jump(Address),
    CallSub(Address),
    SkipEq(Register, Byte),
    SkipNeq(Register, Byte),
    SkipEqRegister(Register, Register),
    ByteInRegister(Register, Byte),
    Add(Register, Byte),
    RegisterInRegister(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    AddRegister(Register, Register),
    SubRegister(Register, Register),
    ShiftRight(Register),
    ReverseSubRegister(Register, Register),
    ShiftLeft(Register),
    SkipNeqRegister(Register, Register),
    SetI(Address),
    JumpPlus(Address),
    RandomAnd(Register, Byte),
    Draw(Register, Register, Byte),
    SkipIfPressed(Register),
    SkipIfNotPressed(Register),
    GetDelayTimerInRegister(Register),
    WaitAndStoreKey(Register),
    SetDelayTimer(Register),
    SetSoundTimer(Register),
    AddI(Register),
    SpriteLocation(Register),
    BCDStore(Register),
    StoreRegister(Register),
    LoadRegister(Register),
}

pub struct Input {
    input: u16, 
}

impl Input {
    pub fn new(input: u16) -> Input {
        Input {input: input}
    }

    pub fn input_to_instruction(&self) -> Option<Instruction> {
        match self.xooo() as u8 {
            0x0 => match self.oxxx() {
                0x0E0 => Some(Instruction::Clear),
                0x0EE => Some(Instruction::Return),
                address => Some(Instruction::SystemJump(address)),
            }
            0x1 => Some(Instruction::Jump(self.oxxx())),
            0x2 => Some(Instruction::CallSub(self.oxxx())),
            0x3 => Some(Instruction::SkipEq(self.oxoo(), self.ooxx())),
            0x4 => Some(Instruction::SkipNeq(self.oxoo(), self.ooxx())),
            0x5 => Some(Instruction::SkipEqRegister(self.oxoo(), self.ooxo())),
            0x6 => Some(Instruction::ByteInRegister(self.oxoo(), self.ooxx())),
            0x7 => Some(Instruction::Add(self.oxoo(), self.ooxx())),
            0x8 => match self.ooox() {
                0x0 => Some(Instruction::RegisterInRegister(self.oxoo(), self.ooxo())),
                0x1 => Some(Instruction::Or(self.oxoo(), self.ooxo())),
                0x2 => Some(Instruction::And(self.oxoo(), self.ooxo())),
                0x3 => Some(Instruction::Xor(self.oxoo(), self.ooxo())),
                0x4 => Some(Instruction::AddRegister(self.oxoo(), self.ooxo())),
                0x5 => Some(Instruction::SubRegister(self.oxoo(), self.ooxo())),
                0x6 => Some(Instruction::ShiftRight(self.oxoo())),
                0x7 => Some(Instruction::ReverseSubRegister(self.oxoo(), self.ooxo())),
                0xE => Some(Instruction::ShiftLeft(self.oxoo())),
                _ => None,
            }
            0x9 => Some(Instruction::SkipNeqRegister(self.oxoo(), self.ooxo())),
            0xA => Some(Instruction::SetI(self.oxxx())),
            0xB => Some(Instruction::JumpPlus(self.oxxx())),
            0xC => Some(Instruction::RandomAnd(self.oxoo(), self.ooxx())),
            0xD => Some(Instruction::Draw(self.oxoo(), self.ooxo(), self.ooox())),
            0xE => match self.ooxx() {
                0x9E => Some(Instruction::SkipIfPressed(self.oxoo())),
                0xA1 => Some(Instruction::SkipIfNotPressed(self.oxoo())),
                _ => None,
            }
            0xF => match self.ooxx() {
                0x07 => Some(Instruction::GetDelayTimerInRegister(self.oxoo())),
                0x0A => Some(Instruction::WaitAndStoreKey(self.oxoo())),
                0x15 => Some(Instruction::SetDelayTimer(self.oxoo())),
                0x18 => Some(Instruction::SetSoundTimer(self.oxoo())),
                0x1E => Some(Instruction::AddI(self.oxoo())),
                0x29 => Some(Instruction::SpriteLocation(self.oxoo())),
                0x33 => Some(Instruction::BCDStore(self.oxoo())),
                0x55 => Some(Instruction::StoreRegister(self.oxoo())),
                0x65 => Some(Instruction::LoadRegister(self.oxoo())),
                _ => None,
            }
            _ => None,
        }
    }

    fn xooo(&self) -> u8 {
        ((self.input >> 12) & 0xF) as u8
    }

    fn oxxx(&self) -> u16 {
        (self.input & 0xFFF) as u16
    }

    fn ooxx(&self) -> u8 {
        (self.input & 0xFF) as u8
    }

    fn oxoo(&self) -> u8 {
        ((self.input >> 8) & 0xF) as u8
    }

    fn ooxo(&self) -> u8 {
        ((self.input >> 4) & 0xF) as u8
    }
    
    fn ooox(&self) -> u8 {
        (self.input & 0xF) as u8
    }
}