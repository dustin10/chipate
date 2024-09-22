use crate::core::memory::RAM;

use anyhow::Context;
use std::{fs::read, path::Path};

pub mod cpu;
pub mod gfx;
pub mod memory;

const PROGRAM_START_ADDR: u16 = 0x200;

#[derive(Clone, Debug)]
pub struct Program {
    pub name: String,
    data: Vec<u8>,
}

impl Program {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self {
            name,
            data
        }
    }
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        tracing::debug!("loading program from path: {:?}", path.as_ref());

        let name = path.as_ref()
            .file_name()
            .and_then(|s| s.to_str().map(String::from))
            .unwrap_or_else(|| String::from("Unknown"));

        let data = std::fs::read(path)?;

        Ok(Self::new(name, data))
    }
    pub fn load(&self, memory: &mut RAM) {
        memory.write_block(PROGRAM_START_ADDR, &self.data);
    }
}
