use gust_hierarchy::world::World;

use crate::components::player_component::PlayerComponent;
use crate::handlers::input_handler::InputHandler;

mod player_update_system;
mod object_rotation_system;
mod velocity_update_system;

pub struct UpdateSystem;

impl UpdateSystem {
    pub fn update(delta_time: f32, game_input: &InputHandler, world: &mut World) {
        // First update the player
        let player = world.query_one_entity::<PlayerComponent>();

        if let Some(player) = player {
            // Update the player's transform
            player_update_system::PlayerUpdateSystem::update(player, &delta_time, world, &game_input);
        }

        // Update the objects
        // object_rotation_system::ObjectRotationSystem::update(world, delta_time);
        velocity_update_system::VelocityUpdateSystem::update(world, delta_time);
    }
}
