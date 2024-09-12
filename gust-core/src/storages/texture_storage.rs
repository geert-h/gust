use std::collections::HashMap;

use glium::Texture2d;

pub struct TextureStorage {
    textures: HashMap<TextureId, Texture2d>,
    texture_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub u32);

impl TextureStorage {
    pub fn new() -> Self {
        TextureStorage {
            textures: HashMap::new(),
            texture_count: 0,
        }
    }

    pub fn add_texture(&mut self, texture: Texture2d) -> TextureId {
        self.texture_count += 1;
        let texture_id = TextureId(self.texture_count as u32);

        let res = self.textures.insert(texture_id, texture);

        if res.is_some() {
            self.texture_count -= 1;
        }

        texture_id
    }

    pub fn get_texture(&self, texture_id: TextureId) -> Option<&Texture2d> {
        self.textures.get(&texture_id)
    }

    pub fn get_texture_mut(&mut self, texture_id: TextureId) -> Option<&mut Texture2d> {
        self.textures.get_mut(&texture_id)
    }

    pub fn has_texture(&self, texture_id: TextureId) -> bool {
        self.textures.contains_key(&texture_id)
    }

    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }
}