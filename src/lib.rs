#![allow(unused)]

pub mod core;

use crate::core::{
    cpu::CPU,
    gfx::{Display, Font},
    memory::RAM,
    Program,
};

use std::time::{Duration, Instant};

pub const PROGRAM_START_ADDR: u16 = 0x200;

#[derive(Clone, Debug)]
pub struct Config {
    pub instructions_per_sec: u16,
    pub font: Font,
}

#[derive(Clone, Debug)]
pub struct Emu {
    config: Config,
    cpu: CPU,
    memory: RAM,
    display: Display,
}

impl Emu {
    pub fn new(config: Config) -> Self {
        let mut memory = RAM::new();

        config.font.load(&mut memory);
        tracing::debug!("loaded {} font into memory", config.font.name);

        Self {
            config,
            cpu: CPU::default(),
            memory,
            display: Display::new(),
        }
    }
    pub fn load_program(&mut self, program: Program) {
        program.load(&mut self.memory);
        tracing::debug!("loaded {} program into memory", program.name);
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        let min_ms_per_tick = 1000_u128 / self.config.instructions_per_sec as u128;

        let mut last_instant = Instant::now();

        loop {
            let elapsed = last_instant.elapsed();
            if elapsed.as_millis() >= min_ms_per_tick {
                self.cpu.tick(&mut self.memory);

                last_instant = Instant::now();
            }
        }

        Ok(())
    }
}
