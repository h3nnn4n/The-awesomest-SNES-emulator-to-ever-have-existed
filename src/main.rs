extern crate imgui;

use std::time;
use std::time::Instant;
use std::thread;

mod context;
use context::Context;
mod snes;
use snes::Snes;

fn main() {
    let snes = Snes::new();
    let mut context = Context::new();
    
    // App's master loop
    loop {
        let start_instant = Instant::now();

        context.handle_events();
        context.render_frame();        

        // Calculate elapsed time this frame
        let elapsed = start_instant.elapsed(); 
        let dt = (elapsed.as_secs() * 1000) as f32 + (elapsed.subsec_nanos() as f32 / 1_000_000.0);

        let new_title = format!("SNES thing | Δt: {:.6} ms", dt);
        context.set_title(new_title);
        
        let expected_dt = 16.666667;
        if expected_dt >= dt { 
            let sleep_duration = time::Duration::from_millis(expected_dt as u64 - dt as u64);
            thread::sleep(sleep_duration);   
        }
    }
}
