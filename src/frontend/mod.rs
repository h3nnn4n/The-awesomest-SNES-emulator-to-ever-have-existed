extern crate sdl2;
extern crate glium;
extern crate glium_sdl2;
extern crate imgui_glium_renderer;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::glium::{Surface};
use self::glium_sdl2::{DisplayBuild};

use imgui;
use imgui::ImGui;

use std::process;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct CursorState {
    position: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct Frontend {
    context: sdl2::Sdl,
    display: glium_sdl2::Display,
    cursor_state: CursorState,
    imgui: ImGui,
    imgui_renderer: imgui_glium_renderer::Renderer,
}

impl Frontend {
    pub fn new () -> Frontend {
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
        let imgui_renderer = Renderer::init(&mut imgui, &display).unwrap();

        Frontend {
            context: ctx,
            display: display,
            cursor_state: CursorState::default(),
            imgui: imgui,
            imgui_renderer: imgui_renderer, 
        }
    }
    
    pub fn handle_events (&mut self) {
        let mut events = self.context.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit    {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::MouseWheel {y, ..} => {
                    self.cursor_state.wheel = y as f32;
                },
                _ => {}
            }
        }

        
        let state = events.mouse_state();
        state.pressed_mouse_buttons();
        self.cursor_state.position = (state.x(), state.y());
        self.cursor_state.pressed = (state.left(), state.middle(), state.right());        
        
    }

    pub fn get_dimensions (&mut self) -> (u32, u32) {
        self.display.draw().get_dimensions()
    }
    
    pub fn render_frame<'a, F: FnMut(&imgui::Ui) -> bool> (&mut self, mut fn_ui: F, dt: f32) {
        let mut target = self.display.draw();
        target.clear_color(0.01, 0.01, 0.01, 1.0);

        // Render GUI
        update_mouse_imgui(&mut self.imgui, &mut self.cursor_state);
        let dimensions = target.get_dimensions();
        let ui = self.imgui.frame(dimensions, dimensions, dt);
        if fn_ui(&ui) {
            self.imgui_renderer.render(&mut target, ui).expect("Rendering failed");
        } 
        
        target.finish().unwrap();
    }

    pub fn set_title (&mut self, title: String) {
        let window = self.display.window_mut();
        window.set_title(title.as_str()).unwrap();
    }
}


fn update_mouse_imgui (imgui: &mut ImGui, cursor_state: &mut CursorState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(
        cursor_state.position.0 as f32 / scale.0,
        cursor_state.position.1 as f32 / scale.1,
    );
    imgui.set_mouse_down(
        &[
            cursor_state.pressed.0,
            cursor_state.pressed.1,
            cursor_state.pressed.2,
            false,
            false,
        ],
    );
    imgui.set_mouse_wheel(cursor_state.wheel / scale.1);
    cursor_state.wheel = 0.0;
}

