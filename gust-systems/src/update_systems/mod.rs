use gust_components::Component;
use gust_components::ComponentType::PlayerComponentType;
use gust_core::entity::Entity;
use gust_core::handlers::input_handler::InputHandler;
use gust_hierarchy::world::World;

mod player_update_system;
mod object_rotation_system;
mod velocity_update_system;

pub struct UpdateSystem;

impl UpdateSystem {
    pub fn update(delta_time: f32, game_input: &InputHandler, world: &mut World) {
        // First update the player
        let player = world.query_one(PlayerComponentType).collect::<Vec<(Entity, &Component)>>().iter().map(|(entity, _component)| *entity).collect::<Vec<Entity>>()[0];

        // Update the player's transform
        player_update_system::PlayerUpdateSystem::update(player, delta_time, world, &game_input);

        // Update the objects
        // object_rotation_system::ObjectRotationSystem::update(world, delta_time);
        velocity_update_system::VelocityUpdateSystem::update(world, delta_time);
    }
}
