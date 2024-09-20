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
    fn push(&self, address: u16) {
        todo!()
    }
    fn pop(&mut self) -> u16 {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct CPU {
    registers: Registers,
    memory: RAM,
    prog_counter: u16,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }
}
