mod cpu;
mod ppu;

use self::cpu::CPU;
use self::ppu::PPU;

pub struct Snes {
    cpu: CPU,
    ppu: PPU,
}

impl Snes {
    pub fn new () -> Snes {
        println!("I'm the greatest Snes struct ever!");
        Snes{ 
            cpu: CPU::new(),
            ppu: PPU::new(),
        }
    }

    pub fn get_cpu (&self) -> &CPU {
        &self.cpu
    }
}
