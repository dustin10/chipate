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
    font: Font,
}

impl Emu {
    pub fn new(program: Program, font: Font) -> Self {
        let mut memory = RAM::new();

        program.load(&mut memory);
        tracing::debug!("loaded program into memory");

        font.load(&mut memory);
        tracing::debug!("loaded font into memory");

        Self {
            program,
            cpu: CPU::new(memory),
            display: Display::new(),
            font,
        }
    }
}
