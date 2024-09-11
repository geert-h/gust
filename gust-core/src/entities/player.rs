use winit::keyboard::{Key, NamedKey};

use gust_math::matrices::mat3::Mat3;
use gust_math::vectors::vect3::Vect3;

use crate::entities::viewer::Viewer;
use crate::handlers::input_handler::InputHandler;

#[derive(Copy, Clone)]
pub struct Player {
    pub position: Vect3,
    pub direction: Vect3,
    pub up: Vect3,
    pub speed: f32,
}

impl Player {
    const MAX_VERTICAL_ANGLE: f32 = 180.0f32 * std::f32::consts::PI / 180.0f32;
    const MIN_VERTICAL_ANGLE: f32 = 0.0f32 * std::f32::consts::PI / 180.0f32;

    pub fn new(position: Vect3, direction: Vect3, up: Vect3, speed: f32) -> Self {
        Player {
            position,
            direction,
            up,
            speed,
        }
    }

    pub fn init() -> Self {
        let position = Vect3::new(-5.0, 0.0, 1.0);
        let direction = Vect3::new(1.0, 0.0, 0.0);
        let up = Vect3::new(0.0, 0.0, 1.0);
        let speed = 10.0;

        Player::new(position, direction, up, speed)
    }

    pub fn update(&mut self, dt: &f32, game_input: &InputHandler) {
        self.update_direction(dt, game_input);
        self.update_position(dt, game_input);
    }

    fn update_direction(&mut self, dt: &f32, game_input: &InputHandler) {
        let delta_x = game_input.mouse_input.mouse_delta.0;
        let delta_y = game_input.mouse_input.mouse_delta.1;

        let sensitivity = 0.0005;
        let delta_x = delta_x * sensitivity;
        let delta_y = delta_y * sensitivity;

        let rotation_matrix_side = Mat3::rotation_matrix(&self.up, -delta_x);

        let mut new_direction = self.direction.clone();

        let right = self.direction.cross(&self.up).normalize();

        let vertical_angle = self.direction.dot(&self.up).acos();

        // Clamp the vertical angle
        if vertical_angle - delta_y > Self::MAX_VERTICAL_ANGLE || vertical_angle - delta_y < Self::MIN_VERTICAL_ANGLE {
            new_direction = rotation_matrix_side.clone() * new_direction.normalize();
            self.direction = new_direction;
            return;
        }

        let rotation_matrix_up = Mat3::rotation_matrix(&right, delta_y);

        new_direction = rotation_matrix_up * rotation_matrix_side * new_direction.normalize();

        new_direction.normalize();

        self.direction = dt.clone() * new_direction;
    }

    fn update_position(&mut self, dt: &f32, game_input: &InputHandler) {
        let look_direction = self.direction.clone();
        let up = self.up.clone();

        let horizontal = (look_direction - up * look_direction.dot(&up)).normalize();
        let right = horizontal.cross(&up).normalize();

        let mut cumulative_vector = Vect3::from_slice(&[0.0, 0.0, 0.0]);

        if game_input.keyboard_input.is_character_pressed('w') {
            cumulative_vector = cumulative_vector + horizontal.clone();
        }

        if game_input.keyboard_input.is_character_pressed('s') {
            cumulative_vector = cumulative_vector - horizontal.clone();
        }

        if game_input.keyboard_input.is_character_pressed('a') {
            cumulative_vector = cumulative_vector + right;
        }

        if game_input.keyboard_input.is_character_pressed('d') {
            cumulative_vector = cumulative_vector - right;
        }

        if game_input.keyboard_input.is_key_pressed(Key::Named(NamedKey::Space)) {
            cumulative_vector = cumulative_vector + up;
        }

        if game_input.keyboard_input.is_key_pressed(Key::Named(NamedKey::Shift)) {
            cumulative_vector = cumulative_vector - up;
        }

        cumulative_vector.normalize();
        self.position = self.position + cumulative_vector * self.speed * dt.clone();
    }
}

impl Viewer for Player {
    fn position(&self) -> Vect3 {
        self.position
    }

    fn direction(&self) -> Vect3 {
        self.direction
    }

    fn up(&self) -> Vect3 {
        self.up
    }

    fn fov(&self) -> f32 {
        std::f32::consts::PI / 3.0
    }

    fn z_far(&self) -> f32 {
        1024.0
    }

    fn z_near(&self) -> f32 {
        0.1
    }

    fn aspect_ratio(&self) -> f32 {
        480.0 / 800.0
    }
}