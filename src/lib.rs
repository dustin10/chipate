#![allow(unused)]

use crate::core::{
    cpu::CPU,
    gfx::{Display, Font},
    memory::RAM,
    Program,
};

pub mod core;

#[derive(Debug)]
pub struct Emu {
    program: Program,
    cpu: CPU,
    display: Display,
}

impl Emu {
    pub fn new(program: Program) -> Self {
        let mut memory = RAM::new();

        Font::load(&mut memory);
        program.load(&mut memory);

        Self {
            program,
            cpu: CPU::new(memory),
            display: Display::new(),
        }
    }
}
