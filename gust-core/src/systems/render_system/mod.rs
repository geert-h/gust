use glium::{Display, Frame, Program, Surface, Texture2d, VertexBuffer};
use glium::DrawParameters;
use glium::glutin::surface::WindowSurface;
use glium::uniforms::{UniformBuffer, Uniforms};
use crate::components::texture_component::TextureComponent;
use crate::components::mesh_component::MeshComponent;
use crate::components::transform_component::TransformComponent;
use crate::objects::game_object::GameObject;
use crate::primitives::lights_block::LightsBlock;
use crate::primitives::mesh::Mesh;
use crate::primitives::vertex::Vertex;
use crate::systems::game::Game;

pub struct RenderSystem {
    pub display: Display<WindowSurface>,
    program: Program,
    params: DrawParameters<'static>,
}

impl RenderSystem {
    pub fn new(display: Display<WindowSurface>) -> Self {
        let vert_shader_string = include_str!("../../../../resources/shaders/vert.glsl");
        let frac_shader_string = include_str!("../../../../resources/shaders/frac.glsl");

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

    pub fn render(&self, game: &Game, textures: &[Texture2d], buffer: &UniformBuffer<LightsBlock>) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        for (object, texture) in game.objects.iter().zip(textures) {
            let flattened_triangles: Vec<Vertex> = object.mesh.triangles
                .iter()
                .flat_map(|triangle| triangle.iter().cloned())
                .collect();

            let vertex_buffer = VertexBuffer::new(&self.display, &flattened_triangles).unwrap();

            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &self.program,
                    &game.get_uniforms(game.player.clone(), object, texture, &buffer),
                    &self.params,
                )
                .unwrap();
        }
        target.finish().unwrap();
    }

    pub fn draw_objects(&self, game: &Game, textures: &[Texture2d], buffer: &UniformBuffer<LightsBlock>) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        for (object, texture) in game.objects.iter().zip(textures) {
            self.draw_object(&mut target, &self.display, object, &game.get_uniforms(game.player.clone(), object, texture, &buffer));
        }

        target.finish().unwrap();
    }

    pub fn draw_object(&self, target: &mut Frame, display: &Display<WindowSurface>, object: &GameObject, uniforms: &impl Uniforms) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let flattened_triangles: Vec<Vertex> = object.mesh.triangles
            .iter()
            .flat_map(|triangle| triangle.iter().cloned())
            .collect();

        let vertex_buffer = VertexBuffer::new(display, &flattened_triangles).unwrap();

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.program,
                uniforms,
                &self.params,
            )
            .unwrap();
    }

    pub fn new_draw_objects(&self, game: &Game) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Render each item in game.world 
        for entity in game.world.entities.iter() {
            let transform = game.world.get_component::<TransformComponent>(*entity).unwrap();
            let mesh = game.world.get_component::<MeshComponent>(*entity).unwrap();
            let image = game.world.get_component::<TextureComponent>(*entity).unwrap();

            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

            let flattened_triangles: Vec<Vertex> = mesh.triangles
                .iter()
                .flat_map(|triangle| triangle.iter().cloned())
                .collect();

            let vertex_buffer = VertexBuffer::new(&self.display, &flattened_triangles).unwrap();

            let uniforms = game.get_uniforms(game.player.clone(), object, texture, &buffer);

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
    }

    pub fn new_draw_object(&self, target: &mut Frame, display: &Display<WindowSurface>, transform: TransformComponent, mesh: Mesh, texture: Texture2d) {
        let uniforms = ::glium::uniforms::UniformsStorage::new("perspective", (player.get_perspective()));
        let uniforms = uniforms.add("view", view);
        let uniforms = uniforms.add("model", (object.get_model_matrix()));
        let uniforms = uniforms.add("u_texture", texture);
        let uniforms = uniforms.add("lightsBlock", (&*buffer));
        let uniforms = uniforms.add("u_light_count", lights_used);

        let uniforms = game.get_uniforms(game.player.clone(), object, texture, &buffer);

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
                uniforms,
                &self.params,
            )
            .unwrap();
    }
}
