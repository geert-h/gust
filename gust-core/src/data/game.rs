use glium::{Surface, Texture2d, uniform};
use glium::texture::RawImage2d;
use glium::uniforms::Uniforms;
use glium::VertexBuffer;
use winit::dpi::PhysicalPosition;
use winit::event::Event::WindowEvent;
use winit::event::KeyEvent;
use winit::keyboard::Key;
use winit::window::CursorGrabMode;

use gust_math::matrices::matrix::Matrix;
use gust_math::vectors::vect::Vect;

use crate::data::camera::Camera;
use crate::data::game_input::GameInput;
use crate::data::mesh::{from_wavefront_object, Mesh};
use crate::data::player::Player;
use crate::data::vertex::Vertex;
use crate::parsers::wavefront_object_parser;

pub struct Game {
    t: f32,
    object: Mesh,
    pub player: Player,
    pub game_input: GameInput,
    pub camera: Camera,
}

impl Game {
    pub fn new() -> Self {
        let wavefront_object = wavefront_object_parser::parse_wavefront_object("C:\\Users\\Geert\\source\\repos\\Personal\\gust\\resources\\assets\\objects\\BalKubus.obj");
        let object = from_wavefront_object(wavefront_object);

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

    pub fn handle_keyboard_input(&mut self, key: Key) {
        self.game_input.handle_keyboard_input(key);
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

        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/BallRender.png")), image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = Texture2d::new(&display, image).unwrap();

        window.set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Confined))
            .unwrap();

        window.set_cursor_visible(false);

        let flattened_triangles: Vec<Vertex> = self.object.triangles
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
            model: Matrix::homogenous_slice(),
            u_texture: texture,
            u_light: light,
            view : view,
        }
    }
}


fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = Vect::from_slice(direction).normalize();

    let s = &Vect::from_slice(up).normalize().cross(&f).unwrap().normalize();

    let u = f.cross(&s).unwrap().normalize();

    let p = [
        -position[0] * s[0] - position[1] * s[1] - position[2] * s[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    let p = Vect::from_slice(&p);

    let res = Matrix::from_vects([s.clone(), u.clone(), f.clone(), p.clone()].to_vec());

    let slices = res.to_slices();

    [
        [slices[0][0], slices[1][0], slices[2][0], 0.0],
        [slices[0][1], slices[1][1], slices[2][1], 0.0],
        [slices[0][2], slices[1][2], slices[2][2], 0.0],
        [slices[3][0], slices[3][1], slices[3][2], 1.0],
    ]
}
