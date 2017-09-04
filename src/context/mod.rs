extern crate sdl2;
extern crate glium;
extern crate glium_sdl2;
extern crate imgui_glium_renderer;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::glium::{Api, GliumCreationError, SwapBuffersError, Version, Surface};
use self::glium::backend::{Facade};
use self::glium_sdl2::{DisplayBuild, SDL2Facade};

use imgui::ImGui;

use std::process;

pub struct Context {
    context: sdl2::Sdl,
    display: glium_sdl2::Display,
    imgui: ImGui,
    imgui_renderer: imgui_glium_renderer::Renderer,
}

impl Context {
    pub fn new () -> Context {
        use self::imgui_glium_renderer::Renderer;
        
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();

        let display = match video_ctx
            .window("I'm a friendly Snes thing window!", 512, 448)
            .position_centered()
            .opengl()
            .build_glium()
        {
            Ok(display) => display,
            Err(err)   => panic!("Failed to create window: {}", err)
        };
        
        let mut imgui = ImGui::init();
        let mut imgui_renderer = Renderer::init(&mut imgui, &display).unwrap();
        Context {
            context: ctx,
            display: display,
            imgui: imgui,
            imgui_renderer: imgui_renderer,
        }
    }
    
    pub fn handle_events (&self) {
        let mut events = self.context.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit    {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                _ => {}
            }
        }        
    }

    pub fn render_imgui (&mut self) {

    }
    

    pub fn render_frame (&mut self) {
        let ui = self.imgui.frame((512, 448), (512, 448), 0.0016);
        let mut target = self.display.draw();
        target.clear_color(0.01, 0.01, 0.01, 1.0);

        use imgui::*;
        ui.window(im_str!("Hello world"))
            .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("This...is...imgui-rs!")); 
            });

        
        self.imgui_renderer.render(&mut target, ui).expect("Rendering failed");
        target.finish().unwrap();
    }

    pub fn set_title (&mut self, title: String) {
        let window = self.display.window_mut();
        window.set_title(title.as_str()).unwrap();
    }
}

