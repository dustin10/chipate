use crate::core::memory::RAM;

#[derive(Debug, Default)]
struct Registers {
    vs: [u8; 16],
    l: u16,
}

#[derive(Debug, Default)]
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
        todo!()
    }
}

#[derive(Debug)]
pub struct CPU {
    registers: Registers,
    memory: RAM,
    prog_counter: u16,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new(memory: RAM) -> Self {
        Self {
            registers: Registers::default(),
            memory,
            prog_counter: 0,
            stack: Stack::default(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    pub fn execute(&mut self) {
        let instruction = self.fetch();
    }
    fn fetch(&mut self) -> u16 {
        let low = self.memory.read(self.prog_counter) as u16;
        let high = self.memory.read(self.prog_counter + 1) as u16;

        self.prog_counter += 2;

        low << 8 | high
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new(RAM::default())
    }
}
