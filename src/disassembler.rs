use std::fmt::Write;
use std::error::Error;

use crate::decoder;
use crate::instruction::Instruction;

fn print_opcodes(instruction: &[u8], instruction_length: usize) {
    let mut opcodes = String::new();

    for byte in instruction.iter().take(instruction_length) {
        let _ = write!(opcodes, "{:02x} ", byte);
    }

    print!("{: <10}", opcodes)
}

fn register(reg: u8) -> &'static str {
    match reg {
        0b000 => "B",
        0b001 => "C",
        0b010 => "D",
        0b011 => "E",
        0b100 => "H",
        0b101 => "L",
        0b110 => "M",
        0b111 => "A",
        _     => unreachable!(),
    }
}

fn register_pair(rp: u8) -> &'static str {
    match rp {
        0b000 => "B",
        0b001 => "D",
        0b010 => "H",
        0b011 => "SP",
        0b100 => "PSW",
        _     => unreachable!(),
    }
}

pub fn disassemble(program_counter: u16, instruction: &[u8])
        -> Result<Instruction, Box<dyn Error>> {
    let decoded_instruction = decoder::decode(instruction)?;

    print!("{:04x}  ", program_counter);

    match decoded_instruction {
        Instruction::Stc => {
            print_opcodes(instruction, 1);
            println!("STC");
        }
        Instruction::Cmc => {
            print_opcodes(instruction, 1);
            println!("CMC");
        }
        Instruction::Inr { reg } => {
            print_opcodes(instruction, 1);
            println!("INR  {}", register(reg));
        }
        Instruction::Dcr { reg } => {
            print_opcodes(instruction, 1);
            println!("DCR  {}", register(reg));
        }
        Instruction::Cma => {
            print_opcodes(instruction, 1);
            println!("CMA");
        }
        Instruction::Daa => {
            print_opcodes(instruction, 1);
            println!("DAA");
        }
        Instruction::Nop => {
            print_opcodes(instruction, 1);
            println!("NOP");
        }
        Instruction::Mov { dst, src } => {
            print_opcodes(instruction, 1);
            println!("MOV  {}, {}", register(dst), register(src));
        }
        Instruction::Stax { rp } => {
            print_opcodes(instruction, 1);
            println!("STAX {}", register_pair(rp));
        }
        Instruction::Ldax { rp } => {
            print_opcodes(instruction, 1);
            println!("LDAX {}", register_pair(rp));
        }
        Instruction::Add { reg } => {
            print_opcodes(instruction, 1);
            println!("ADD  {}", register(reg));
        }
        Instruction::Adc { reg } => {
            print_opcodes(instruction, 1);
            println!("ADC  {}", register(reg));
        }
        Instruction::Sub { reg } => {
            print_opcodes(instruction, 1);
            println!("SUB  {}", register(reg));
        }
        Instruction::Sbb { reg } => {
            print_opcodes(instruction, 1);
            println!("SBB  {}", register(reg));
        }
        Instruction::Ana { reg } => {
            print_opcodes(instruction, 1);
            println!("ANA  {}", register(reg));
        }
        Instruction::Xra { reg } => {
            print_opcodes(instruction, 1);
            println!("XRA  {}", register(reg));
        }
        Instruction::Ora { reg } => {
            print_opcodes(instruction, 1);
            println!("ORA  {}", register(reg));
        }
        Instruction::Cmp { reg } => {
            print_opcodes(instruction, 1);
            println!("CMP  {}", register(reg));
        }
        Instruction::Rlc => {
            print_opcodes(instruction, 1);
            println!("RLC");
        }
        Instruction::Rrc => {
            print_opcodes(instruction, 1);
            println!("RRC");
        }
        Instruction::Ral => {
            print_opcodes(instruction, 1);
            println!("RAL");
        }
        Instruction::Rar => {
            print_opcodes(instruction, 1);
            println!("RAR");
        }
        Instruction::Push { rp } => {
            print_opcodes(instruction, 1);
            println!("PUSH {}", register_pair(rp));
        }
        Instruction::Pop { rp } => {
            print_opcodes(instruction, 1);
            println!("POP  {}", register_pair(rp));
        }
        Instruction::Dad { rp } => {
            print_opcodes(instruction, 1);
            println!("DAD  {}", register_pair(rp));
        }
        Instruction::Inx { rp } => {
            print_opcodes(instruction, 1);
            println!("INX  {}", register_pair(rp));
        }
        Instruction::Dcx { rp } => {
            print_opcodes(instruction, 1);
            println!("DCX  {}", register_pair(rp));
        }
        Instruction::Xchg => {
            print_opcodes(instruction, 1);
            println!("XCHG");
        }
        Instruction::Xthl => {
            print_opcodes(instruction, 1);
            println!("XTHL");
        }
        Instruction::Sphl => {
            print_opcodes(instruction, 1);
            println!("SPHL");
        }
        Instruction::Lxi { rp, data } => {
            print_opcodes(instruction, 3);
            println!("LXI  {}, {:#x}", register_pair(rp), data);
        }
        Instruction::Mvi { reg, data } => {
            print_opcodes(instruction, 2);
            println!("MVI  {}, {:#x}", register(reg), data);
        }
        Instruction::Adi { data } => {
            print_opcodes(instruction, 2);
            println!("ADI  {:#x}", data);
        }
        Instruction::Aci { data } => {
            print_opcodes(instruction, 2);
            println!("ACI  {:#x}", data);
        }
        Instruction::Sui { data } => {
            print_opcodes(instruction, 2);
            println!("SUI  {:#x}", data);
        }
        Instruction::Sbi { data } => {
            print_opcodes(instruction, 2);
            println!("SBI  {:#x}", data);
        }
        Instruction::Ani { data } => {
            print_opcodes(instruction, 2);
            println!("ANI  {:#x}", data);
        }
        Instruction::Xri { data } => {
            print_opcodes(instruction, 2);
            println!("XRI  {:#x}", data);
        }
        Instruction::Ori { data } => {
            print_opcodes(instruction, 2);
            println!("ORI  {:#x}", data);
        }
        Instruction::Cpi { data } => {
            print_opcodes(instruction, 2);
            println!("CPI  {:#x}", data);
        }
        Instruction::Sta { exp } => {
            print_opcodes(instruction, 3);
            println!("STA  {:#x}", exp);
        }
        Instruction::Lda { exp } => {
            print_opcodes(instruction, 3);
            println!("LDA  {:#x}", exp);
        }
        Instruction::Shld { exp } => {
            print_opcodes(instruction, 3);
            println!("SHLD {:#x}", exp);
        }
        Instruction::Lhld { exp } => {
            print_opcodes(instruction, 3);
            println!("LHLD {:#x}", exp);
        }
        Instruction::Pchl => {
            print_opcodes(instruction, 1);
            println!("PCHL");
        }
        Instruction::Jmp { exp } => {
            print_opcodes(instruction, 3);
            println!("JMP  {:#x}", exp);
        }
        Instruction::Jc { exp } => {
            print_opcodes(instruction, 3);
            println!("JC   {:#x}", exp);
        }
        Instruction::Jnc { exp } => {
            print_opcodes(instruction, 3);
            println!("JNC  {:#x}", exp);
        }
        Instruction::Jz { exp } => {
            print_opcodes(instruction, 3);
            println!("JZ   {:#x}", exp);
        }
        Instruction::Jnz { exp } => {
            print_opcodes(instruction, 3);
            println!("JNZ  {:#x}", exp);
        }
        Instruction::Jm { exp } => {
            print_opcodes(instruction, 3);
            println!("JM   {:#x}", exp);
        }
        Instruction::Jp { exp } => {
            print_opcodes(instruction, 3);
            println!("JP   {:#x}", exp);
        }
        Instruction::Jpe { exp } => {
            print_opcodes(instruction, 3);
            println!("JPE  {:#x}", exp);
        }
        Instruction::Jpo { exp } => {
            print_opcodes(instruction, 3);
            println!("JPO  {:#x}", exp);
        }
        Instruction::Call { sub } => {
            print_opcodes(instruction, 3);
            println!("CALL {:#x}", sub);
        }
        Instruction::Cc { sub } => {
            print_opcodes(instruction, 3);
            println!("CC   {:#x}", sub);
        }
        Instruction::Cnc { sub } => {
            print_opcodes(instruction, 3);
            println!("CNC  {:#x}", sub);
        }
        Instruction::Cz { sub } => {
            print_opcodes(instruction, 3);
            println!("CZ   {:#x}", sub);
        }
        Instruction::Cnz { sub } => {
            print_opcodes(instruction, 3);
            println!("CNZ  {:#x}", sub);
        }
        Instruction::Cm { sub } => {
            print_opcodes(instruction, 3);
            println!("CM   {:#x}", sub);
        }
        Instruction::Cp { sub } => {
            print_opcodes(instruction, 3);
            println!("CP   {:#x}", sub);
        }
        Instruction::Cpe { sub } => {
            print_opcodes(instruction, 3);
            println!("CPE  {:#x}", sub);
        }
        Instruction::Cpo { sub } => {
            print_opcodes(instruction, 3);
            println!("CPO  {:#x}", sub);
        }
        Instruction::Ret => {
            print_opcodes(instruction, 1);
            println!("RET");
        }
        Instruction::Rc => {
            print_opcodes(instruction, 1);
            println!("RC");
        }
        Instruction::Rnc => {
            print_opcodes(instruction, 1);
            println!("RNC");
        }
        Instruction::Rz => {
            print_opcodes(instruction, 1);
            println!("RZ");
        }
        Instruction::Rnz => {
            print_opcodes(instruction, 1);
            println!("RNZ");
        }
        Instruction::Rm => {
            print_opcodes(instruction, 1);
            println!("RM");
        }
        Instruction::Rp => {
            print_opcodes(instruction, 1);
            println!("RP");
        }
        Instruction::Rpe => {
            print_opcodes(instruction, 1);
            println!("RPE");
        }
        Instruction::Rpo => {
            print_opcodes(instruction, 1);
            println!("RPO");
        }
        Instruction::Rst { exp } => {
            print_opcodes(instruction, 1);
            println!("RST  {}", exp);
        }
        Instruction::Ei => {
            print_opcodes(instruction, 1);
            println!("EI");
        }
        Instruction::Di => {
            print_opcodes(instruction, 1);
            println!("DI");
        }
        Instruction::In  { exp } => {
            print_opcodes(instruction, 2);
            println!("IN   {:#x}", exp);
        }
        Instruction::Out { exp } => {
            print_opcodes(instruction, 2);
            println!("OUT  {:#x}", exp);
        }
        Instruction::Hlt => {
            print_opcodes(instruction, 1);
            println!("HLT");
        }
    };
    Ok(decoded_instruction)
}
