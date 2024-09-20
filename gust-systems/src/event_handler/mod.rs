use std::time::Instant;

use glium::Display;
use glium::glutin::surface::WindowSurface;
use glium::uniforms::UniformBuffer;
use winit::dpi::PhysicalPosition;
use winit::event::Event::WindowEvent;
use winit::event::KeyEvent;
use winit::window::{CursorGrabMode, Window};

use gust_core::primitives::lights_block::LightsBlock;

use crate::systems::game::Game;
use crate::systems::render_system::RenderSystem;

pub struct EventHandler {
    event_loop: winit::event_loop::EventLoop<()>,
    pub window: Window,
}

impl EventHandler {
    pub fn new() -> (Self, Display<WindowSurface>) {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("Event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Gust")
            .build(&event_loop);

        (EventHandler {
            event_loop,
            window,
        }, display)
    }

    pub fn run(self, game: &mut Game, render_system: RenderSystem) {
        self.initialize_window();

        let buffer = self.construct_light_block(&render_system.display);

        let mut mouse_position = PhysicalPosition::new(400.0, 240.0);

        self.event_loop.run(move |event, window_target| {
            match event {
                WindowEvent { event: window_event, .. } => match window_event {
                    winit::event::WindowEvent::KeyboardInput { event: KeyEvent { logical_key: key, state, .. }, .. } => {
                        match state {
                            winit::event::ElementState::Pressed => {
                                game.input_handler.handle_keyboard_input(key);
                            }
                            winit::event::ElementState::Released => {
                                game.input_handler.handle_key_release(key);
                            }
                        }
                    }
                    winit::event::WindowEvent::CursorMoved { position: new_position, .. } => {
                        mouse_position = new_position;
                        self.window.set_cursor_position(PhysicalPosition::new(400.0, 240.0)).unwrap();
                        game.input_handler.handle_mouse_input(mouse_position);
                        mouse_position = PhysicalPosition::new(400.0, 240.0);
                    }
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::Resized(window_size) => {
                        render_system.display.resize(window_size.into());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        EventHandler::handle_redraw_request(mouse_position, game, &render_system, &buffer);
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

    fn construct_light_block(&self, display: &Display<WindowSurface>) -> UniformBuffer<LightsBlock> {
        let light_positions: [[f32; 3]; 5] = [
            [0.0, 0.0, 5.0],
            [10.0, 10.0, 5.0],
            [-10.0, 10.0, 5.0],
            [10.0, -10.0, 5.0],
            [-10.0, -10.0, 5.0],
        ];

        let light_colors: [[f32; 3]; 5] = [
            [1.0, 1.0, 1.0]; 5
        ];

        UniformBuffer::new(display, LightsBlock {
            light_positions,
            _padding: [0.0; 5],
            light_colors,
        }).unwrap()
    }

    fn initialize_window(&self) {
        self.window.set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_e| self.window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();

        self.window.set_cursor_visible(false);
    }

    fn handle_redraw_request(mouse_position: PhysicalPosition<f64>, game: &mut Game, render_system: &RenderSystem, buffer: &UniformBuffer<LightsBlock>) {
        let now = Instant::now();
        let elapsed = now.duration_since(game.last_frame_time);

        game.last_frame_time = now;
        game.dt = elapsed.as_secs_f32();
        game.t = game.t + game.dt;

        game.input_handler.handle_mouse_input(mouse_position);
        game.update();
        render_system.render(game, buffer);
    }
}
