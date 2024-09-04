use std::time::Instant;

use glium::{Display, Texture2d};
use glium::glutin::surface::WindowSurface;
use glium::uniforms::UniformBuffer;
use winit::dpi::PhysicalPosition;
use winit::event::Event::WindowEvent;
use winit::event::KeyEvent;
use winit::window::CursorGrabMode;
use crate::primitives::lights_block::LightsBlock;
use crate::systems::game::{Game};
use crate::systems::renderer::Renderer;

pub struct EventHandler {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}

impl EventHandler {
    pub fn new() -> (Self, Display<WindowSurface>) {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("Event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Gust")
            .build(&event_loop);

        (EventHandler { event_loop, window }, display)
    }

    pub fn run(self, game: &mut Game, renderer: Renderer, buffer: UniformBuffer<LightsBlock>) {
        let textures: Vec<_> = game.objects.iter().map(|object| object.get_texture(&renderer.display)).collect();

        self.window.set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_e| self.window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();

        self.window.set_cursor_visible(false);

        let mut mouse_position = PhysicalPosition::new(400.0, 240.0);

        self.event_loop.run(move |event, window_target| {
            match event {
                WindowEvent { event: window_event, .. } => match window_event {
                    winit::event::WindowEvent::KeyboardInput { event: KeyEvent { logical_key: key, state, .. }, .. } => {
                        match state {
                            winit::event::ElementState::Pressed => {
                                game.game_input.handle_keyboard_input(key);
                            }
                            winit::event::ElementState::Released => {
                                game.game_input.handle_key_release(key);
                            }
                        }
                    }
                    winit::event::WindowEvent::CursorMoved { position: new_position, .. } => {
                        mouse_position = new_position;
                        self.window.set_cursor_position(PhysicalPosition::new(400.0, 240.0)).unwrap();
                        game.game_input.handle_mouse_input(mouse_position);
                        mouse_position = PhysicalPosition::new(400.0, 240.0);
                    }
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::Resized(window_size) => {
                        renderer.display.resize(window_size.into());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        EventHandler::handle_redraw_request(mouse_position, game, &textures, &renderer, &buffer);
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    self.window.request_redraw();
                }
                _ => (),
            };
        }).unwrap();
    }

    fn handle_redraw_request(mouse_position: PhysicalPosition<f64>, game: &mut Game, textures: &Vec<Texture2d>, renderer: &Renderer, buffer: &UniformBuffer<LightsBlock>) {
        let now = Instant::now();
        let elapsed = now.duration_since(game.last_frame_time);

        game.last_frame_time = now;
        game.dt = elapsed.as_secs_f32();
        game.t = game.t + game.dt;

        game.game_input.handle_mouse_input(mouse_position);
        game.update();
        renderer.render(game, textures, buffer);
    }
}
