use std::f32::consts::PI;
use std::path::Path;
use std::time::Instant;

use glium::{Display, Texture2d};
use glium::glutin::surface::WindowSurface;

use gust_components::Component::{CameraComponent, ColliderComponent, MaterialComponent, MeshComponent, PlayerComponent, RigidBodyComponent, TextureComponent, TransformComponent, VelocityComponent};
use gust_components::components::camera_component::CameraComponentImpl;
use gust_components::components::player_component::PlayerComponentImpl;
use gust_components::components::transform_component::TransformComponentImpl;
use gust_components::components::velocity_component::VelocityComponentImpl;
use gust_components::physics::collider_component::ColliderComponentImpl;
use gust_components::physics::collider_component::ColliderType::AABB;
use gust_components::physics::material_component::MaterialComponentImpl;
use gust_components::physics::rigid_body_component::RigidBodyComponentImpl;
use gust_core::handlers::input_handler::InputHandler;
use gust_core::objects::intermediaries::wavefront_object::WavefrontObject;
use gust_core::primitives::mesh::Mesh;
use gust_core::storages::mesh_storage::MeshStorage;
use gust_core::storages::texture_storage::TextureStorage;
use gust_hierarchy::world::World;
use gust_math::vectors::vect3::Vect3;

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
        let monkey_wavefront = WavefrontObject::parse(Path::new("resources/assets/objects/monkey.obj"));
        let floor_wavefront = WavefrontObject::parse(Path::new("resources/assets/objects/floor.obj"));

        // Load the meshes
        let monkey_mesh = Mesh::from_wavefront(monkey_wavefront.clone());
        let floor_mesh = Mesh::from_wavefront(floor_wavefront.clone());

        let monkey_aabb = ColliderComponentImpl::from_wavefront_aabb(&monkey_wavefront);
        let floor_aabb = ColliderComponentImpl::from_wavefront_aabb(&floor_wavefront);

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

        let player_collider = ColliderComponentImpl {
            collider_type: AABB {
                min: Vect3::new(-0.5, -1.0, -0.5),
                max: Vect3::new(0.5, 1.0, 0.5),
            },
        };

        // Create RigidBodyComponent for the player
        let player_rigid_body = RigidBodyComponentImpl {
            mass: 1.0, // Adjust mass as needed
            forces: Vect3::zeros(),
            acceleration: Vect3::zeros(),
            is_static: false,
        };

        // Create MaterialComponent for the player
        let player_material = MaterialComponentImpl {
            friction: 0.6,     // Adjust as needed
            restitution: 0.3,  // Adjust as needed
        };

        world.add_component(player, TransformComponent(identity_transform));
        world.add_component(player, VelocityComponent(velocity.clone()));
        world.add_component(player, PlayerComponent(PlayerComponentImpl));
        world.add_component(player, CameraComponent(camera));
        world.add_component(player, ColliderComponent(player_collider));
        world.add_component(player, RigidBodyComponent(player_rigid_body));
        world.add_component(player, MaterialComponent(player_material));

        // Make monkey object
        let monkey = world.spawn();
        let monkey_transform = TransformComponentImpl::default()
            .with_position([0.0, 0.0, 1.0].into());

        // Create RigidBodyComponent for the monkey
        let monkey_rigid_body = RigidBodyComponentImpl {
            mass: 1.0,
            forces: Vect3::zeros(),
            acceleration: Vect3::zeros(),
            is_static: false,
        };

        // Create MaterialComponent for the monkey
        let monkey_material = MaterialComponentImpl {
            friction: 0.5,
            restitution: 0.8,
        };

        // Add components to the monkey entity
        world.add_component(monkey, TransformComponent(monkey_transform));
        world.add_component(monkey, MeshComponent(gust_components::components::mesh_component::MeshComponentImpl(monkey_mesh_id)));
        world.add_component(monkey, TextureComponent(gust_components::components::texture_component::TextureComponentImpl(monkey_texture_id)));
        world.add_component(monkey, VelocityComponent(velocity));
        world.add_component(monkey, ColliderComponent(monkey_aabb));
        world.add_component(monkey, RigidBodyComponent(monkey_rigid_body));
        world.add_component(monkey, MaterialComponent(monkey_material));

        // Make floor object
        let floor = world.spawn();
        let floor_transform = TransformComponentImpl::default();

        // Create RigidBodyComponent for the floor
        let floor_rigid_body = RigidBodyComponentImpl {
            mass: 0.0, // Mass is irrelevant for static objects
            forces: Vect3::zeros(),
            acceleration: Vect3::zeros(),
            is_static: true, // Indicates that the object is static
        };

        // Create MaterialComponent for the floor
        let floor_material = MaterialComponentImpl {
            friction: 0.9,     // High friction
            restitution: 0.5,  // Low restitution (less bouncy)
        };

        // Add components to the floor entity
        world.add_component(floor, TransformComponent(floor_transform));
        world.add_component(floor, MeshComponent(gust_components::components::mesh_component::MeshComponentImpl(floor_mesh_id)));
        world.add_component(floor, TextureComponent(gust_components::components::texture_component::TextureComponentImpl(floor_texture_id)));
        world.add_component(floor, ColliderComponent(floor_aabb));
        world.add_component(floor, RigidBodyComponent(floor_rigid_body));
        world.add_component(floor, MaterialComponent(floor_material));

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

