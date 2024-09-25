use gust_components::Component::{RigidBodyComponent, TransformComponent, VelocityComponent};
use gust_components::ComponentType::{RigidBodyComponentType, TransformComponentType, VelocityComponentType};
use gust_hierarchy::world::World;

use crate::physics_systems::collision_detection_system::CollisionPair;

pub struct CollisionResolutionSystem;

impl CollisionResolutionSystem {
    pub fn run(delta_time: f32, world: &mut World, collision_pairs: Vec<CollisionPair>) {
        let component_types = vec![TransformComponentType, RigidBodyComponentType, VelocityComponentType];
        for collision in collision_pairs {
            let (entity_a, entity_b) = collision.get_entities();

            let [TransformComponent(ref mut transform_a),
            RigidBodyComponent(ref mut rigid_body_a),
            VelocityComponent(ref mut velocity_a)]
                = world.get_components_mut(entity_a, component_types.clone())
                .unwrap()[..] else { continue };

            let is_static_a = rigid_body_a.is_static;

            let [TransformComponent(ref mut transform_b),
            RigidBodyComponent(ref mut rigid_body_b),
            VelocityComponent(ref mut velocity_b)]
                = world.get_components_mut(entity_b, component_types.clone())
                .unwrap()[..] else { continue };

            if is_static_a && rigid_body_b.is_static {
                continue;
            }
        }
    }
}