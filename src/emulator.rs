use std::error::Error;

use crate::cpu_state::CpuState;
use crate::disassembler;
use crate::instruction::Instruction;
use crate::interrupt_timer::InterruptTimers;
use crate::io_ports::IOPorts;
use crate::memory::Memory;

pub struct Emulator {
    cpu_state:        CpuState,
    memory:           Memory,
    io_ports:         IOPorts,
    interrupt_timers: InterruptTimers,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu_state:        CpuState::new(),
            memory:           Memory::new(),
            io_ports:         IOPorts::new(),
            interrupt_timers: InterruptTimers::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        self.memory.write(0, std::fs::read(path)?.as_slice());
        Ok(())
    }

    fn get_register(&self, reg: u8) -> u8 {
        match reg {
            0b000 => self.cpu_state.bc.rh,
            0b001 => self.cpu_state.bc.rl,
            0b010 => self.cpu_state.de.rh,
            0b011 => self.cpu_state.de.rl,
            0b100 => self.cpu_state.hl.rh,
            0b101 => self.cpu_state.hl.rl,
            0b110 => self.memory.read8(self.cpu_state.hl.get()),
            0b111 => self.cpu_state.psw.a,
            _     => unreachable!(),
        }
    }

    fn set_register(&mut self, reg: u8, value: u8) {
        match reg {
            0b000 => self.cpu_state.bc.rh = value,
            0b001 => self.cpu_state.bc.rl = value,
            0b010 => self.cpu_state.de.rh = value,
            0b011 => self.cpu_state.de.rl = value,
            0b100 => self.cpu_state.hl.rh = value,
            0b101 => self.cpu_state.hl.rl = value,
            0b110 => self.memory.write8(self.cpu_state.hl.get(), value),
            0b111 => self.cpu_state.psw.a = value,
            _     => unreachable!(),
        };
    }

    fn get_register_pair(&mut self, rp: u8) -> u16 {
        match rp {
            0b000 => self.cpu_state.bc.get(),
            0b001 => self.cpu_state.de.get(),
            0b010 => self.cpu_state.hl.get(),
            0b011 => self.cpu_state.sp.get(),
            0b100 => self.cpu_state.psw.get(),
            _     => unreachable!(),
        }
    }

    fn set_register_pair(&mut self, rp: u8, value: u16) {
        match rp {
            0b000 => self.cpu_state.bc.set(value),
            0b001 => self.cpu_state.de.set(value),
            0b010 => self.cpu_state.hl.set(value),
            0b011 => self.cpu_state.sp.set(value),
            0b100 => self.cpu_state.psw.set(value),
            _     => unreachable!(),
        }
    }

    fn read_sp(&self) -> u16 {
        self.memory.read16(self.cpu_state.sp.get())
    }

    fn write_sp(&mut self, data: u16) {
        self.memory.write16(self.cpu_state.sp.get(), data);
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let program_counter = self.cpu_state.pc.get();
            let mut instruction = [0u8; 3];

            self.memory.read(program_counter.into(), &mut instruction);

            if self.interrupt_timers.interrupt {
                if self.cpu_state.inte {
                    let interrupt_number = self.interrupt_timers.number;

                    instruction[0] = 0b11000111 | (interrupt_number << 3);
                    self.cpu_state.inte = false;
                }
                self.interrupt_timers.interrupt = false;
            }

            let decoded_instruction = disassembler::disassemble(
                program_counter, &instruction)?;

            match decoded_instruction {
                Instruction::Stc => {
                    self.cpu_state.psw.set_carry(1);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Cmc => {
                    self.cpu_state.psw.set_carry(
                        !self.cpu_state.psw.get_carry());

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Inr { reg } => {
                    let register = self.get_register(reg).wrapping_add(1);

                    self.set_register(reg, register);

                    self.cpu_state.psw.set_parity(register);
                    self.cpu_state.psw.set_zero(register);
                    self.cpu_state.psw.set_sign(register);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 5;
                    } else {
                        self.interrupt_timers += 10;
                    }
                }
                Instruction::Dcr { reg } => {
                    let register = self.get_register(reg).wrapping_sub(1);

                    self.set_register(reg, register);

                    self.cpu_state.psw.set_parity(register);
                    self.cpu_state.psw.set_zero(register);
                    self.cpu_state.psw.set_sign(register);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 5;
                    } else {
                        self.interrupt_timers += 10;
                    }
                }
                Instruction::Cma => {
                    self.cpu_state.psw.a = !self.cpu_state.psw.a;

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Daa => {
                    unimplemented!("DAA")
                }
                Instruction::Nop => {
                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Mov { dst, src } => {
                    let register = self.get_register(src);

                    self.set_register(dst, register);

                    self.cpu_state.pc += 1;

                    if dst != 0b110 && src != 0b110 {
                        self.interrupt_timers += 5;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Stax { rp } => {
                    let register_pair = self.get_register_pair(rp);

                    self.memory.write8(register_pair, self.cpu_state.psw.a);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 7;
                }
                Instruction::Ldax { rp } => {
                    let register_pair = self.get_register_pair(rp);

                    self.cpu_state.psw.a = self.memory.read8(register_pair);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 7;
                }
                Instruction::Add { reg } => {
                    let (acc, cy) = self.cpu_state.psw.a
                        .overflowing_add(self.get_register(reg));

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Adc { reg } => {
                    let register = self.get_register(reg);

                    let (acc, cy) = self.cpu_state.psw.a.carrying_add(
                        register, self.cpu_state.psw.is_carry_set());

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Sub { reg } => {
                    let (acc, cy) = self.cpu_state.psw.a
                        .overflowing_sub(self.get_register(reg));

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Sbb { reg } => {
                    let register = self.get_register(reg);

                    let (acc, cy) = self.cpu_state.psw.a.borrowing_sub(
                        register, self.cpu_state.psw.is_carry_set());

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Ana { reg } => {
                    self.cpu_state.psw.a &= self.get_register(reg);

                    self.cpu_state.psw.set_carry(0);
                    self.cpu_state.psw.set_parity(self.cpu_state.psw.a);
                    self.cpu_state.psw.set_zero(self.cpu_state.psw.a);
                    self.cpu_state.psw.set_sign(self.cpu_state.psw.a);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Xra { reg } => {
                    self.cpu_state.psw.a ^= self.get_register(reg);

                    self.cpu_state.psw.set_carry(0);
                    self.cpu_state.psw.set_parity(self.cpu_state.psw.a);
                    self.cpu_state.psw.set_zero(self.cpu_state.psw.a);
                    self.cpu_state.psw.set_sign(self.cpu_state.psw.a);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Ora { reg } => {
                    self.cpu_state.psw.a |= self.get_register(reg);

                    self.cpu_state.psw.set_carry(0);
                    self.cpu_state.psw.set_parity(self.cpu_state.psw.a);
                    self.cpu_state.psw.set_zero(self.cpu_state.psw.a);
                    self.cpu_state.psw.set_sign(self.cpu_state.psw.a);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Cmp { reg } => {
                    let (acc, cy) = self.cpu_state.psw.a
                        .overflowing_sub(self.get_register(reg));

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc += 1;

                    if reg != 0b110 {
                        self.interrupt_timers += 4;
                    } else {
                        self.interrupt_timers += 7;
                    }
                }
                Instruction::Rlc => {
                    let acc = self.cpu_state.psw.a;

                    let (acc, cy) = (acc.rotate_left(1), acc >> 7);

                    self.cpu_state.psw.a = acc;
                    self.cpu_state.psw.set_carry(cy);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Rrc => {
                    let acc = self.cpu_state.psw.a;

                    let (acc, cy) = (acc.rotate_right(1), acc & 1);

                    self.cpu_state.psw.a = acc;
                    self.cpu_state.psw.set_carry(cy);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Ral => {
                    let mut acc = self.cpu_state.psw.a;
                    let mut cy  = self.cpu_state.psw.get_carry();

                    (acc, cy) = ((acc << 1) | cy , acc & 0x80);

                    self.cpu_state.psw.a = acc;
                    self.cpu_state.psw.set_carry(cy);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Rar => {
                    let mut acc = self.cpu_state.psw.a;
                    let mut cy  = self.cpu_state.psw.get_carry();

                    (acc, cy) = ((acc >> 1) | (cy << 7), acc & 1);

                    self.cpu_state.psw.a = acc;
                    self.cpu_state.psw.set_carry(cy);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Push { rp } => {
                    let register_pair = self.get_register_pair(rp);

                    self.cpu_state.sp -= 2;

                    self.write_sp(register_pair);

                    self.cpu_state.pc     +=  1;
                    self.interrupt_timers += 11;
                }
                Instruction::Pop { rp } => {
                    let register_pair = self.read_sp();

                    self.cpu_state.sp += 2;

                    self.set_register_pair(rp, register_pair);

                    self.cpu_state.pc     +=  1;
                    self.interrupt_timers += 10;
                }
                Instruction::Dad { rp } => {
                    let register_pair = self.get_register_pair(rp);

                    let (hl, cy) = self.cpu_state.hl.get()
                        .overflowing_add(register_pair);

                    self.cpu_state.hl.set(hl);
                    self.cpu_state.psw.set_carry(u8::from(cy));

                    self.cpu_state.pc     +=  1;
                    self.interrupt_timers += 10;
                }
                Instruction::Inx { rp } => {
                    let register_pair = self.get_register_pair(rp)
                        .wrapping_add(1);

                    self.set_register_pair(rp, register_pair);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 5;
                }
                Instruction::Dcx { rp } => {
                    let register_pair = self.get_register_pair(rp)
                        .wrapping_sub(1);

                    self.set_register_pair(rp, register_pair);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 5;
                }
                Instruction::Xchg => {
                    std::mem::swap(&mut self.cpu_state.de,
                        &mut self.cpu_state.hl);

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Xthl => {
                    let tmp = self.read_sp();

                    self.write_sp(self.cpu_state.hl.get());
                    self.cpu_state.hl.set(tmp);

                    self.cpu_state.pc     +=  1;
                    self.interrupt_timers += 18;
                }
                Instruction::Sphl => {
                    self.cpu_state.sp.set(self.cpu_state.hl.get());

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 5;
                }
                Instruction::Lxi { rp, data } => {
                    self.set_register_pair(rp, data);

                    self.cpu_state.pc     +=  3;
                    self.interrupt_timers += 10;
                }
                Instruction::Mvi { reg, data } => {
                    self.set_register(reg, data);

                    self.cpu_state.pc += 2;

                    if reg != 0b110 {
                        self.interrupt_timers += 7;
                    } else {
                        self.interrupt_timers += 10;
                    }
                }
                Instruction::Adi { data } => {
                    let (acc, cy) = self.cpu_state.psw.a.overflowing_add(data);

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Aci { data } => {
                    let (acc, cy) = self.cpu_state.psw.a.carrying_add(
                        data, self.cpu_state.psw.is_carry_set());

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Sui { data } => {
                    let (acc, cy) = self.cpu_state.psw.a.overflowing_sub(data);

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Sbi { data } => {
                    let (acc, cy) = self.cpu_state.psw.a.borrowing_sub(
                        data, self.cpu_state.psw.is_carry_set());

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Ani { data } => {
                    let acc = self.cpu_state.psw.a & data;

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(0);
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Xri { data } => {
                    let acc = self.cpu_state.psw.a ^ data;

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(0);
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Ori { data } => {
                    let acc = self.cpu_state.psw.a | data;

                    self.cpu_state.psw.a = acc;

                    self.cpu_state.psw.set_carry(0);
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Cpi { data } => {
                    let (acc, cy) = self.cpu_state.psw.a.overflowing_sub(data);

                    self.cpu_state.psw.set_carry(u8::from(cy));
                    self.cpu_state.psw.set_parity(acc);
                    self.cpu_state.psw.set_zero(acc);
                    self.cpu_state.psw.set_sign(acc);

                    self.cpu_state.pc     += 2;
                    self.interrupt_timers += 7;
                }
                Instruction::Sta { exp } => {
                    self.memory.write8(exp, self.cpu_state.psw.a);

                    self.cpu_state.pc     +=  3;
                    self.interrupt_timers += 13;
                }
                Instruction::Lda { exp } => {
                    self.cpu_state.psw.a = self.memory.read8(exp);

                    self.cpu_state.pc     +=  3;
                    self.interrupt_timers += 13;
                }
                Instruction::Shld { exp } => {
                    self.memory.write16(exp, self.cpu_state.hl.get());

                    self.cpu_state.pc     +=  3;
                    self.interrupt_timers += 16;
                }
                Instruction::Lhld { exp } => {
                    self.cpu_state.hl.set(self.memory.read16(exp));

                    self.cpu_state.pc     +=  3;
                    self.interrupt_timers += 16;
                }
                Instruction::Pchl => {
                    self.cpu_state.pc.set(self.cpu_state.hl.get());
                    self.interrupt_timers += 5;
                }
                Instruction::Jmp { exp } => {
                    self.cpu_state.pc.set(exp);
                    self.interrupt_timers += 10;
                }
                Instruction::Jc { exp } => {
                    if self.cpu_state.psw.is_carry_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Jnc { exp } => {
                    if !self.cpu_state.psw.is_carry_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Jz { exp } => {
                    if self.cpu_state.psw.is_zero_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Jnz { exp } => {
                    if !self.cpu_state.psw.is_zero_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Jm { exp } => {
                    if self.cpu_state.psw.is_sign_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Jp { exp } => {
                    if !self.cpu_state.psw.is_sign_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Jpe { exp } => {
                    if self.cpu_state.psw.is_parity_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Jpo { exp } => {
                    if !self.cpu_state.psw.is_parity_set() {
                        self.cpu_state.pc.set(exp);
                    } else {
                        self.cpu_state.pc += 3;
                    }
                    self.interrupt_timers += 10;
                }
                Instruction::Call { sub } => {
                    self.cpu_state.pc += 3;
                    self.cpu_state.sp -= 2;

                    self.write_sp(self.cpu_state.pc.get());
                    self.cpu_state.pc.set(sub);

                    self.interrupt_timers += 17;
                }
                Instruction::Cc { sub } => {
                    self.cpu_state.pc += 3;

                    if self.cpu_state.psw.is_carry_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Cnc { sub } => {
                    self.cpu_state.pc += 3;

                    if !self.cpu_state.psw.is_carry_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Cz { sub } => {
                    self.cpu_state.pc += 3;

                    if self.cpu_state.psw.is_zero_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Cnz { sub } => {
                    self.cpu_state.pc += 3;

                    if !self.cpu_state.psw.is_zero_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Cm { sub } => {
                    self.cpu_state.pc += 3;

                    if self.cpu_state.psw.is_sign_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Cp { sub } => {
                    self.cpu_state.pc += 3;

                    if !self.cpu_state.psw.is_sign_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Cpe { sub } => {
                    self.cpu_state.pc += 3;

                    if self.cpu_state.psw.is_parity_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Cpo { sub } => {
                    self.cpu_state.pc += 3;

                    if !self.cpu_state.psw.is_parity_set() {
                        self.cpu_state.sp -= 2;

                        self.write_sp(self.cpu_state.pc.get());
                        self.cpu_state.pc.set(sub);

                        self.interrupt_timers += 17;
                    } else {
                        self.interrupt_timers += 11;
                    }
                }
                Instruction::Ret => {
                    self.cpu_state.pc.set(self.read_sp());

                    self.cpu_state.sp     +=  2;
                    self.interrupt_timers += 10;
                }
                Instruction::Rc => {
                    if self.cpu_state.psw.is_carry_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rnc => {
                    if !self.cpu_state.psw.is_carry_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rz => {
                    if self.cpu_state.psw.is_zero_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rnz => {
                    if !self.cpu_state.psw.is_zero_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rm  => {
                    if self.cpu_state.psw.is_sign_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rp => {
                    if !self.cpu_state.psw.is_sign_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rpe => {
                    if self.cpu_state.psw.is_parity_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rpo => {
                    if !self.cpu_state.psw.is_parity_set() {
                        self.cpu_state.pc.set(self.read_sp());

                        self.cpu_state.sp     +=  2;
                        self.interrupt_timers += 11;
                    } else {
                        self.cpu_state.pc     += 1;
                        self.interrupt_timers += 5;
                    }
                }
                Instruction::Rst { exp } => {
                    self.cpu_state.sp -= 2;

                    self.write_sp(self.cpu_state.pc.get());

                    self.cpu_state.pc.set(u16::from(exp * 8));
                    self.interrupt_timers += 11;
                }
                Instruction::Ei => {
                    self.cpu_state.inte = true;

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::Di => {
                    self.cpu_state.inte = false;

                    self.cpu_state.pc     += 1;
                    self.interrupt_timers += 4;
                }
                Instruction::In { exp } => {
                    self.cpu_state.psw.a = self.io_ports.read(exp);

                    self.cpu_state.pc     +=  2;
                    self.interrupt_timers += 10;
                }
                Instruction::Out { exp } => {
                    self.io_ports.write(exp, self.cpu_state.psw.a);

                    self.cpu_state.pc     +=  2;
                    self.interrupt_timers += 10;
                }
                Instruction::Hlt => {
                    return Ok(())
                }
            };
        }
    }
}
