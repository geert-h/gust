use crate::objects::intermediaries::wavefront_object::WavefrontObject;

pub fn parse_wavefront_object(file_path: &str) -> WavefrontObject {
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
                let face = parse_face(line);
                faces.push(face);
            }
            _ => {}
        }
    }

    WavefrontObject {
        vertices,
        tex_coords,
        normals,
        faces,
    }
}

fn parse_face(mut line: &str) -> Vec<[u32; 3]> {
    //remove the 'f' from the line
    line = line.trim_start_matches('f').trim();
    let mut faces = Vec::new();
    for face in line.split_whitespace() {
        let mut indices: [u32; 3] = [0; 3];
        for (index, value) in face.split('/').enumerate() {
            if value.is_empty() {
                continue;
            }
            indices[index] = value.parse().unwrap();
        }
        faces.push(indices);
    }
    faces
}