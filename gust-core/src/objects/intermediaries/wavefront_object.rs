#[derive(Debug, Clone)]
pub struct WavefrontObject {
    pub vertices: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub faces: Vec<Vec<[u32; 3]>>,
}

// Face can have multiple forms
// 1. a list of vertex indices.
//   This has the form v1 v2 v3
// 2. a list of vertex and texture indices
//   This has the form v1/vt1 v2/vt2 v3/vt3
// 3. a list of vertex, texture, and normal indices
//   This has the form v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
// 4. a list of vertex and normal indices
//   This has the form v1//vn1 v2//vn2 v3//vn3