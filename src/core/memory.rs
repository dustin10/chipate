#[derive(Clone, Debug)]
pub struct RAM {
    data: [u8; 4096],
}

impl RAM {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
    pub fn write(&mut self, address: u16, byte: u8) {
        self.data[address as usize] = byte;
    }
    pub fn write_block(&mut self, start_addr: u16, bytes: &[u8]) {
        let dest_start = start_addr as usize;
        let dest_end = start_addr as usize + bytes.len();

        self.data[dest_start..dest_end].copy_from_slice(&bytes[0..bytes.len()]);
    }
}

impl Default for RAM {
    fn default() -> Self {
        Self { data: [0; 4096] }
    }
}
