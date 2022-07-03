pub enum Flag {
    Carry  = 1 << 0,
    Parity = 1 << 2,
    Zero   = 1 << 6,
    Sign   = 1 << 7,
}

impl From<Flag> for u8 {
    fn from(flag: Flag) -> u8 {
        flag as u8
    }
}
