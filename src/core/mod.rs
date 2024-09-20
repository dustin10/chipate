use crate::core::memory::RAM;

use anyhow::Context;
use std::{fs::read, path::Path};

pub mod cpu;
pub mod gfx;
pub mod memory;

const PROGRAM_START_ADDR: u16 = 0x200;

#[derive(Debug)]
pub struct Program {
    data: Vec<u8>,
}

impl Program {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        tracing::debug!("loading program from path {:?}", path.as_ref());

        let data = std::fs::read(path)?;

        Ok(Self { data })
    }
    pub fn load(&self, memory: &mut RAM) {
        memory.write_block(PROGRAM_START_ADDR, &self.data);
    }
}
