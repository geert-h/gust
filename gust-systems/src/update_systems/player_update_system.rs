use winit::keyboard::{Key, NamedKey};

use gust_components::components::transform_component::TransformComponent;
use gust_core::entity::Entity;
use gust_core::handlers::input_handler::InputHandler;
use gust_hierarchy::world::World;
use gust_math::matrices::mat3::Mat3;
use gust_math::vectors::vect3::Vect3;

pub struct PlayerUpdateSystem;

impl PlayerUpdateSystem {
    const MAX_VERTICAL_ANGLE: f32 = 180.0f32 * std::f32::consts::PI / 180.0f32;
    const MIN_VERTICAL_ANGLE: f32 = 0.0f32 * std::f32::consts::PI / 180.0f32;

    pub fn update(player_entity: Entity, dt: &f32, world: &mut World, game_input: &InputHandler) {
        let transform = world.get_component_mut::<TransformComponent>(player_entity).unwrap();
        // let mut player_velocity = world.get_component_mut::<VelocityComponent>(player_entity).unwrap();

        PlayerUpdateSystem::update_direction(dt, transform, game_input);
        PlayerUpdateSystem::update_position(dt, transform, game_input);
    }

    fn update_direction(dt: &f32, player_transform: &mut TransformComponent, game_input: &InputHandler) {
        let delta_x = game_input.mouse_input.mouse_delta.0;
        let delta_y = game_input.mouse_input.mouse_delta.1;

        let sensitivity = 0.0005;
        let delta_x = delta_x * sensitivity;
        let delta_y = delta_y * sensitivity;

        let rotation_matrix_side = Mat3::rotation_matrix(&player_transform.up, -delta_x);

        let mut new_direction = player_transform.forward.clone();

        let right = player_transform.forward.cross(&player_transform.up).normalize();

        let vertical_angle = player_transform.forward.dot(&player_transform.up).acos();

        // Clamp the vertical angle
        if vertical_angle - delta_y > Self::MAX_VERTICAL_ANGLE || vertical_angle - delta_y < Self::MIN_VERTICAL_ANGLE {
            new_direction = rotation_matrix_side.clone() * new_direction.normalize();
            player_transform.forward = new_direction;
            return;
        }

        let rotation_matrix_up = Mat3::rotation_matrix(&right, delta_y);

        new_direction = rotation_matrix_up * rotation_matrix_side * new_direction.normalize();

        new_direction.normalize();

        player_transform.forward = (dt.clone() * new_direction).normalize();
    }

    fn update_position(dt: &f32, player_transform: &mut TransformComponent, game_input: &InputHandler) {
        let look_direction = player_transform.forward.clone();
        let up = player_transform.up.clone();

        let horizontal = (look_direction - up * look_direction.dot(&up)).normalize();
        let right = horizontal.cross(&up).normalize();

        let mut cumulative_vector = Vect3::from_slice(&[0.0, 0.0, 0.0]);

        if game_input.keyboard_input.is_character_pressed('w') {
            cumulative_vector = cumulative_vector + horizontal;
        }

        if game_input.keyboard_input.is_character_pressed('s') {
            cumulative_vector = cumulative_vector - horizontal;
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

        let temp_speed = 10.0;

        cumulative_vector.normalize();
        player_transform.position = player_transform.position + cumulative_vector * temp_speed * dt.clone();
    }
}