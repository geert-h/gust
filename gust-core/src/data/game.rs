use glium::{Surface, Texture2d, uniform};
use glium::uniforms::Uniforms;
use glium::VertexBuffer;
use winit::dpi::PhysicalPosition;
use winit::event::Event::WindowEvent;
use winit::event::KeyEvent;
use winit::window::CursorGrabMode;

use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;

use crate::data::camera::Camera;
use crate::data::game_input::GameInput;
use crate::data::game_object::GameObject;
use crate::data::player::Player;
use crate::data::vertex::Vertex;

pub struct Game {
    t: f32,
    object: GameObject,
    pub player: Player,
    pub game_input: GameInput,
    pub camera: Camera,
}

impl Game {
    pub fn new() -> Self {
        let object = GameObject::init();
        Game {
            t: 0.0,
            player: Player::init(),
            game_input: GameInput::new(),
            object,
            camera: Camera::init(),
        }
    }

    pub fn update(&mut self) {
        if self.game_input.keyboard_input.is_character_pressed('r') {
            self.player = Player::init();
        }
        self.player.update(&self.game_input);
    }

    pub fn run(&mut self) {
        let vert_shader_string = include_str!("../../../resources/shaders/vert.glsl");
        let frac_shader_string = include_str!("../../../resources/shaders/frac.glsl");

        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("Event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Gust")
            .build(&event_loop);

        let texture = self.object.get_texture(&display);

        window.set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();

        window.set_cursor_visible(false);

        let flattened_triangles: Vec<Vertex> = self.object.mesh.triangles
            .iter()
            .flat_map(|triangle| triangle
                .iter()
                .cloned()
            )
            .collect();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_buffer = VertexBuffer::new(&display, &flattened_triangles).unwrap();
        let mut mouse_position = PhysicalPosition::new(400.0, 240.0);

        event_loop
            .run(move |event, window_target| {
                match event {
                    WindowEvent { event: window_event, .. } => match window_event {
                        winit::event::WindowEvent::KeyboardInput { event: KeyEvent { logical_key: key, state, .. }, .. } => {
                            match state {
                                winit::event::ElementState::Pressed => {
                                    self.game_input.handle_keyboard_input(key);
                                }
                                winit::event::ElementState::Released => {
                                    self.game_input.handle_key_release(key);
                                }
                            }
                        }
                        winit::event::WindowEvent::CursorMoved { position: new_position, .. } => {
                            mouse_position = new_position;
                            window.set_cursor_position(PhysicalPosition::new(400.0, 240.0)).unwrap();
                            self.game_input.handle_mouse_input(mouse_position);
                            mouse_position = PhysicalPosition::new(400.0, 240.0);
                        }
                        winit::event::WindowEvent::CloseRequested => window_target.exit(),
                        winit::event::WindowEvent::Resized(window_size) => {
                            display.resize(window_size.into());
                        }
                        winit::event::WindowEvent::RedrawRequested => {
                            self.t += 0.02;
                            self.game_input.handle_mouse_input(mouse_position);
                            self.update();

                            let program = glium::Program::from_source(
                                &display,
                                vert_shader_string,
                                frac_shader_string,
                                None,
                            )
                                .unwrap();

                            let params = glium::DrawParameters {
                                depth: glium::Depth {
                                    test: glium::draw_parameters::DepthTest::IfLess,
                                    write: true,
                                    ..Default::default()
                                },
                                backface_culling:
                                glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                                ..Default::default()
                            };

                            let mut target = display.draw();
                            target.clear_color_and_depth((0.3, 0.3, 0.4, 1.0), 1.0);
                            target
                                .draw(
                                    &vertex_buffer,
                                    &indices,
                                    &program,
                                    &self.get_uniforms(self.player.position.to_vec().try_into().unwrap(), self.player.direction.to_vec().try_into().unwrap(), self.t, &texture),
                                    &params,
                                )
                                .unwrap();

                            target.finish().unwrap();
                        }
                        _ => (),
                    },
                    winit::event::Event::AboutToWait => {
                        window.request_redraw();
                    }
                    _ => (),
                };
            })
            .unwrap();
    }

    fn get_uniforms<'a>(&'a self, position: [f32; 3], direction: [f32; 3], _t: f32, texture: &'a Texture2d) -> impl Uniforms + 'a {
        let light = [1.4, 0.4, -0.7f32];

        let view = view_matrix(&position, &direction, &[0.0, 0.0, 1.0]);

        uniform! {
            perspective: self.camera.get_perspective(),
            model: Mat4::identity().to_slices(),
            u_texture: texture,
            u_light: light,
            view : view,
        }
    }
}


fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = Vect3::from_slice(direction).normalize();

    let s = Vect3::from_slice(up).normalize().cross(&f).normalize();

    let u = f.cross(&s).normalize();

    let p = [
        -position[0] * s[0] - position[1] * s[1] - position[2] * s[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    let p = Vect3::from_slice(&p);

    let res = Mat4::from_vects([s.to_vect4(0.0), u.to_vect4(0.0), f.to_vect4(0.0), p.to_vect4(1.0)]).transpose();

    res.to_slices()
}
