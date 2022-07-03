use std::ops::AddAssign;
use std::ops::SubAssign;

pub struct PointerRegister {
    register: u16,
}

impl PointerRegister {
    pub fn new() -> PointerRegister {
        PointerRegister {
            register: 0,
        }
    }

    pub fn get(&self) -> u16 {
        self.register
    }

    pub fn set(&mut self, value: u16) {
        self.register = value;
    }
}

impl AddAssign<u16> for PointerRegister {
    fn add_assign(&mut self, other: u16) {
        self.register = self.register.wrapping_add(other);
    }
}

impl SubAssign<u16> for PointerRegister {
    fn sub_assign(&mut self, other: u16) {
        self.register = self.register.wrapping_sub(other);
    }
}
