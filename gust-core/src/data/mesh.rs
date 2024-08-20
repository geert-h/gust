use crate::data::vertex::Vertex;
use crate::objects::intermediaries::wavefront_object::WavefrontObject;

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<[Vertex; 3]>,
}

impl Mesh {
    pub fn from_wavefront(wavefront_object: WavefrontObject, v: f32) -> Self {
        let mut polygons = Vec::new();
        for face in wavefront_object.faces {
            let mut vertices = Vec::new();

            for vertex_indices in face {
                let mut temp = wavefront_object.vertices[(vertex_indices[0] - 1) as usize];
                temp[0] += v;
                let vertex = Vertex {
                    position: temp,
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


    pub fn construct_floor_mesh() -> Self {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();

        let mut vertex = Vertex {
            position: [1.0, 1.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 0.0],
        };
        vertices.push(vertex);
        normals.push(vertex.normal);
        tex_coords.push(vertex.tex_coords);

        vertex = Vertex {
            position: [-1.0, 1.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 0.0],
        };
        vertices.push(vertex);
        normals.push(vertex.normal);
        tex_coords.push(vertex.tex_coords);

        vertex = Vertex {
            position: [-1.0, -1.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 1.0],
        };
        vertices.push(vertex);
        normals.push(vertex.normal);
        tex_coords.push(vertex.tex_coords);

        vertex = Vertex {
            position: [1.0, -1.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 1.0],
        };
        vertices.push(vertex);
        normals.push(vertex.normal);
        tex_coords.push(vertex.tex_coords);

        let triangles = vec![
            [vertices[0], vertices[1], vertices[2]],
            [vertices[0], vertices[2], vertices[3]],
        ];

        Mesh { triangles }
    }
}

// pub fn from_wavefront_object(wavefront_object: WavefrontObject, v: f32) -> Mesh {
//     let mut polygons = Vec::new();
//     for face in wavefront_object.faces {
//         let mut vertices = Vec::new();
//
//         for vertex_indices in face {
//             let mut temp = wavefront_object.vertices[(vertex_indices[0] - 1) as usize];
//             temp[0] += v;
//             let vertex = Vertex {
//                 position: temp,
//                 tex_coords: wavefront_object.tex_coords[(vertex_indices[1] - 1) as usize],
//                 normal: wavefront_object.normals[(vertex_indices[2] - 1) as usize],
//             };
//             vertices.push(vertex);
//         }
//         polygons.push(vertices);
//     }
//
//     let mut triangles = Vec::new();
//     for polygon in polygons {
//         triangles.append(&mut polygon_to_triangles(polygon));
//     }
//
//     Mesh { triangles }
// }
