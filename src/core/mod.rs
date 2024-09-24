use crate::{core::memory::RAM, PROGRAM_START_ADDR};

use anyhow::Context;
use std::{fs::read, path::Path};

pub mod cpu;
pub mod memory;

#[derive(Clone, Debug)]
pub struct Program {
    pub name: String,
    data: Vec<u8>,
}

impl Program {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self { name, data }
    }
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        tracing::debug!("loading program from path: {:?}", path.as_ref());

        let name = path
            .as_ref()
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

const FONT_GLYPH_WIDTH: u8 = 4;

const FONT_GLYPH_HEIGHT: u8 = 5;

const FONT_START_ADDR: u16 = 0x050;

const DEFAULT_FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    data: [u8; 80],
}

impl Font {
    pub fn new(name: String, data: [u8; 80]) -> Self {
        Self { name, data }
    }
    pub fn load(&self, memory: &mut RAM) {
        memory.write_block(FONT_START_ADDR, &self.data);
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new(String::from("Default"), DEFAULT_FONT_DATA)
    }
}
