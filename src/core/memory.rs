#[derive(Debug)]
pub struct RAM {
    memory: [u8; 4096],
}

impl RAM {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read(&self, address: u8) -> u8 {
        self.memory[address as usize]
    }
    pub fn write(&mut self, address: u8, byte: u8) {
        self.memory[address as usize] = byte;
    }
}

impl Default for RAM {
    fn default() -> Self {
        Self { memory: [0; 4096] }
    }
}
