use std::collections::HashMap;

use winit::keyboard::Key;

pub struct KeyBoardState {
    pub pressed_keys: HashMap<Key, f32>,
}

impl KeyBoardState {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains_key(&key)
    }

    pub fn is_character_pressed(&self, key: char) -> bool {
        self.pressed_keys.contains_key(&Key::Character(smol_str::SmolStr::from(key.to_string())))
    }

    pub fn get_key_duration(&self, key: Key) -> Option<f32> {
        self.pressed_keys.get(&key).map(|duration| *duration)
    }

    pub fn add_key(&mut self, key: Key) {
        self.pressed_keys.insert(key, 0.0);
    }

    pub fn remove_key(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    pub fn update_duration(&mut self, key: Key, duration: f32) {
        self.pressed_keys.insert(key, duration);
    }
}
