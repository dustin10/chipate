use anyhow::Context;
use cpu::CPU;
use std::{fs::read, path::Path};

pub mod cpu;
pub mod gfx;
pub mod memory;

#[derive(Debug)]
pub struct Program {
    data: Vec<u8>,
}

impl Program {
    pub fn load_from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        tracing::debug!("loading program from path {:?}", path.as_ref());

        let data = std::fs::read(path).context("failed to load CHIP-8 program from disk")?;

        Ok(Self { data })
    }
}
