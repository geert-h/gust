use crate::objects::intermediaries::wavefront_object::WavefrontObject;
use crate::primitives::vertex::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<[Vertex; 3]>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            triangles: Vec::new(),
        }
    }

    pub fn from_wavefront(wavefront_object: WavefrontObject) -> Self {
        let mut polygons = Vec::new();
        for face in wavefront_object.faces {
            let mut vertices = Vec::new();

            for vertex_indices in face {
                let vertex = Vertex {
                    position: wavefront_object.vertices[(vertex_indices[0] - 1) as usize],
                    tex_coords: wavefront_object.tex_coords[(vertex_indices[1] - 1) as usize],
                    normal: wavefront_object.normals[(vertex_indices[2] - 1) as usize],
                };
                vertices.push(vertex);
            }
            polygons.push(vertices);
        }

        let mut triangles = Vec::new();
        for polygon in polygons {
            triangles.append(&mut Mesh::polygon_to_triangles(polygon));
        }

        Mesh { triangles }
    }

    fn polygon_to_triangles(polygon: Vec<Vertex>) -> Vec<[Vertex; 3]> {
        let mut triangles = Vec::new();

        // it is important that every triangle has the same winding order
        for i in 1..polygon.len() - 1 {
            triangles.push([polygon[0].clone(), polygon[i + 1].clone(), polygon[i].clone()]);
        }

        triangles
    }
}