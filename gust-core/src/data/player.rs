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
            speed: 0.05,
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

        // We calculate the right axis of the camera by taking the cross product of the direction and the up axis
        let right = direction.cross(&Vect::from_slice(&[0.0, 1.0, 0.0])).unwrap();

        let rotation_matrix_up = Matrix::rotation_matrix(&right, delta_y);

        new_direction = rotation_matrix_up * new_direction;

        new_direction = rotation_matrix * new_direction;

        new_direction.normalize();

        self.direction = new_direction;
    }

    fn update_position(&mut self, game_input: &GameInput) {
        let mut direction = self.direction.clone();
        let mut position = self.position.clone();

        let mut x_z_direction = Vect::from_slice(&[direction[0], 0.0, direction[2]]);
        x_z_direction.normalize();

        let mut cumulative_vector = Vect::from_slice(&[0.0, 0.0, 0.0]);

        if game_input.keyboard_input.is_character_pressed('w') {
            cumulative_vector = cumulative_vector + x_z_direction.clone();
        }

        if game_input.keyboard_input.is_character_pressed('s') {
            cumulative_vector = cumulative_vector - x_z_direction.clone();
        }

        if game_input.keyboard_input.is_character_pressed('a') {
            let right = Vect::from_slice(&[x_z_direction[2], x_z_direction[1], -x_z_direction[0]]);
            cumulative_vector = cumulative_vector - right;
        }

        if game_input.keyboard_input.is_character_pressed('d') {
            let right = Vect::from_slice(&[x_z_direction[2], x_z_direction[1], -x_z_direction[0]]);
            cumulative_vector = cumulative_vector + right;
        }

        if game_input.keyboard_input.is_key_pressed(Key::Named(NamedKey::Space)) {
            cumulative_vector[1] += 1.0;
        }

        if game_input.keyboard_input.is_key_pressed(Key::Named(NamedKey::Shift)) {
            cumulative_vector[1] -= 1.0;
        }

        cumulative_vector.normalize();
        position = position + cumulative_vector * self.speed;
        self.position = position;
    }
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