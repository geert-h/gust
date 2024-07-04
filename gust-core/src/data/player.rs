use std::ops::Neg;
use winit::keyboard::{Key, NamedKey};
use gust_math::matrix::Matrix;
use gust_math::vect::Vect;
use crate::handlers::input_handler::GameInput;

pub struct Player {
    pub position: Vect,
    pub direction: Vect,
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vect::new(3),
            direction: Vect::new(3),
            speed: 0.1,
        }
    }

    pub fn update(&mut self, game_input : &GameInput) {
        self.update_direction(game_input);
        self.update_position(game_input);
    }

    fn update_direction(&mut self, game_input: &GameInput) {
        let mut direction = self.direction.clone();

        let delta_x = game_input.mouse_input.mouse_delta.0;
        let delta_y = game_input.mouse_input.mouse_delta.1;

        let sensitivity = 0.0005;
        let delta_x = delta_x * sensitivity;
        let delta_y = delta_y * sensitivity;

        let axis = Vect::from_slice(&[0.0, 1.0, 0.0]);

        let rotation_matrix = Matrix::rotation_matrix(&axis, -delta_x);

        let mut new_direction = direction.clone();

        new_direction[1] += clamp(delta_y, -1.0, 1.0);

        new_direction = rotation_matrix * new_direction;

        new_direction.normalize();

        self.direction = new_direction;
    }


    fn update_position(&mut self, game_input: &GameInput) {
        let mut direction = self.direction.clone();
        let mut position = self.position.clone();

        let mut x_z_direction = Vect::from_slice(&[direction[0], 0.0, direction[2]]);
        x_z_direction.normalize();

        if game_input.keyboard_input.is_character_pressed('w') {
            position = position + x_z_direction.clone() * self.speed;
            println!("Position: {:?}", position);
        }

        if game_input.keyboard_input.is_character_pressed('s') {
            position = position - x_z_direction.clone() * self.speed;
        }

        if game_input.keyboard_input.is_character_pressed('a') {
            let right = Vect::from_slice(&[direction[2], direction[1], direction[0]]);
            position = position - right * self.speed;
        }

        if game_input.keyboard_input.is_character_pressed('d') {
            let right = Vect::from_slice(&[direction[2], direction[1], direction[0]]);
            position = position + right * self.speed;
        }

        if game_input.keyboard_input.is_key_pressed(Key::Named(NamedKey::Space)) {
            position[1] += self.speed;
        }

        if game_input.keyboard_input.is_key_pressed(Key::Named(NamedKey::Shift)) {
            position[1] -= self.speed;
        }

        self.position = position;
    }

//     match key {
//     Key::Character(key_value) if key_value == smol_str::SmolStr::from("w") => {
//     let x_z_direction = [direction[0], direction[2]];
//     let normalized_direction = normalize(&x_z_direction);
//     position[0] += normalized_direction[0] * 0.1;
//     position[2] += normalized_direction[1] * 0.1;
//     }
//     Key::Character(key_value) if key_value == smol_str::SmolStr::from("s") => {
//     let x_z_direction = [direction[0], direction[2]];
//     let normalized_direction = normalize(&x_z_direction);
//     position[0] -= normalized_direction[0] * 0.1;
//     position[2] -= normalized_direction[1] * 0.1;
//     }
//     Key::Character(key_value) if key_value == smol_str::SmolStr::from("a") => {
//     let x_z_direction = [direction[0], direction[2]];
//     let normalized_direction = normalize(&x_z_direction);
//     position[0] -= normalized_direction[1] * 0.1;
//     position[2] += normalized_direction[0] * 0.1;
//     }
//     Key::Character(key_value) if key_value == smol_str::SmolStr::from("d") => {
//     let x_z_direction = [direction[0], direction[2]];
//     let normalized_direction = normalize(&x_z_direction);
//     position[0] += normalized_direction[1] * 0.1;
//     position[2] -= normalized_direction[0] * 0.1;
//     }
//     Key::Named(NamedKey::Space) => {
//     position[2] += 0.1;
//     }
//     Key::Named(NamedKey::Shift) => {
//     position[1] -= 0.1;
//     }
//     _ => (),
// }
}

fn clamp(value : f32, min : f32, max : f32) -> f32 {
    if value < min {
        return min;
    }

    if value > max {
        return max;
    }

    value
}