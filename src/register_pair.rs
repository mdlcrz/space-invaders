pub struct RegisterPair {
    pub rh: u8,
    pub rl: u8,
}

impl RegisterPair {
    pub fn new() -> RegisterPair {
        RegisterPair {
            rh: 0,
            rl: 0,
        }
    }

    pub fn get(&self) -> u16 {
        (u16::from(self.rh) << 8) | u16::from(self.rl)
    }

    pub fn set(&mut self, value: u16) {
        self.rh = (value >> 8)   as u8;
        self.rl = (value & 0xff) as u8;
    }
}
