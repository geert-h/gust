use glium::{Display, Program, Surface, Texture2d, VertexBuffer};
use glium::DrawParameters;
use glium::glutin::surface::WindowSurface;
use glium::uniforms::UniformBuffer;

use crate::primitives::vertex::Vertex;
use crate::systems::game::{Game, UniformBlock};

pub struct Renderer {
    pub display: Display<WindowSurface>,
    program: Program,
    params: DrawParameters<'static>,
}

impl Renderer {
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

        Renderer {
            display,
            program,
            params,
        }
    }

    pub fn render(&self, game: &Game, textures: &[Texture2d], buffer: &UniformBuffer<UniformBlock>) {
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
                    &game.get_uniforms(game.player.clone(), texture, &buffer),
                    &self.params,
                )
                .unwrap();
        }
        target.finish().unwrap();
    }
}
