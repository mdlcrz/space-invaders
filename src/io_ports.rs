pub struct IOPorts {
    input:          InputPorts,
    output:         OutputPorts,
    shift_register: u16,
}

impl IOPorts {
    pub fn new() -> IOPorts {
        IOPorts {
            input:          InputPorts::new(),
            output:         OutputPorts::new(),
            shift_register: 0,
        }
    }

    pub fn read(&self, port: u8) -> u8 {
        match port {
            0 => self.input.input0,
            1 => self.input.input1,
            2 => self.input.input2,
            3 => {
                let shift_amount = self.output.shift_amount;

                (self.shift_register >> (8 - shift_amount)) as u8
            }
            _ => panic!("invalid read port"),
        }
    }

    pub fn write(&mut self, port: u8, value: u8) {
        match port {
            2 => self.output.shift_amount = value & 0b111,
            3 => self.output.sound1 = value,
            4 => {
                self.shift_register >>= 8;
                self.shift_register  |= u16::from(value) << 8;
            }
            5 => self.output.sound2 = value,
            6 => self.output.watchdog = value,
            _ => panic!("invalid write port"),
        }
    }
}

struct InputPorts {
    input0: u8,
    input1: u8,
    input2: u8,
}

impl InputPorts {
    fn new() -> InputPorts {
        InputPorts {
            input0: 0b00001110,
            input1: 0,
            input2: 0,
        }
    }
}

struct OutputPorts {
    shift_amount: u8,
    sound1:       u8,
    sound2:       u8,
    watchdog:     u8,
}

impl OutputPorts {
    fn new() -> OutputPorts {
        OutputPorts {
            shift_amount: 0,
            sound1:       0,
            sound2:       0,
            watchdog:     0,
        }
    }
}
