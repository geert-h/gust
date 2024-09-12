use std::collections::HashMap;

use crate::primitives::mesh::Mesh;

pub struct MeshStorage {
    pub meshes: HashMap<MeshId, Mesh>,
    mesh_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshId(pub u32);

impl MeshStorage {
    pub fn new() -> Self {
        MeshStorage {
            meshes: HashMap::new(),
            mesh_count: 0,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshId {
        self.mesh_count += 1;
        let mesh_id = MeshId(self.mesh_count as u32);
        let res = self.meshes.insert(mesh_id, mesh);

        if res.is_some() {
            self.mesh_count -= 1;
        }

        mesh_id
    }

    pub fn get_mesh(&self, mesh_id: MeshId) -> Option<&Mesh> {
        self.meshes.get(&mesh_id)
    }

    pub fn get_mesh_mut(&mut self, mesh_id: MeshId) -> Option<&mut Mesh> {
        self.meshes.get_mut(&mesh_id)
    }

    pub fn has_mesh(&self, mesh_id: MeshId) -> bool {
        self.meshes.contains_key(&mesh_id)
    }

    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }
}