use data::wavefront_object;
use crate::data;

pub fn parse_wavefront_object(file_path: &str) -> wavefront_object::WavefrontObject {
    let file = std::fs::read_to_string(file_path).expect("Failed to read file");

    let mut vertices = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut faces = Vec::new();

    for line in file.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                vertices.push([x, y, z]);
            }
            Some("vt") => {
                let u: f32 = parts.next().unwrap().parse().unwrap();
                let v: f32 = parts.next().unwrap().parse().unwrap();
                tex_coords.push([u, v]);
            }
            Some("vn") => {
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                normals.push([x, y, z]);
            }
            Some("f") => {
                let mut face = [0; 3];
                for i in 0..3 {
                    let indices: Vec<&str> = parts.next().unwrap().split('/').collect();
                    let vertex_index: u32 = indices[0].parse().unwrap();
                    face[i] = vertex_index - 1;
                }
                faces.push(face);
            }
            _ => {}
        }
    }

    wavefront_object::WavefrontObject {
        vertices,
        tex_coords,
        normals,
        faces,
    }
}