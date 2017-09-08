


#[derive(Debug, Clone, Copy)]
pub enum Register {
    A,
    X,
    Y,
    D,
    S,
    DB,
    PB,
    P,
    PC,
}


pub struct Registers {
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub d: u16,
    pub sp: u16,
    pub dp: u8,
    pub pb: u8,
    pub p: u8,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            d: 0,
            sp: 0,
            dp: 0,
            pb: 0,
            p: 0,
            pc: 0,
        }
    }
}
