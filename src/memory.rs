pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: vec![0; 0x5000],
        }
    }

    pub fn read(&self, address: usize, buf: &mut [u8]) {
        buf.copy_from_slice(&self.memory[address..address + buf.len()]);
    }

    pub fn read8(&self, address: u16) -> u8 {
        let mut data = [0u8; 1];

        self.read(address.into(), &mut data);
        u8::from_le_bytes(data)
    }

    pub fn read16(&self, address: u16) -> u16 {
        let mut data = [0u8; 2];

        self.read(address.into(), &mut data);
        u16::from_le_bytes(data)
    }

    pub fn write(&mut self, address: usize, buf: &[u8]) {
        self.memory[address..address + buf.len()].copy_from_slice(buf);
    }

    pub fn write8(&mut self, address: u16, data: u8) {
        self.write(address.into(), &data.to_le_bytes());
    }

    pub fn write16(&mut self, address: u16, data: u16) {
        self.write(address.into(), &data.to_le_bytes());
    }
}
