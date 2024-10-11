use gust_components::Component::{RigidBodyComponent, TransformComponent};
use gust_components::ComponentType::{RigidBodyComponentType, TransformComponentType};
use gust_core::handlers::input_handler::InputHandler;
use gust_hierarchy::world::World;
use gust_math::constants::GRAVITY;

pub struct ForceAccumulationSystem;

impl ForceAccumulationSystem {
    pub fn run(delta_time: f32, world: &mut World, game_input: &InputHandler) {
        let gravity = GRAVITY;

        for (entity, mut components) in world.query_mut(vec![TransformComponentType, RigidBodyComponentType]) {
            let [TransformComponent(ref mut transform_component), RigidBodyComponent(ref mut rigid_body)] = components[..] else { continue };

            if rigid_body.is_static {
                continue;
            }

            rigid_body.forces += gravity * rigid_body.mass;
        }
    }
}