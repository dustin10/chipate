#![allow(unused)]

use std::time::Instant;

use crate::core::{
    cpu::CPU,
    gfx::{Display, Font},
    memory::RAM,
    Program,
};

pub mod core;

#[derive(Clone, Debug)]
pub struct Emu {
    ips: u16,
    program: Program,
    cpu: CPU,
    display: Display,
    font: Font,
}

impl Emu {
    pub fn new(ips: u16, program: Program, font: Font) -> Self {
        let mut memory = RAM::new();

        program.load(&mut memory);
        tracing::debug!("loaded {} program into memory", program.name);

        font.load(&mut memory);
        tracing::debug!("loaded {} font into memory", font.name);

        Self {
            ips,
            program,
            cpu: CPU::new(memory),
            display: Display::new(),
            font,
        }
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut curr_time = Instant::now();

        self.cpu.tick();
        Ok(())
    }
}
