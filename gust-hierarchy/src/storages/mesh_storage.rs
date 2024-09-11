use std::collections::HashMap;

pub struct MeshStorage {
    pub meshes: HashMap<MeshId, String>,
}

pub struct MeshId(u32);