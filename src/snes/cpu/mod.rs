// https://en.wikibooks.org/wiki/Super_NES_Programming/65c816_reference
// https://github.com/michielvoo/SNES/wiki/CPU
// https://wiki.superfamicom.org/snes/show/65816+Reference

mod registers;
use self::registers::Registers;

pub struct CPU {
    regs: registers::Registers,
}

impl CPU {
    pub fn new () -> CPU {
        println!("I'm the most greatest S-CPU ever!");
        CPU {
            regs: Registers::new(),
        }
    }

    pub fn get_a   (&self) -> u16 { self.regs.a }
    pub fn get_x   (&self) -> u16 { self.regs.x }
    pub fn get_y   (&self) -> u16 { self.regs.y }
    pub fn get_sp  (&self) -> u8  { self.regs.sp }
    pub fn get_db  (&self) -> u16 { self.regs.db }
    pub fn get_pb  (&self) -> u8  { self.regs.pb }
    pub fn get_p   (&self) -> u8  { self.regs.p }
    pub fn get_pc  (&self) -> u16 { self.regs.pc }
}
