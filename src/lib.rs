#![allow(unused)]

pub mod core;

use crate::core::{
    cpu::{Mode, CPU},
    gfx::{Display, Font},
    memory::RAM,
    Program,
};

use std::time::{Duration, Instant};

pub const PROGRAM_START_ADDR: u16 = 0x200;

#[derive(Clone, Debug)]
pub struct Config {
    pub mode: Mode,
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
        let mut last_tick = Instant::now();

        let min_ms_per_timer_dec = 1000_u128 / 60_u128;
        let mut last_timer = Instant::now();

        loop {
            let tick_elapsed = last_tick.elapsed();
            if tick_elapsed.as_millis() >= min_ms_per_tick {
                self.cpu.tick(&mut self.memory, &mut self.display);

                last_tick = Instant::now();
            }

            let timer_elapsed = last_timer.elapsed();
            if timer_elapsed.as_millis() >= min_ms_per_timer_dec {
                self.cpu.dec_timers();
            }
        }

        Ok(())
    }
}

fn print_display_state(display: &Display) {
    let mut grid = String::new();
    for r in 0..32 {
        grid.push('\n');
        for c in 0..64 {
            let idx = r as u16 * 64 + c as u16;

            let white = display.read_pixel(idx);
            if white {
                grid.push('\u{2588}');
            } else {
                grid.push(' ');
            }
        }
    }
    println!("{}", grid);
}
