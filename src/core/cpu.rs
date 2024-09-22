use crate::core::memory::RAM;

#[derive(Clone, Debug, Default)]
struct Registers {
    vs: [u8; 16],
    l: u16,
}

#[derive(Clone, Debug, Default)]
struct Stack {
    data: Vec<u16>,
}

impl Stack {
    fn push(&mut self, address: u16) {
        self.data.push(address);
    }
    fn pop(&mut self) -> Option<u16> {
        self.data.pop()
    }
}

#[derive(Debug)]
enum Instruction {
    ClearScreen,
    Jump,
}

impl Instruction {
    fn from_op_code(op_code: u16) -> Option<Instruction> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct CPU {
    registers: Registers,
    prog_counter: u16,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            prog_counter: 0,
            stack: Stack::default(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    pub fn tick(&mut self, memory: &mut RAM) {
        let op_code = self.fetch(memory);

        match Instruction::from_op_code(op_code) {
            None => tracing::warn!("enountered unknown op code: {:X?}", op_code),
            Some(instruction) => self.execute(instruction),
        }
    }
    fn fetch(&mut self, memory: &mut RAM) -> u16 {
        let low = memory.read(self.prog_counter) as u16;
        let high = memory.read(self.prog_counter + 1) as u16;

        self.prog_counter += 2;

        low << 8 | high
    }
    fn execute(&mut self, instruction: Instruction) {
        todo!()
    }
}
