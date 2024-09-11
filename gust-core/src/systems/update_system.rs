use gust_hierarchy::world::World;

use crate::components::transform_component::TransformComponent;
use crate::components::velocity_component::VelocityComponent;

pub fn update_system(world: &mut World) {
    let entities = world.query::<(TransformComponent, VelocityComponent)>();
}