#![feature(bigint_helper_methods)]

use std::error::Error;
use emulator::Emulator;

mod cpu_state;
mod decoder;
mod disassembler;
mod emulator;
mod flag;
mod instruction;
mod interrupt_timer;
mod memory;
mod pointer_register;
mod io_ports;
mod program_state_word;
mod register_pair;

fn main() -> Result<(), Box<dyn Error>> {
    let mut emulator = Emulator::new();

    emulator.load_rom("./rom/space_invaders")?;
    emulator.run()?;

    Ok(())
}
