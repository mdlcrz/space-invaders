use crate::flag::Flag;

pub struct ProgramStateWord {
    flags: u8,
    pub a: u8,
}

impl ProgramStateWord {
    pub fn new() -> ProgramStateWord {
        ProgramStateWord {
            flags: 0b00000010,
            a:     0,
        }
    }

    pub fn get(&self) -> u16 {
        (u16::from(self.flags) << 8) | u16::from(self.a)
    }

    pub fn set(&mut self, value: u16) {
        self.flags = (value >> 8)   as u8;
        self.a     = (value & 0xff) as u8;
    }

    pub fn get_carry(&self) -> u8 {
        self.get_flag(Flag::Carry)
    }

    fn get_flag(&self, flag: Flag) -> u8 {
        u8::from(self.is_flag_set(flag))
    }

    pub fn set_carry(&mut self, value: u8) {
        self.set_flag(Flag::Carry, value);
    }

    pub fn set_parity(&mut self, value: u8) {
        self.set_flag(Flag::Parity, value);
    }

    pub fn set_zero(&mut self, value: u8) {
        self.set_flag(Flag::Zero, value);
    }

    pub fn set_sign(&mut self, value: u8) {
        self.set_flag(Flag::Sign, value);
    }

    fn set_flag(&mut self, flag: Flag, value: u8) {
        let should_set_flag = match flag {
            Flag::Carry  => value != 0,
            Flag::Parity => value.count_ones() & 1 == 0,
            Flag::Zero   => value == 0,
            Flag::Sign   => value & 0x80 == 0x80,
        };

        if should_set_flag {
            self.flags |=  u8::from(flag);
        } else {
            self.flags &= !u8::from(flag);
        }
    }

    pub fn is_carry_set(&self) -> bool {
        self.is_flag_set(Flag::Carry)
    }

    pub fn is_parity_set(&self) -> bool {
        self.is_flag_set(Flag::Parity)
    }

    pub fn is_zero_set(&self) -> bool {
        self.is_flag_set(Flag::Zero)
    }

    pub fn is_sign_set(&self) -> bool {
        self.is_flag_set(Flag::Sign)
    }

    fn is_flag_set(&self, flag: Flag) -> bool {
        self.flags & u8::from(flag) != 0
    }
}
