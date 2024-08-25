use glium::{Texture2d, uniform};
use glium::uniforms::Uniforms;

use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;

use crate::components::camera::Camera;
use crate::components::light::Light;
use crate::components::player::Player;
use crate::handlers::event_handler::EventHandler;
use crate::handlers::input_handler::InputHandler;
use crate::objects::game_object::GameObject;
use crate::systems::renderer::Renderer;

pub struct Game {
    pub t: f32,
    pub objects: Vec<GameObject>,
    lights: Vec<Light>,
    pub player: Player,
    pub game_input: InputHandler,
    pub camera: Camera,
}

impl Game {
    pub fn new() -> Self {
        let object = GameObject::init_floor_object();
        let object2 = GameObject::init();

        Game {
            t: 0.0,
            player: Player::init(),
            game_input: InputHandler::new(),
            objects: vec![object, object2],
            lights: vec![],
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
        let (event_handler, display) = EventHandler::new();
        let renderer = Renderer::new(display);
        event_handler.run(self, renderer);
    }

    pub fn get_uniforms<'a>(&'a self, player: Player, texture: &'a Texture2d) -> impl Uniforms + 'a {
        let view = self.view_matrix(player.position, player.direction, player.up);

        uniform! {
            perspective: self.camera.get_perspective(),
            model: Mat4::identity().to_slices(),
            u_texture: texture,
            view : view,
        }
    }

    fn view_matrix(&self, position: Vect3, direction: Vect3, up: Vect3) -> [[f32; 4]; 4] {
        let f = direction.clone().normalize();
        let s = up.cross(&f).normalize();
        let u = f.cross(&s).normalize();

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
