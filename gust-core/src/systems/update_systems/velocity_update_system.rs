use gust_hierarchy::world::World;

use crate::components::transform_component::TransformComponent;
use crate::components::velocity_component::VelocityComponent;

pub struct VelocityUpdateSystem;

impl VelocityUpdateSystem {
    pub fn update(world: &mut World, dt: f32) {
        let entities = world.query_mut2::<VelocityComponent, TransformComponent>();

        for (_entity, (velocity, mut transform)) in entities {
            transform.position = transform.position + velocity.velocity * dt;
        }
    }
}