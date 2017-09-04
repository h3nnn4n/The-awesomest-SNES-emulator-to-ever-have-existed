extern crate imgui;

use std::time;
use std::time::Instant;
use std::thread;

mod frontend;
use frontend::Frontend;
mod snes;
use snes::Snes;

fn main() {
    let snes = Snes::new();
    let mut frontend = Frontend::new();

    let mut delta_time = 16.666667;
    
    // App's master loop
    loop {
        let start_instant = Instant::now();

        frontend.handle_events();
        frontend.render_frame(show_gui_function, delta_time);        

        // Calculate elapsed time this frame
        let elapsed = start_instant.elapsed(); 
        delta_time = (elapsed.as_secs() * 1000) as f32 + (elapsed.subsec_nanos() as f32 / 1_000_000.0);

        let new_title = format!("SNES thing | Δt: {:.6} ms", delta_time);
        frontend.set_title(new_title);
        
        let expected_dt = 16.666667;
        if expected_dt >= delta_time { 
            let sleep_duration = time::Duration::from_millis(expected_dt as u64 - delta_time as u64);
            thread::sleep(sleep_duration);   
        }
    }
}

fn show_gui_function<'a> (ui: &imgui::Ui<'a>) -> bool {
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

fn show_cpu_window<'a> (ui: &imgui::Ui<'a>, snes: &Snes) -> bool {
    use imgui::*;
    ui.window(im_str!("CPU Status"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("A:"));
            ui.separator();            
        });

    true
}
