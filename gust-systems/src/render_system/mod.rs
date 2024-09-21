use glium::{Display, Frame, Program, Surface, Texture2d, uniform, VertexBuffer};
use glium::DrawParameters;
use glium::glutin::surface::WindowSurface;
use glium::uniforms::{UniformBuffer, Uniforms};

use gust_components::Component;
use gust_components::Component::{CameraComponent, MeshComponent, TextureComponent, TransformComponent};
use gust_components::ComponentType::{CameraComponentType, MeshComponentType, PlayerComponentType, TextureComponentType, TransformComponentType};
use gust_core::entity::Entity;
use gust_core::primitives::lights_block::LightsBlock;
use gust_core::primitives::mesh::Mesh;
use gust_core::primitives::vertex::Vertex;
use gust_hierarchy::world::World;
use gust_math::matrices::mat4::Mat4;

use crate::game::Game;

pub struct RenderSystem {
    pub display: Display<WindowSurface>,
    program: Program,
    params: DrawParameters<'static>,
}

impl RenderSystem {
    pub fn new(display: Display<WindowSurface>) -> Self {
        let vert_shader_string = include_str!("../../../resources/shaders/vert.glsl");
        let frac_shader_string = include_str!("../../../resources/shaders/frac.glsl");

        let program = Program::from_source(
            &display,
            vert_shader_string,
            frac_shader_string,
            None,
        ).unwrap();

        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        RenderSystem {
            display,
            program,
            params,
        }
    }

    pub fn render(&self, game: &Game, buffer: &UniformBuffer<LightsBlock>) {
        self.draw_objects(game, buffer);
    }

    pub fn draw_objects(&self, game: &Game, buffer: &UniformBuffer<LightsBlock>) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let player = game.world.query_one(PlayerComponentType).collect::<Vec<(Entity, &Component)>>().iter().map(|(entity, _component)| *entity).collect::<Vec<Entity>>()[0];
        let TransformComponent(player_transform) = game.world.get_component(player, TransformComponentType).unwrap() else { return; };
        let CameraComponent(player_camera) = game.world.get_component(player, CameraComponentType).unwrap() else { return; };
        let player_view = player_camera.view_matrix(player_transform.position, player_transform.forward, player_transform.up);
        let player_perspective = player_camera.get_perspective();

        // Render each item in game.world
        for entity in game.world.entities.iter() {
            if game.world.has_component(*entity, PlayerComponentType)
                || !game.world.has_component(*entity, TransformComponentType)
                || !game.world.has_component(*entity, MeshComponentType)
                || !game.world.has_component(*entity, TextureComponentType) {
                continue;
            }

            let TransformComponent(object_transform) = game.world.get_component(*entity, TransformComponentType).unwrap() else { continue; };
            let MeshComponent(mesh_id) = game.world.get_component(*entity, MeshComponentType).unwrap() else { continue; };
            let TextureComponent(texture_id) = game.world.get_component(*entity, TextureComponentType).unwrap() else { continue; };

            let object_transform_matrix = object_transform.get_transform_matrix();
            let object_transform = self.propagate_transform(*entity, object_transform_matrix, &game.world).to_slices();
            let mesh = game.mesh_storage.get_mesh(mesh_id.0).unwrap();
            let texture = game.texture_storage.get_texture(texture_id.0).unwrap();

            self.draw_object(&mut target, &self.display, object_transform, player_view, player_perspective, mesh, texture, buffer);
        }

        target.finish().unwrap();
    }

    pub fn draw_object(&self, target: &mut Frame, display: &Display<WindowSurface>, object_transform: [[f32; 4]; 4], player_view: [[f32; 4]; 4], player_perspective: [[f32; 4]; 4], mesh: &Mesh, texture: &Texture2d, buffer: &UniformBuffer<LightsBlock>) {
        let uniforms = self.get_uniforms(object_transform, player_view, player_perspective, &texture, &buffer);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let flattened_triangles: Vec<Vertex> = mesh.triangles
            .iter()
            .flat_map(|triangle| triangle.iter().cloned())
            .collect();

        let vertex_buffer = VertexBuffer::new(display, &flattened_triangles).unwrap();

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.program,
                &uniforms,
                &self.params,
            )
            .unwrap();
    }

    fn get_uniforms<'a>(&'a self, model_transform: [[f32; 4]; 4], player_view: [[f32; 4]; 4], player_perspective: [[f32; 4]; 4], texture: &'a Texture2d, buffer: &'a UniformBuffer<LightsBlock>) -> impl Uniforms + 'a {
        let lights_used = 5;

        uniform! {
            perspective: player_perspective,
            view : player_view,
            model: model_transform,
            u_texture: texture,
            lightsBlock: &*buffer,
            u_light_count : lights_used,
        }
    }

    fn propagate_transform(&self, entity: Entity, entity_transform: Mat4, world: &World) -> Mat4 {
        // Get parent transform if it exists
        let parent = world.get_parent(entity);

        if parent.is_none() {
            return entity_transform;
        }

        let parent = parent.unwrap();
        if let None = world.get_component(parent, TransformComponentType) {
            // If the parent doesn't have a transform, it cannot exist in the scene
            // Hence, we return an error
            panic!("Parent entity does not have a transform component");
        }
        let TransformComponent(parent_transform) = world.get_component(parent, TransformComponentType).unwrap() else { return entity_transform; };

        let new_transform = parent_transform.get_transform_matrix() * entity_transform;

        // Recursively propagate the transform
        self.propagate_transform(parent, new_transform, world)
    }
}
