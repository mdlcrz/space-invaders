use std::error::Error;

use crate::instruction::Instruction;

pub fn decode(instruction: &[u8]) -> Result<Instruction, Box<dyn Error>> {
    let opcode = instruction[0];

    let instruction = match opcode {
        0x00 => {
            Instruction::Nop
        }
        0x01 | 0x11 | 0x21 | 0x31 => {
            let rp   = (opcode >> 4) & 0b11;
            let data = u16::from_le_bytes(instruction[1..3].try_into()?);

            Instruction::Lxi { rp, data }
        }
        0x02 | 0x12 => {
            let rp = (opcode >> 4) & 0b1;

            Instruction::Stax { rp }
        }
        0x03 | 0x13 | 0x23 | 0x33 => {
            let rp = (opcode >> 4) & 0b11;

            Instruction::Inx { rp }
        }
        0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 | 0x3c => {
            let reg = (opcode >> 3) & 0b111;

            Instruction::Inr { reg }
        }
        0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => {
            let reg = (opcode >> 3) & 0b111;

            Instruction::Dcr { reg }
        }
        0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e => {
            let reg  = (opcode >> 3) & 0b111;
            let data = instruction[1];

            Instruction::Mvi { reg, data }
        }
        0x07 | 0x0f | 0x17 | 0x1f => {
            let op = (opcode >> 3) & 0b11;

            match op {
                0b00 => Instruction::Rlc,
                0b01 => Instruction::Rrc,
                0b10 => Instruction::Ral,
                0b11 => Instruction::Rar,
                _    => unreachable!(),
            }
        }
        0x09 | 0x19 | 0x29 | 0x39 => {
            let rp = (opcode >> 4) & 0b11;

            Instruction::Dad { rp }
        }
        0x0a | 0x1a => {
            let rp = (opcode >> 4) & 0b1;

            Instruction::Ldax { rp }
        }
        0x0b | 0x1b | 0x2b | 0x3b => {
            let rp = (opcode >> 4) & 0b11;

            Instruction::Dcx { rp }
        }
        0x22 | 0x2a | 0x32 | 0x3a => {
            let op  = (opcode >> 3) & 0b11;
            let exp = u16::from_le_bytes(instruction[1..3].try_into()?);

            match op {
                0b00 => Instruction::Shld { exp },
                0b01 => Instruction::Lhld { exp },
                0b10 => Instruction::Sta  { exp },
                0b11 => Instruction::Lda  { exp },
                _    => unreachable!(),
            }
        }
        0x27 => {
            Instruction::Daa
        }
        0x2f => {
            Instruction::Cma
        }
        0x37 | 0x3f => {
            let op = (opcode >> 3) & 0b1;

            match op {
                0b0 => Instruction::Stc,
                0b1 => Instruction::Cmc,
                _   => unreachable!(),
            }
        }
        0x40..=0x75 | 0x77..=0x7f => {
            let dst = (opcode >> 3) & 0b111;
            let src =  opcode       & 0b111;

            Instruction::Mov { dst, src }
        }
        0x76 => {
            Instruction::Hlt
        }
        0x80..=0xbf => {
            let op  = (opcode >> 3) & 0b111;
            let reg =  opcode       & 0b111;

            match op {
                0b000 => Instruction::Add { reg },
                0b001 => Instruction::Adc { reg },
                0b010 => Instruction::Sub { reg },
                0b011 => Instruction::Sbb { reg },
                0b100 => Instruction::Ana { reg },
                0b101 => Instruction::Xra { reg },
                0b110 => Instruction::Ora { reg },
                0b111 => Instruction::Cmp { reg },
                _     => unreachable!(),
            }
        }
        0xc0 | 0xc8 | 0xc9 | 0xd0 | 0xd8 | 0xe0 | 0xe8 | 0xf0 | 0xf8 => {
            let op = (opcode >> 3) & 0b111;

            match op {
                0b000 => Instruction::Rnz,
                0b001 => {
                    match opcode & 0b1 {
                        0b0 => Instruction::Rz,
                        0b1 => Instruction::Ret,
                        _   => unreachable!(),
                    }
                }
                0b010 => Instruction::Rnc,
                0b011 => Instruction::Rc,
                0b100 => Instruction::Rpo,
                0b101 => Instruction::Rpe,
                0b110 => Instruction::Rp,
                0b111 => Instruction::Rm,
                _     => unreachable!(),
            }
        }
        0xc1 | 0xd1 | 0xe1 | 0xf1 => {
            let mut rp = (opcode >> 4) & 0b11;

            if rp == 0b11 { rp += 1; }

            Instruction::Pop { rp }
        }
        0xc2 | 0xc3 | 0xca | 0xd2 | 0xda | 0xe2 | 0xea | 0xf2 | 0xfa => {
            let op  = (opcode >> 3) & 0b111;
            let exp = u16::from_le_bytes(instruction[1..3].try_into()?);

            match op {
                0b000 => {
                    match opcode & 0b1 {
                        0b0 => Instruction::Jnz { exp },
                        0b1 => Instruction::Jmp { exp },
                        _   => unreachable!(),
                    }
                }
                0b001 => Instruction::Jz  { exp },
                0b010 => Instruction::Jnc { exp },
                0b011 => Instruction::Jc  { exp },
                0b100 => Instruction::Jpo { exp },
                0b101 => Instruction::Jpe { exp },
                0b110 => Instruction::Jp  { exp },
                0b111 => Instruction::Jm  { exp },
                _     => unreachable!(),
            }
        }
        0xc4 | 0xcc | 0xcd | 0xd4 | 0xdc | 0xe4 | 0xec | 0xf4 | 0xfc => {
            let op  = (opcode >> 3) & 0b111;
            let sub = u16::from_le_bytes(instruction[1..3].try_into()?);

            match op {
                0b000 => Instruction::Cnz { sub },
                0b001 => {
                    match opcode & 0b1 {
                        0b0 => Instruction::Cz   { sub },
                        0b1 => Instruction::Call { sub },
                        _   => unreachable!(),
                    }
                }
                0b010 => Instruction::Cnc { sub },
                0b011 => Instruction::Cc  { sub },
                0b100 => Instruction::Cpo { sub },
                0b101 => Instruction::Cpe { sub },
                0b110 => Instruction::Cp  { sub },
                0b111 => Instruction::Cm  { sub },
                _     => unreachable!(),
            }
        }
        0xc5 | 0xd5 | 0xe5 | 0xf5 => {
            let mut rp = (opcode >> 4) & 0b11;

            if rp == 0b11 { rp += 1; }

            Instruction::Push { rp }
        }
        0xc6 | 0xce | 0xd6 | 0xde | 0xe6 | 0xee | 0xf6 | 0xfe => {
            let op   = (opcode >> 3) & 0b111;
            let data = instruction[1];

            match op {
                0b000 => Instruction::Adi { data },
                0b001 => Instruction::Aci { data },
                0b010 => Instruction::Sui { data },
                0b011 => Instruction::Sbi { data },
                0b100 => Instruction::Ani { data },
                0b101 => Instruction::Xri { data },
                0b110 => Instruction::Ori { data },
                0b111 => Instruction::Cpi { data },
                _     => unreachable!(),
            }
        }
        0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => {
            let exp = (opcode >> 3) & 0b111;

            Instruction::Rst { exp }
        }
        0xd3 | 0xdb => {
            let op  = (opcode >> 3) & 0b1;
            let exp = instruction[1];

            match op {
                0b0 => Instruction::Out { exp },
                0b1 => Instruction::In  { exp },
                _   => unreachable!(),
            }
        }
        0xe3 => {
            Instruction::Xthl
        }
        0xe9 => {
            Instruction::Pchl
        }
        0xeb => {
            Instruction::Xchg
        }
        0xf3 | 0xfb => {
            let op = (opcode >> 3) & 0b1;

            match op {
                0b0 => Instruction::Di,
                0b1 => Instruction::Ei,
                _   => unreachable!(),
            }
        }
        0xf9 => {
            Instruction::Sphl
        }
        _ => panic!("invalid opcode: {:02x}", opcode),
    };
    Ok(instruction)
}
