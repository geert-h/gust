use std::collections::HashMap;

use winit::dpi::PhysicalPosition;
use winit::keyboard::Key;

use crate::handlers::input_handler::keyboard_state::KeyBoardState;
use crate::handlers::input_handler::mouse_state::MouseState;

mod keyboard_state;
mod mouse_state;

pub struct InputHandler {
    pub keyboard_input: KeyBoardState,
    pub mouse_input: MouseState,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            keyboard_input: KeyBoardState {
                pressed_keys: HashMap::new(),
            },
            mouse_input: MouseState {
                mouse_position: (0.0, 0.0),
                mouse_delta: (0.0, 0.0),
                lmb_pressed: false,
                rmb_pressed: false,
            },
        }
    }

    pub fn handle_keyboard_input(&mut self, key: Key) {
        // Check if in the pressed keys
        if self.keyboard_input.is_key_pressed(key.clone()) {
            let duration = self.keyboard_input.get_key_duration(key.clone()).unwrap();
            self.keyboard_input.update_duration(key, duration + 1.0);
        } else {
            self.keyboard_input.add_key(key);
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard_input.remove_key(key);
    }

    pub fn handle_mouse_input(&mut self, new_position: PhysicalPosition<f64>) {
        let delta_x = new_position.x as f32 - self.mouse_input.mouse_position.0;
        let delta_y = new_position.y as f32 - self.mouse_input.mouse_position.1;

        self.mouse_input.mouse_delta = (delta_x, delta_y);
        self.mouse_input.mouse_position = (new_position.x as f32, new_position.y as f32);
    }
}
