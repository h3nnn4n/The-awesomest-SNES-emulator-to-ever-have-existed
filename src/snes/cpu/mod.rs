// https://en.wikibooks.org/wiki/Super_NES_Programming/65c816_reference
// https://github.com/michielvoo/SNES/wiki/CPU
// https://wiki.superfamicom.org/snes/show/65816+Reference

pub struct CPU {
    acc: u16,
    x: u16,
    y: u16,
    sp: u16,
    db: u16,
    pb: u16,
    status: u16,
    pc: u16,    
}

impl CPU {
    pub fn new () -> CPU {
        println!("I'm the most greatest S-CPU ever!");
        CPU {
            acc: 0,
            x: 0,
            y: 0,
            sp: 0x0,
            db: 0,
            pb: 0,
            status: 0,
            pc: 0x0,
        }
    }

    pub fn get_acc (&self) -> u16 { self.pc }
    pub fn get_x   (&self) -> u16 { self.x }
    pub fn get_y   (&self) -> u16 { self.y }
    pub fn get_sp  (&self) -> u16 { self.sp }
    pub fn get_db  (&self) -> u16 { self.db }
    pub fn get_pb  (&self) -> u16 { self.pb }
    pub fn get_s   (&self) -> u16 { self.status }
    pub fn get_pc  (&self) -> u16 { self.pc }
}
