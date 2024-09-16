use gust_hierarchy::world::World;
use gust_math::matrices::mat3::Mat3;

use crate::components::mesh_component::MeshComponent;
use crate::components::player_component::PlayerComponent;
use crate::components::transform_component::TransformComponent;

pub struct ObjectRotationSystem;

impl ObjectRotationSystem {
    pub fn update(world: &mut World, dt: f32) {
        let entities: Vec<_> = world.query::<TransformComponent>()
            .into_iter()
            .filter(|(entity, _)| !world.has_component::<PlayerComponent>(*entity) && world.has_component::<MeshComponent>(*entity))
            .map(|(entity, _)| entity)
            .collect();

        for entity in entities {
            let mut transform = world.get_component_mut::<TransformComponent>(entity).unwrap();
            let rotation_matrix = Mat3::rotation_matrix(&transform.forward, 0.1 * dt);
            transform.up = rotation_matrix * transform.up;
        }
    }
}