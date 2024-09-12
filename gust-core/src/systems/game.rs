use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use glium::{Display, Texture2d, uniform};
use glium::glutin::surface::WindowSurface;
use glium::uniforms::{UniformBuffer, Uniforms};

use gust_hierarchy::world::World;
use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;

use crate::components::image_component::ImageComponent;
use crate::components::mesh_component::MeshComponent;
use crate::components::player_component::PlayerComponent;
use crate::components::transform_component::TransformComponent;
use crate::components::velocity_component::VelocityComponent;
use crate::entities::player::Player;
use crate::entities::viewer::Viewer;
use crate::handlers::event_handler::EventHandler;
use crate::handlers::input_handler::InputHandler;
use crate::objects::game_object::GameObject;
use crate::objects::intermediaries::wavefront_object::WavefrontObject;
use crate::primitives::lights_block::LightsBlock;
use crate::primitives::mesh::Mesh;
use crate::storages::mesh_storage::MeshStorage;
use crate::storages::texture_storage::TextureStorage;
use crate::systems::render_system::RenderSystem;
use crate::systems::update_systems::UpdateSystem;

pub struct Game {
    pub t: f32,
    pub dt: f32,
    pub objects: Vec<GameObject>,
    pub player: Player,
    pub input_handler: InputHandler,
    pub last_frame_time: Instant,
    pub world: World,
    pub mesh_storage: MeshStorage,
    pub texture_storage: TextureStorage,
}

impl Game {
    pub fn new() -> Self {
        let objects = Self::construct_objects();

        Game {
            t: 0.0,
            dt: 0.0,
            player: Player::init(),
            input_handler: InputHandler::new(),
            objects,
            last_frame_time: Instant::now(),
            world: World::new(),
            mesh_storage: MeshStorage::new(),
            texture_storage: TextureStorage::new(),
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

    fn construct_scene(&mut self, display: &Display<WindowSurface>) -> World {

        // Load the meshes
        let mesh = Mesh::from_wavefront(WavefrontObject::parse(Path::new("./resources/assets/objects/monkey.obj")));
        let floor_mesh = Mesh::from_wavefront(WavefrontObject::parse(Path::new("./resources/assets/objects/floor.obj")));

        // Add them to the mesh storage
        let monkey_mesh_id = self.mesh_storage.add_mesh(mesh);
        let floor_mesh_id = self.mesh_storage.add_mesh(floor_mesh);

        // Load the textures
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/green.png")), image::ImageFormat::Png).unwrap().to_rgba8();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.clone().into_raw(), image.dimensions());
        let texture = Texture2d::new(display, image).unwrap();

        let floor_image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/wood.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let floor_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&floor_image.clone().into_raw(), floor_image.dimensions());
        let floor_texture = Texture2d::new(display, floor_image).unwrap();

        // Add them to the texture storage
        let monkey_texture_id = self.texture_storage.add_texture(texture);
        let floor_texture_id = self.texture_storage.add_texture(floor_texture);

        let mut world = World::new();

        // Make player entity
        let player = world.spawn();
        let identity_transform = TransformComponent {
            position: [-5.0, 0.0, 1.0].into(),
            forward: [1.0, 0.0, 0.0].into(),
            up: [0.0, 0.0, 1.0].into(),
            scale: [1.0, 1.0, 1.0].into(),
        };

        let velocity = VelocityComponent {
            velocity: [0.0, 0.0, 0.0].into(),
            acceleration: [0.0, 0.0, 0.0].into(),
        };

        world.add_component(player, identity_transform);
        world.add_component(player, velocity);
        world.add_component(player, PlayerComponent);

        // Make monkey object
        let monkey = world.spawn();
        let monkey_transform = TransformComponent {
            position: [0.0, 0.0, 1.0].into(),
            forward: [1.0, 0.0, 0.0].into(),
            up: [0.0, 0.0, 1.0].into(),
            scale: [1.0, 1.0, 1.0].into(),
        };

        world.add_component(monkey, monkey_transform);
        world.add_component(monkey, MeshComponent(monkey_mesh_id));
        world.add_component(monkey, ImageComponent(monkey_texture_id));

        // Make floor object
        let floor = world.spawn();
        let floor_transform = TransformComponent {
            position: [0.0, 0.0, 0.0].into(),
            forward: [1.0, 0.0, 0.0].into(),
            up: [0.0, 0.0, 1.0].into(),
            scale: [1.0, 1.0, 1.0].into(),
        };

        world.add_component(floor, floor_transform);
        world.add_component(floor, MeshComponent(floor_mesh_id));
        world.add_component(floor, ImageComponent(floor_texture_id));

        world
    }

    pub fn update(&mut self) {
        // if self.input_handler.keyboard_input.is_character_pressed('r') {
        //     self.player = Player::init();
        // }
        // let mut player = self.player.clone();
        // player.update(&self.dt, &self.input_handler);
        // self.player = player;

        UpdateSystem::update(self.dt, &self.input_handler, &mut self.world);
    }

    pub fn run(&mut self) {
        let (mut event_handler, display) = EventHandler::new();

        let render_system = RenderSystem::new(display);

        event_handler.run(self, render_system);
    }

    pub fn get_uniforms<'b>(&'b self, player: Player, object: &GameObject, texture: &'b Texture2d, buffer: &'b UniformBuffer<LightsBlock>) -> impl Uniforms + 'b {
        let view = player.view_matrix();

        let lights_used = 1;

        uniform! {
            perspective: player.get_perspective(),
            view : view,
            model: object.get_model_matrix(),
            u_texture: texture,
            lightsBlock: &*buffer,
            u_light_count : lights_used,
        }
    }
}

