extern crate imgui;

use std::time;
use std::time::Instant;
use std::thread;

mod frontend;
use frontend::Frontend;
mod snes;
use snes::Snes;
use snes::cpu::instructions::*;

mod cartridge;
use cartridge::Cartridge;
use cartridge::DataSize;


fn main() {
    let mut app = App::new();
    app.run();
}

struct App {
    snes: Snes,
    frontend: Frontend,
}

impl App {
    pub fn new() -> App {
        App {
            snes: Snes::new(),
            frontend: Frontend::new(),
        }
    }

    pub fn run(&mut self) {

        let cart = Cartridge::new("./target/roms/Super Mario World (U) [!].smc");
        println!("Cart size: {:.2} MB", cart.get_total_size().as_mb());
        println!("Type: {}", cart.get_type());
        println!("SMC header size: {} B", cart.get_smc_header_size());

        let mut delta_time = 16.666667;
        let code61: [u8; 4] = [0x61, 0xFF, 0xEE, 0xDD];
        let code63: [u8; 4] = [0x63, 0xFF, 0xEE, 0xDD];
        let code65: [u8; 4] = [0x65, 0xFF, 0xEE, 0xDD];
        let code67: [u8; 4] = [0x67, 0xFF, 0xEE, 0xDD];
        let code69: [u8; 4] = [0x69, 0xFF, 0xEE, 0xDD];
        let code6D: [u8; 4] = [0x6D, 0xFF, 0xEE, 0xDD];
        let code6F: [u8; 4] = [0x6F, 0xFF, 0xEE, 0xDD];
        let code71: [u8; 4] = [0x71, 0xFF, 0xEE, 0xDD];
        let code72: [u8; 4] = [0x72, 0xFF, 0xEE, 0xDD];
        let code73: [u8; 4] = [0x73, 0xFF, 0xEE, 0xDD];
        let code75: [u8; 4] = [0x75, 0xFF, 0xEE, 0xDD];
        let code77: [u8; 4] = [0x77, 0xFF, 0xEE, 0xDD];
        let code79: [u8; 4] = [0x79, 0xFF, 0xEE, 0xDD];
        let code7D: [u8; 4] = [0x7D, 0xFF, 0xEE, 0xDD];
        let code7F: [u8; 4] = [0x7F, 0xFF, 0xEE, 0xDD];

        println!("{}", disassemble(&code61));
        println!("{}", disassemble(&code63));
        println!("{}", disassemble(&code65));
        println!("{}", disassemble(&code67));
        println!("{}", disassemble(&code69));
        println!("{}", disassemble(&code6D));
        println!("{}", disassemble(&code6F));
        println!("{}", disassemble(&code71));
        println!("{}", disassemble(&code72));
        println!("{}", disassemble(&code73));
        println!("{}", disassemble(&code75));
        println!("{}", disassemble(&code77));
        println!("{}", disassemble(&code79));
        println!("{}", disassemble(&code7D));
        println!("{}", disassemble(&code7F));

        // App's master loop
        loop {
            let start_instant = Instant::now();

            self.frontend.handle_events();

            let &mut App { ref snes, .. } = self;
            self.frontend.render_frame(
                |ui| {
                    show_gui_function(ui);
                    show_cpu_window(ui, &snes);
                    true
                },
                delta_time,
            );

            // Calculate elapsed time this frame
            let elapsed = start_instant.elapsed();
            delta_time = (elapsed.as_secs() * 1000) as f32 +
                (elapsed.subsec_nanos() as f32 / 1_000_000.0);

            let new_title = format!("SNES thing | Δt: {:.6} ms", delta_time);
            self.frontend.set_title(new_title);

            let expected_dt = 16.666667;
            if expected_dt >= delta_time {
                let sleep_duration =
                    time::Duration::from_millis(expected_dt as u64 - delta_time as u64);
                thread::sleep(sleep_duration);
            }
        }
    }
}

fn show_gui_function<'a>(ui: &imgui::Ui<'a>) -> bool {
    use imgui::*;
    ui.window(im_str!("Hello SNES thing :D"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("This...is...gracias Gekkio!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
        });


    true
}

fn show_cpu_window<'a>(ui: &imgui::Ui<'a>, snes: &Snes) -> bool {
    use imgui::*;
    let cpu = snes.get_cpu();
    ui.window(im_str!("CPU Status"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("A: 0x{:04X}", cpu.get_a()));
            ui.same_line_spacing(0.0, 15.0);
            ui.text(im_str!("P: 0x{:04X}", cpu.get_p()));

            ui.text(im_str!("X: 0x{:04X}", cpu.get_x()));
            ui.same_line_spacing(0.0, 15.0);
            ui.text(im_str!("Y: 0x{:04X}", cpu.get_y()));
            ui.spacing();

            ui.text(im_str!("S: 0x{:04X}", cpu.get_s()));
            ui.same_line(0.0);
            ui.text(im_str!("PC: 0x{:04X}", cpu.get_pc()));

            ui.separator();
        });

    true
}
