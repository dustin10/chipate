#![allow(unused)]

use crate::core::{cpu::CPU, gfx::Display, Program};

pub mod core;

#[derive(Debug)]
pub struct Emu {
    program: Program,
    cpu: CPU,
    display: Display,
}

impl Emu {
    pub fn init_with_program(program: Program) -> Self {
        Self {
            program,
            cpu: CPU::new(),
            display: Display::new(),
        }
    }
}
