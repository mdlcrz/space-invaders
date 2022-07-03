use crate::pointer_register::PointerRegister;
use crate::program_state_word::ProgramStateWord;
use crate::register_pair::RegisterPair;

pub struct CpuState {
    pub bc:   RegisterPair,
    pub de:   RegisterPair,
    pub hl:   RegisterPair,
    pub psw:  ProgramStateWord,
    pub pc:   PointerRegister,
    pub sp:   PointerRegister,
    pub inte: bool,
}

impl CpuState {
	pub fn new() -> CpuState {
	    CpuState {
            bc:   RegisterPair::new(),
            de:   RegisterPair::new(),
            hl:   RegisterPair::new(),
            psw:  ProgramStateWord::new(),
            pc:   PointerRegister::new(),
            sp:   PointerRegister::new(),
            inte: false,
        }
    }
}
