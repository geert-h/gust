use crate::data::face::Face;
use crate::data::vertex::Vertex;
pub struct Mesh {
    pub vertices: [f32; 3],
    pub faces: Vec<[f32; 3]>,
    pub normals: [f32; 3],

}
