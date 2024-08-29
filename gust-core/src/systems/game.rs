use std::time::Instant;

use glium::{implement_uniform_block, Texture2d, uniform};
use glium::uniforms::{UniformBuffer, Uniforms};

use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;

use crate::components::camera::Camera;
use crate::components::player::Player;
use crate::handlers::event_handler::EventHandler;
use crate::handlers::input_handler::InputHandler;
use crate::objects::game_object::GameObject;
use crate::objects::intermediaries::wavefront_object::WavefrontObject;
use crate::primitives::mesh::Mesh;
use crate::systems::renderer::Renderer;

pub struct Game {
    pub t: f32,
    pub dt: f32,
    pub objects: Vec<GameObject>,
    pub player: Player,
    pub game_input: InputHandler,
    pub camera: Camera,
    pub last_frame_time: Instant,
}

impl Game {
    pub fn new() -> Self {
        let mesh = Mesh::from_wavefront(WavefrontObject::parse("C:\\Users\\Geert\\source\\repos\\Personal\\gust\\resources\\assets\\objects\\monkey.obj"));

        let object = GameObject {
            id: 0,
            name: "Monkey".to_string(),
            mesh: mesh,
            image: image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/green.png")), image::ImageFormat::Png).unwrap().to_rgba8(),
            object_to_parent: Mat4::identity().translate(Vect3::new(0.0, 0.0, 1.0)),
        };

        let floor_object = GameObject::init_floor_object();

        Game {
            t: 0.0,
            dt: 0.0,
            player: Player::init(),
            game_input: InputHandler::new(),
            objects: vec![object, floor_object],
            camera: Camera::init(),
            last_frame_time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        if self.game_input.keyboard_input.is_character_pressed('r') {
            self.player = Player::init();
        }
        self.player.update(&self.dt, &self.game_input);
    }

    pub fn run(&mut self) {
        let (event_handler, display) = EventHandler::new();

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

        let buffer = UniformBuffer::new(&display, UniformBlock {
            light_positions,
            _padding: [0.0; 5],
            light_colors,
        }).unwrap();

        let renderer = Renderer::new(display);

        event_handler.run(self, renderer, buffer);
    }

    pub fn get_uniforms<'a>(&'a self, player: Player, object: &GameObject, texture: &'a Texture2d, buffer: &'a UniformBuffer<UniformBlock>) -> impl Uniforms + 'a {
        let view = self.view_matrix(player.position, player.direction, player.up);

        let lights_used = 1;

        uniform! {
            perspective: self.camera.get_perspective(),
            model: object.get_model_matrix(),
            u_texture: texture,
            view : view,
            lightsBlock: &*buffer,
            u_light_count : lights_used,
        }
    }

    fn view_matrix(&self, position: Vect3, direction: Vect3, up: Vect3) -> [[f32; 4]; 4] {
        let f = direction.clone().normalize();
        let s = up.cross(&f).normalize();
        let u = f.cross(&s).normalize();

        let position = Vect3::new(position.x, position.y, position.z);

        let p = [
            -position.dot(&s),
            -position.dot(&u),
            -position.dot(&f),
        ];

        [
            [s.x, u.x, f.x, 0.0],
            [s.y, u.y, f.y, 0.0],
            [s.z, u.z, f.z, 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}

#[derive(Copy, Clone)]
pub struct UniformBlock {
    light_positions: [[f32; 3]; 5],
    _padding: [f32; 5],
    light_colors: [[f32; 3]; 5],
}
implement_uniform_block!(UniformBlock, light_positions, light_colors);
