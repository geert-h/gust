use data::mesh;
use crate::data;

pub fn parse_obj_file(file_path: &str) -> mesh::Mesh {
    let mut mesh = mesh::Mesh::new();
    let file = std::fs::read_to_string(file_path).unwrap();
    for line in file.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                mesh.add_vertex(data::vertex::Vertex::new(x, y, z));
            }
            Some("f") => {
                let mut face = data::face::Face::new();
                for part in parts {
                    let vertex_index: usize = part.parse().unwrap();
                    face.add_vertex(mesh.get_vertex(vertex_index - 1));
                }
                mesh.add_face(face);
            }
            _ => {}
        }
    }
    mesh
}