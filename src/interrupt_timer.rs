use std::ops::AddAssign;

pub struct InterruptTimers {
    pub number:    u8,
    pub interrupt: bool,
    timers:        [InterruptTimer; 2],
}

impl InterruptTimers {
    pub fn new() -> InterruptTimers {
        InterruptTimers {
            number:    0,
            interrupt: false,
            timers: [
                InterruptTimer::new(1, 16667),
                InterruptTimer::new(2, 0),
            ],
        }
    }
}

impl AddAssign<u16> for InterruptTimers {
    fn add_assign(&mut self, other: u16) {
        for timer in &mut self.timers {
            *timer += other;
            if timer.interrupt {
                self.interrupt  = true;
                self.number     = timer.number;
                timer.interrupt = false;
            }
        }
    }
}

struct InterruptTimer {
    number:    u8,
    cycles:    u16,
    interrupt: bool,
}

impl InterruptTimer {
    fn new(number: u8, cycles: u16) -> InterruptTimer {
        InterruptTimer {
            number,
            cycles,
            interrupt: false,
        }
    }
}

impl AddAssign<u16> for InterruptTimer {
    fn add_assign(&mut self, other: u16) {
        self.cycles += other;

        if self.cycles >= 33334 {
            self.cycles   -= 33334;
            self.interrupt = true;
        }
    }
}
