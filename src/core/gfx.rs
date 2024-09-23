use crate::core::memory::RAM;

pub const WINDOW_PIXELS_WIDTH: u8 = 64;

pub const WINDOW_PIXELS_HEIGHT: u8 = 32;

const NUM_PIXELS: usize = 64 * 32;

#[derive(Clone, Debug)]
pub struct Display {
    pixels: [bool; NUM_PIXELS],
}

impl Display {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }
    pub fn read_pixel(&self, idx: u16) -> bool {
        self.pixels[idx as usize]
    }
    pub fn write_pixel(&mut self, idx: u16, value: bool) {
        self.pixels[idx as usize] = value;
    }
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [false; NUM_PIXELS],
        }
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
