use std::f32::consts::PI;
use std::path::Path;
use std::time::Instant;

use glium::{Display, Texture2d};
use glium::glutin::surface::WindowSurface;

use gust_components::Component::{CameraComponent, MeshComponent, PlayerComponent, TextureComponent, TransformComponent, VelocityComponent};
use gust_components::components::camera_component::CameraComponentImpl;
use gust_components::components::player_component::PlayerComponentImpl;
use gust_components::components::transform_component::TransformComponentImpl;
use gust_components::components::velocity_component::VelocityComponentImpl;
use gust_core::handlers::input_handler::InputHandler;
use gust_core::objects::intermediaries::wavefront_object::WavefrontObject;
use gust_core::primitives::mesh::Mesh;
use gust_core::storages::mesh_storage::MeshStorage;
use gust_core::storages::texture_storage::TextureStorage;
use gust_hierarchy::world::World;

use crate::event_handler::EventHandler;
use crate::render_system::RenderSystem;
use crate::update_systems::UpdateSystem;

pub struct Game {
    pub t: f32,
    pub dt: f32,
    pub input_handler: InputHandler,
    pub last_frame_time: Instant,
    pub world: World,
    pub mesh_storage: MeshStorage,
    pub texture_storage: TextureStorage,
}

impl Game {
    pub fn new() -> Self {
        Game {
            t: 0.0,
            dt: 0.0,
            input_handler: InputHandler::new(),
            last_frame_time: Instant::now(),
            world: World::new(),
            mesh_storage: MeshStorage::new(),
            texture_storage: TextureStorage::new(),
        }
    }

    fn construct_scene(&mut self, display: &Display<WindowSurface>) -> World {
        // Load the meshes
        let monkey_mesh = Mesh::from_wavefront(WavefrontObject::parse(Path::new("resources/assets/objects/monkey.obj")));
        let floor_mesh = Mesh::from_wavefront(WavefrontObject::parse(Path::new("resources/assets/objects/floor.obj")));

        // Add them to the mesh storage
        let monkey_mesh_id = self.mesh_storage.add_mesh(monkey_mesh);
        let floor_mesh_id = self.mesh_storage.add_mesh(floor_mesh);

        // Load the textures
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../resources/assets/green.png")), image::ImageFormat::Png).unwrap().to_rgba8();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.clone().into_raw(), image.dimensions());
        let texture = Texture2d::new(display, image).unwrap();

        let floor_image = image::load(std::io::Cursor::new(&include_bytes!("../../resources/assets/wood.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let floor_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&floor_image.clone().into_raw(), floor_image.dimensions());
        let floor_texture = Texture2d::new(display, floor_image).unwrap();

        // Add them to the texture storage
        let monkey_texture_id = self.texture_storage.add_texture(texture);
        let floor_texture_id = self.texture_storage.add_texture(floor_texture);

        let mut world = World::new();

        // Make player entity
        let player = world.spawn();
        let identity_transform = TransformComponentImpl::default().with_position([-5.0, 0.0, 1.0].into());

        let velocity = VelocityComponentImpl {
            velocity: [0.0, 0.0, 0.0].into(),
        };

        let camera = CameraComponentImpl {
            fov: PI / 3.0,
            aspect_ratio: 480.0 / 800.0,
            z_near: 0.1,
            z_far: 1024.0,
        };

        world.add_component(player, TransformComponent(identity_transform));
        world.add_component(player, VelocityComponent(velocity.clone()));
        world.add_component(player, PlayerComponent(PlayerComponentImpl));
        world.add_component(player, CameraComponent(camera));

        // Make monkey object
        let monkey = world.spawn();
        let monkey_transform = TransformComponentImpl::default()
            .with_position([0.0, 0.0, 1.0].into());

        world.add_component(monkey, TransformComponent(monkey_transform));
        world.add_component(monkey, MeshComponent(gust_components::components::mesh_component::MeshComponentImpl(monkey_mesh_id)));
        world.add_component(monkey, TextureComponent(gust_components::components::texture_component::TextureComponentImpl(monkey_texture_id)));
        world.add_component(monkey, VelocityComponent(velocity));

        // Make floor object
        let floor = world.spawn();
        let floor_transform = TransformComponentImpl::default();

        world.add_component(floor, TransformComponent(floor_transform));
        world.add_component(floor, MeshComponent(gust_components::components::mesh_component::MeshComponentImpl(floor_mesh_id)));
        world.add_component(floor, TextureComponent(gust_components::components::texture_component::TextureComponentImpl(floor_texture_id)));

        let transform_entity = world.spawn();
        let transform = TransformComponentImpl::default().with_scale([1.0, 1.0, 1.0].into());

        world.add_component(transform_entity, TransformComponent(transform));

        world.set_parent(transform_entity, monkey);

        world
    }

    pub fn update(&mut self) {
        UpdateSystem::update(self.dt, &self.input_handler, &mut self.world);
    }

    pub fn run(&mut self) {
        let (event_handler, display) = EventHandler::new();

        self.world = self.construct_scene(&display);

        let render_system = RenderSystem::new(display);

        event_handler.run(self, render_system);
    }
}

