use std::collections::HashMap;
use winit::dpi::PhysicalPosition;
use winit::keyboard::{Key};

pub struct GameInput {
    pub keyboard_input : KeyBoardInput,
    pub mouse_input : MouseInput,
}

pub struct KeyBoardInput {
    pressed_keys : HashMap<Key, f32>
}

impl KeyBoardInput {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains_key(&key)
    }

    pub fn is_character_pressed(&self, key: char) -> bool {
        self.pressed_keys.contains_key(&Key::Character(smol_str::SmolStr::from(key.to_string())))
    }

    fn get_key_duration(&self, key: Key) -> Option<f32> {
        self.pressed_keys.get(&key).map(|duration| *duration)
    }

    fn add_key(&mut self, key: Key) {
        self.pressed_keys.insert(key, 0.0);
    }

    fn remove_key(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    fn update_duration(&mut self, key: Key, duration: f32) {
        self.pressed_keys.insert(key, duration);
    }
}

pub struct MouseInput {
    pub mouse_position: (f32, f32),
    pub mouse_delta: (f32, f32),
    pub lmb_pressed: bool,
    pub rmb_pressed: bool,
}

impl GameInput {
    pub fn new() -> Self {
        GameInput {
            keyboard_input : KeyBoardInput {
                pressed_keys : HashMap::new(),
            },
            mouse_input : MouseInput {
                mouse_position : (0.0, 0.0),
                mouse_delta : (0.0, 0.0),
                lmb_pressed : false,
                rmb_pressed : false,
            },
        }
    }

    pub fn handle_keyboard_input(&mut self, key : Key) {
        // Check if in the pressed keys
        if self.keyboard_input.is_key_pressed(key.clone()) {
            let duration = self.keyboard_input.get_key_duration(key.clone()).unwrap();
            self.keyboard_input.update_duration(key, duration + 1.0);
        } else {
            self.keyboard_input.add_key(key);
        }
    }

    pub fn handle_key_release(&mut self, key : Key) {
        self.keyboard_input.remove_key(key);
    }

    pub fn handle_mouse_input(&mut self, new_position: PhysicalPosition<f64>) {
        let delta_x = new_position.x as f32 - self.mouse_input.mouse_position.0;
        let delta_y = new_position.y as f32 - self.mouse_input.mouse_position.1;

        self.mouse_input.mouse_delta = (delta_x, delta_y);
        self.mouse_input.mouse_position = (new_position.x as f32, new_position.y as f32);
    }
}