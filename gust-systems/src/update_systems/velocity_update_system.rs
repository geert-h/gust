use gust_components::components::transform_component::TransformComponent;
use gust_components::components::velocity_component::VelocityComponent;
use gust_hierarchy::world::World;

pub struct VelocityUpdateSystem;

impl VelocityUpdateSystem {
    pub fn update(world: &mut World, dt: f32) {
        let entities = world.query_mut2::<VelocityComponent, TransformComponent>();

        for (_entity, (velocity, mut transform)) in entities {
            transform.position = transform.position + velocity.velocity * dt;
        }
    }
}