use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use glium::{implement_uniform_block, Texture2d, uniform};
use glium::uniforms::{UniformBuffer, Uniforms};

use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;
use crate::components::floor_object::FloorObject;
use crate::components::player::Player;
use crate::components::viewer::Viewer;
use crate::handlers::event_handler::EventHandler;
use crate::handlers::input_handler::InputHandler;
use crate::objects::game_object::GameObject;
use crate::objects::intermediaries::wavefront_object::WavefrontObject;
use crate::primitives::mesh::Mesh;
use crate::scene::scene_tree::{GameTreeObject, Node, SceneTree};
use crate::systems::renderer::Renderer;

pub struct Game {
    pub t: f32,
    pub dt: f32,
    pub objects: Vec<GameObject>,
    pub player: Player,
    pub game_input: InputHandler,
    pub last_frame_time: Instant,
}

impl Game {
    pub fn new() -> Self {
        let objects = Self::construct_objects();

        Game {
            t: 0.0,
            dt: 0.0,
            player: Player::init(),
            game_input: InputHandler::new(),
            objects,
            last_frame_time: Instant::now(),
        }
    }

    fn construct_objects() -> Vec<GameObject> {
        let mesh = Mesh::from_wavefront(WavefrontObject::parse(Path::new("./resources/assets/objects/monkey.obj")));

        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/green.png")), image::ImageFormat::Png).unwrap().to_rgba8();

        let object = GameObject {
            id: 0,
            name: "Monkey".to_string(),
            mesh: Rc::new(mesh),
            image: Rc::new(image),
            object_to_parent: Mat4::identity().translate(Vect3::new(0.0, 0.0, 1.0)),
        };

        let wavefront_object = WavefrontObject::parse(Path::new("./resources/assets/objects/floor.obj"));
        let mesh = Mesh::from_wavefront(wavefront_object);
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/wood.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();

        let floor_object = GameObject::new(0, "floor".to_string(), Rc::new(image), Rc::new(mesh), Mat4::identity());

        vec![object, floor_object]
    }

    fn build_scene_tree(&self) -> SceneTree<dyn GameTreeObject> {
        let scene_tree = SceneTree::new();

        let player = Node::new(self.player.clone());

        let wavefront_object = WavefrontObject::parse(Path::new("./resources/assets/objects/floor.obj"));
        let mesh = Mesh::from_wavefront(wavefront_object);
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/wood.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();

        let floor_object = FloorObject::new(Vect3::zeros(), Vect3::new(0.0, 0.0, 1.0), Vect3::new(0.0, 1.0, 0.0), Rc::new(mesh), Rc::new(image));

        let floor = Node::new(floor_object);

        scene_tree.root.

            scene_tree.set_viewer(player);

        scene_tree
    }

    pub fn load_objects(&mut self) {
        self.objects = Self::construct_objects();
    }

    pub fn update(&mut self) {
        if self.game_input.keyboard_input.is_character_pressed('r') {
            self.player = Player::init();
        }
        let mut player = self.player.clone();
        player.update(self);
        self.player = player;
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

    pub fn get_uniforms<'b>(&'b self, player: Player, object: &GameObject, texture: &'b Texture2d, buffer: &'b UniformBuffer<UniformBlock>) -> impl Uniforms + 'b {
        let view = player.view_matrix();

        let lights_used = 1;

        uniform! {
            perspective: player.get_perspective(),
            model: object.get_model_matrix(),
            u_texture: texture,
            view : view,
            lightsBlock: &*buffer,
            u_light_count : lights_used,
        }
    }
}

#[derive(Copy, Clone)]
pub struct UniformBlock {
    light_positions: [[f32; 3]; 5],
    _padding: [f32; 5],
    light_colors: [[f32; 3]; 5],
}
implement_uniform_block!(UniformBlock, light_positions, light_colors);
