
pub enum Register {
    A, X, Y, SP, DB, PB, P
}


pub struct Registers {
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub sp: u8,
    pub db: u16,
    pub pb: u8,
    pub p: u8,
    pub pc: u16, 
}

impl Registers {
    pub fn new () -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            db: 0,
            pb: 0,
            p: 0,
            pc: 0,
        }
    }
}
