use std::fmt;

// Helpers
#[derive(Clone, Copy)]
pub enum RomType {
    Unknown,
    LoRom,
    HiRom,
    ExtraLoRom,
    ExtraHiRom,
}

impl fmt::Display for RomType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let printable = match *self {
            RomType::LoRom => "LoRom",
            RomType::HiRom => "HiRom",
            RomType::ExtraLoRom => "Extra LoRom",
            RomType::ExtraHiRom => "Extra HiRom", 
            RomType::Unknown => "Unknown", 
        };

        write!(f, "{}", printable)
    }
}

pub trait DataSize {
    fn as_kb(&self) -> f32;
    fn as_mb(&self) -> f32;
}

impl DataSize for usize {
    fn as_kb(&self) -> f32 {
        *self as f32 / 1024 as f32
    }

    fn as_mb(&self) -> f32 {
        *self as f32 / (1024 * 1024) as f32
    }
}

// Cartridge
pub struct Cartridge {
    data: Vec<u8>,
    rom_type: RomType,
}


impl Cartridge {
    pub fn new(file_path: &'static str) -> Cartridge {
        use std::fs::File;
        use std::path::Path;
        use std::error::Error;
        use std::io::Read;

        let path = Path::new(file_path);

        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()), 
        };

        let mut data = Vec::<u8>::new();
        let result = file.read_to_end(&mut data).unwrap();


        let smc_offset = data.len() % 0x400;
        let rom_type = match data[smc_offset + 0x15] {
            0x20 => RomType::LoRom,
            0x21 => RomType::HiRom,
            0x32 => RomType::ExtraLoRom,
            0x35 => RomType::ExtraHiRom,
            _ => RomType::Unknown,
        };


        Cartridge {
            data: data,
            rom_type: rom_type,
        }
    }

    pub fn get_total_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_smc_header_size(&self) -> usize {
        self.get_total_size() % 0x400
    }

    pub fn has_smc_header(&self) -> bool {
        0 < self.get_smc_header_size()
    }

    pub fn get_type(&self) -> RomType {
        self.rom_type
    }
}
