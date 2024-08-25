use glium::implement_vertex;


#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }
}

implement_vertex!(Vertex, position, normal, tex_coords);