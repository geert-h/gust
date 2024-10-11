use gust_core::handlers::input_handler::InputHandler;
use gust_hierarchy::world::World;

use crate::physics_systems::collision_detection_system::CollisionDetectionSystem;
use crate::physics_systems::collision_resolution_system::CollisionResolutionSystem;
use crate::physics_systems::force_accumulation_system::ForceAccumulationSystem;
use crate::physics_systems::physics_integration_system::PhysicsIntegrationSystem;

pub mod physics_integration_system;
pub mod collision_detection_system;
pub mod force_accumulation_system;
pub mod collision_resolution_system;

pub struct PhysicsSystem;

impl PhysicsSystem {
    pub fn run(delta_time: f32, world: &mut World, game_input: &InputHandler) {
        ForceAccumulationSystem::run(delta_time, world, game_input);
        PhysicsIntegrationSystem::run(delta_time, world);
        let collision_pairs = CollisionDetectionSystem::run(delta_time, world);
        CollisionResolutionSystem::run(delta_time, world, collision_pairs);
    }
}