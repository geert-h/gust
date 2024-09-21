use gust_components::Component::{TransformComponent, VelocityComponent};
use gust_components::ComponentType::{TransformComponentType, VelocityComponentType};
use gust_hierarchy::world::World;

pub struct VelocityUpdateSystem;

impl VelocityUpdateSystem {
    pub fn update(world: &mut World, dt: f32) {
        for (_entity, mut components) in world.query_mut(vec![TransformComponentType, VelocityComponentType]) {
            let ([TransformComponent(transform)], [VelocityComponent(velocity)]) = components.split_at_mut(1) else { return; };

            transform.position = transform.position + velocity.velocity * dt;
        }
    }
}