use std::path::Path;
use winit::dpi::PhysicalPosition;
use winit::keyboard::Key;
use gust_math::vect::Vect;
use crate::data::player::Player;
use crate::handlers::input_handler::GameInput;

pub struct Game {
    // vert_shader_path : Path,
    // frag_shader_path : Path,

    pub player : Player,
    pub game_input : GameInput,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player : Player::new(),
            game_input : GameInput::new(),
        }
    }

    pub fn update(&mut self) {
        if self.game_input.keyboard_input.is_character_pressed('r') {
            self.player.position = Vect::from_slice(&[0.0, 0.0, 5.0]);
            self.player.direction = Vect::from_slice(&[0.0f32, 0.0, -1.0]);
        }
        self.player.update(&self.game_input);
    }

    pub fn handle_keyboard_input(&mut self, key : Key) {
        self.game_input.handle_keyboard_input(key);
    }

    pub fn handle_key_release(&mut self, key : Key) {
        self.game_input.handle_key_release(key);
    }

    pub fn handle_mouse_input(&mut self, new_position: PhysicalPosition<f64>) {
        self.game_input.handle_mouse_input(new_position);
    }
}
